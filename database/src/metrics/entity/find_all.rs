use crate::metrics::entity::Metric;
use crate::metrics::MetricHeader;

/// Right now, we expect tags to match exactly.
pub struct Command<'a> {
    headers: &'a [MetricHeader],
    window: (Option<u64>, Option<u64>),
    limit: Option<usize>,
}

impl<'a> Command<'a> {
    pub fn new(
        headers: &'a [MetricHeader],
        window: (Option<u64>, Option<u64>),
        limit: Option<usize>,
    ) -> Self {
        Self {
            headers,
            window,
            limit,
        }
    }

    pub async fn execute<'c, E: sqlx::Executor<'c, Database = sqlx::Sqlite>>(
        self,
        executor: E,
    ) -> sqlx::Result<Vec<Metric>> {
        let mut qb = sqlx::QueryBuilder::new("select timestamp, name, tags, value");
        qb.push(" from metrics");
        match self.window {
            (Some(min), Some(max)) => {
                qb.push(" where timestamp >= ")
                    .push_bind(min as i64)
                    .push(" and timestamp <= ")
                    .push_bind(max as i64);
            }
            (Some(min), None) => {
                qb.push(" where timestamp > ").push_bind(min as i64);
            }
            (None, Some(max)) => {
                qb.push(" where timestamp < ").push_bind(max as i64);
            }
            _ => {
                qb.push(" where true");
            }
        }
        if !self.headers.is_empty() {
            qb.push(" and (");
            for (index, header) in self.headers.iter().enumerate() {
                if index > 0 {
                    qb.push(" or");
                }
                qb.push(" (")
                    .push("name = ")
                    .push_bind(header.name.as_ref());
                for (name, value) in header.tags.entries() {
                    qb.push(" and")
                        .push(" json_extract(tags, ")
                        .push_bind(format!("$.{name}"))
                        .push(") = ");
                    crate::tag_value_bind!(qb, value);
                }
                qb.push(")");
            }
            qb.push(")");
        }
        qb.push(" order by timestamp asc");
        if let Some(limit) = self.limit {
            qb.push(" limit ").push_bind(limit as i64);
        }
        //
        let query = qb.build_query_as::<'_, Metric>();
        let rows = query.fetch_all(executor).await?;
        Ok(rows)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::metrics::entity::{Metric, MetricValue};
    use crate::metrics::MetricHeader;

    async fn create_metrics(
        db: &crate::Client,
        header: MetricHeader,
        values: impl Iterator<Item = (u64, MetricValue)>,
    ) -> Vec<Metric> {
        let metrics = values
            .map(|(timestamp, value)| Metric {
                timestamp,
                header: header.clone(),
                value,
            })
            .collect::<Vec<_>>();

        crate::metrics::entity::create::Command::new(&metrics)
            .execute(db.as_ref())
            .await
            .unwrap();

        metrics
    }

    #[tokio::test]
    async fn should_find_latest_for_single_header() {
        let db = crate::Client::test().await;

        let not_expected_header = MetricHeader::new("bar").with_tag("hostname", "rambo");
        let _not_expected_events = create_metrics(
            &db,
            not_expected_header,
            (0..10).map(|index| (index, MetricValue::count(index))),
        )
        .await;

        let expected_header = MetricHeader::new("foo").with_tag("hostname", "rambo");
        let expected_events = create_metrics(
            &db,
            expected_header.clone(),
            (0..10).map(|index| (index, MetricValue::count(index))),
        )
        .await;
        assert_eq!(expected_events.len(), 10);

        let found = super::Command::new(&[expected_header], (None, None), Some(10))
            .execute(db.as_ref())
            .await
            .unwrap();

        assert_eq!(found.len(), 1);
        let last = found.last().unwrap();
        assert_eq!(last.timestamp, 9);
        assert_eq!(last.value.as_count().unwrap(), 9);
    }

    #[tokio::test]
    async fn should_find_similar_tags() {
        let db = crate::Client::test().await;

        let expected_header = MetricHeader::new("foo").with_tag("hostname", "rambo");
        let expected_events = create_metrics(
            &db,
            expected_header.clone(),
            (0..10).map(|index| (index, MetricValue::count(index))),
        )
        .await;
        assert_eq!(expected_events.len(), 10);

        let similar_header = MetricHeader::new("foo")
            .with_tag("hostname", "rambo")
            .with_tag("user", "other");
        let _similar_events = create_metrics(
            &db,
            similar_header,
            (0..10).map(|index| (index + 20, MetricValue::count(index + 20))),
        )
        .await;

        let found = super::Command::new(&[expected_header], (None, None), Some(10))
            .execute(db.as_ref())
            .await
            .unwrap();

        assert_eq!(found.len(), 2);
        let last = found.last().unwrap();
        assert_eq!(last.timestamp, 9);
        assert_eq!(last.value.as_count().unwrap(), 9);
        let first = found.first().unwrap();
        assert_eq!(first.timestamp, 29);
        assert_eq!(first.value.as_count().unwrap(), 29);
    }

    #[tokio::test]
    async fn should_return_multiple_events() {
        let db = crate::Client::test().await;

        let first_header = MetricHeader::new("first").with_tag("hostname", "rambo");
        let first_events = create_metrics(
            &db,
            first_header.clone(),
            (0..10).map(|index| (10 + index, MetricValue::count(index))),
        )
        .await;
        assert_eq!(first_events.len(), 10);

        let second_header = MetricHeader::new("second").with_tag("hostname", "rambo");
        let second_events = create_metrics(
            &db,
            second_header.clone(),
            (0..10).map(|index| (20 + index, MetricValue::count(index))),
        )
        .await;
        assert_eq!(second_events.len(), 10);

        let other_header = MetricHeader::new("foo")
            .with_tag("hostname", "rambo")
            .with_tag("user", "other");
        let _other_events = create_metrics(
            &db,
            other_header,
            (0..10).map(|index| (index + 2, MetricValue::count(index))),
        )
        .await;

        let found = super::Command::new(&[first_header, second_header], (None, None), Some(10))
            .execute(db.as_ref())
            .await
            .unwrap();

        assert_eq!(found.len(), 2);
        let found: HashSet<_> = found.into_iter().map(|item| item.timestamp).collect();
        assert!(found.contains(&19));
        assert!(found.contains(&29));
    }

    #[tokio::test]
    async fn should_return_events_in_window() {
        let db = crate::Client::test().await;

        let header = MetricHeader::new("first").with_tag("hostname", "rambo");
        let events = create_metrics(
            &db,
            header.clone(),
            (0..100).map(|index| (index, MetricValue::count(index))),
        )
        .await;
        assert_eq!(events.len(), 100);

        let found = super::Command::new(&[header], (Some(15), Some(20)), Some(10))
            .execute(db.as_ref())
            .await
            .unwrap();

        assert_eq!(found.len(), 1);
        assert_eq!(found[0].timestamp, 20);
    }
}
