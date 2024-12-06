use super::MetricAggr;
use crate::metrics::MetricHeader;

pub struct Command<'a> {
    headers: &'a [MetricHeader],
    timerange: (u64, u64),
    divisions: usize,
}

impl<'a> Command<'a> {
    pub fn new(headers: &'a [MetricHeader], timerange: (u64, u64), divisions: usize) -> Self {
        Self {
            headers,
            timerange,
            divisions,
        }
    }

    fn build_subset_headers_filter<'b>(
        &self,
        qb: &'b mut sqlx::QueryBuilder<'b, sqlx::Sqlite>,
    ) -> &'b mut sqlx::QueryBuilder<'b, sqlx::Sqlite>
    where
        'a: 'b,
    {
        if self.headers.is_empty() {
            qb
        } else {
            qb.push(" and (");
            for (index, header) in self.headers.iter().enumerate() {
                if index > 0 {
                    qb.push(" or");
                }
                qb.push(" (name = ").push_bind(header.name.as_ref());
                for (name, value) in header.tags.entries() {
                    qb.push(" and json_extract(tags, ")
                        .push_bind(format!("$.{name}"))
                        .push(") = ");
                    crate::tag_value_bind!(qb, value);
                }
                qb.push(")");
            }
            qb.push(")")
        }
    }

    fn build_subset<'b>(
        &self,
        qb: &'b mut sqlx::QueryBuilder<'b, sqlx::Sqlite>,
    ) -> &'b mut sqlx::QueryBuilder<'b, sqlx::Sqlite>
    where
        'a: 'b,
    {
        let (from_ts, to_ts) = self.timerange;
        qb.push("select");
        qb.push(" timestamp,");
        qb.push(" (timestamp - ")
            .push_bind(from_ts as i64)
            .push(") * ")
            .push_bind(self.divisions as i64)
            .push(" / (")
            .push_bind(to_ts as i64)
            .push(" - ")
            .push_bind(from_ts as i64)
            .push(")")
            .push(" as division,");
        qb.push(" name,");
        qb.push(" tags,");
        qb.push(" value");
        qb.push(" from metrics");
        qb.push(" where timestamp > ").push(self.timerange.0);
        qb.push(" and timestamp <= ").push(self.timerange.1);
        self.build_subset_headers_filter(qb)
    }

    fn build_count_subset<'b>(
        &self,
        qb: &'b mut sqlx::QueryBuilder<'b, sqlx::Sqlite>,
    ) -> &'b mut sqlx::QueryBuilder<'b, sqlx::Sqlite>
    where
        'a: 'b,
    {
        qb.push(" select");
        qb.push(" json_object('from', min(timestamp), 'to', max(timestamp), 'count', count(timestamp)) as timestamp,");
        qb.push(" name, tags,");
        qb
            .push(" json_object('type', 'count', 'min', min(json_extract(value, '$.value')), 'avg', avg(json_extract(value, '$.value')), 'max', max(json_extract(value, '$.value')), 'sum', sum(json_extract(value, '$.value'))) as value");
        qb.push(" from metrics_subset");
        qb.push(" where json_extract(value, '$.type') = 'count'");
        qb.push(" group by division, name, tags")
    }

    fn build_gauge_subset<'b>(
        &self,
        qb: &'b mut sqlx::QueryBuilder<'b, sqlx::Sqlite>,
    ) -> &'b mut sqlx::QueryBuilder<'b, sqlx::Sqlite>
    where
        'a: 'b,
    {
        qb.push(" select");
        qb.push(" json_object('from', min(timestamp), 'to', max(timestamp), 'count', count(timestamp)) as timestamp,");
        qb.push(" name, tags,");
        qb
            .push(" json_object('type', 'gauge', 'min', min(json_extract(value, '$.value')), 'avg', avg(json_extract(value, '$.value')), 'max', max(json_extract(value, '$.value'))) as value");
        qb.push(" from metrics_subset");
        qb.push(" where json_extract(value, '$.type') = 'gauge'");
        qb.push(" group by division, name, tags")
    }

    pub async fn execute<'c, E: sqlx::Executor<'c, Database = sqlx::Sqlite>>(
        self,
        executor: E,
    ) -> sqlx::Result<Vec<MetricAggr>> {
        // metrics_subset
        let mut qb = sqlx::QueryBuilder::new("with metrics_subset as (");
        let qb = self.build_subset(&mut qb);
        qb.push(")");
        // count metrics
        qb.push(", metrics_count_subset as (");
        let qb = self.build_count_subset(qb);
        qb.push(")");
        // gauge metrics
        qb.push(", metrics_gauge_subset as (");
        let qb = self.build_gauge_subset(qb);
        qb.push(")");
        // main query
        qb.push(" select * from metrics_count_subset");
        qb.push(" union select * from metrics_gauge_subset");
        //
        let query = qb.build_query_as::<'_, MetricAggr>();
        let rows = query.fetch_all(executor).await?;
        Ok(rows)
    }
}

#[cfg(test)]
mod tests {
    use crate::helper::now;
    use crate::metrics::entity::MetricValue;
    use crate::metrics::MetricHeader;

    const NOW: u64 = 633009600;
    const ONE_MINUTE: u64 = 60 * 60;
    const ONE_HOUR: u64 = 60 * 60;
    const ONE_DAY: u64 = ONE_HOUR * 24;
    const ONE_WEEK: u64 = ONE_DAY * 7;
    const ONE_WEEK_AGO: u64 = NOW - ONE_WEEK;

    struct TimestampGenerator {
        from: u64,
        to: u64,
        interval: u64,
    }

    impl TimestampGenerator {
        fn new(from: u64, to: u64, interval: u64) -> Self {
            Self { from, to, interval }
        }
    }

    impl Iterator for TimestampGenerator {
        type Item = u64;

        fn next(&mut self) -> Option<Self::Item> {
            if self.from <= self.to {
                let now = self.from;
                self.from += self.interval;
                Some(now)
            } else {
                None
            }
        }
    }

    #[tokio::test]
    async fn should_build_query() {
        let current = now();
        let before = current - 60 * 60; // 1h gap
        let headers = vec![MetricHeader::new("hello.world").with_tag("host", "whatever")];

        let db = crate::Client::test().await;

        let list = super::Command::new(&headers, (before, current), 10)
            .execute(db.as_ref())
            .await
            .unwrap();
        assert_eq!(list.len(), 0);
    }

    #[tokio::test]
    async fn should_aggregare_regular_values() {
        let db = crate::Client::test().await;

        let header = MetricHeader::new("foo").with_tag("host", "rpi");
        let generated = crate::helper::create_metrics(
            &db,
            header.clone(),
            TimestampGenerator::new(ONE_WEEK_AGO - ONE_DAY, NOW + ONE_DAY, ONE_MINUTE)
                .enumerate()
                .map(|(index, ts)| (ts, MetricValue::gauge(index as f64))),
        )
        .await;
        assert_eq!(generated.len(), 217);

        let list = super::Command::new(&[header], (ONE_WEEK_AGO + 1, NOW + 1), 7)
            .execute(db.as_ref())
            .await
            .unwrap();
        assert_eq!(list.len(), 7);
        for item in list {
            assert_eq!(item.timerange.count, 24);
        }
    }

    #[tokio::test]
    async fn should_handle_empty_spots() {
        let db = crate::Client::test().await;

        let header = MetricHeader::new("foo").with_tag("host", "rpi");
        let generated = crate::helper::create_metrics(
            &db,
            header.clone(),
            TimestampGenerator::new(NOW - 2 * ONE_DAY, NOW + ONE_DAY, ONE_MINUTE)
                .enumerate()
                .map(|(index, ts)| (ts, MetricValue::gauge(index as f64))),
        )
        .await;
        assert_eq!(generated.len(), 73);

        let list = super::Command::new(&[header], (ONE_WEEK_AGO, NOW), 7)
            .execute(db.as_ref())
            .await
            .unwrap();
        assert_eq!(list.len(), 3);
        assert_eq!(list[0].timerange.count, 24);
        assert_eq!(list[1].timerange.count, 24);
        assert_eq!(list[2].timerange.count, 1);
    }
}
