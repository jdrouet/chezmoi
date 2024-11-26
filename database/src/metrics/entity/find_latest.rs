use sqlx::types::Json;

use crate::metrics::entity::Metric;
use crate::metrics::MetricHeader;

/// Right now, we expect tags to match exactly.
pub struct Command<'a> {
    headers: &'a [MetricHeader],
}

impl<'a> Command<'a> {
    pub fn new(headers: &'a [MetricHeader]) -> Self {
        Self { headers }
    }

    pub async fn execute<'c, E: sqlx::Executor<'c, Database = sqlx::Sqlite>>(
        self,
        executor: E,
    ) -> sqlx::Result<Vec<Metric>> {
        // metrics_subset
        let mut qb = sqlx::QueryBuilder::new("with metrics_subset as (");
        qb.push("select timestamp, name, tags, value,");
        qb.push(" row_number() over (partition by name, tags order by timestamp desc) as idx");
        qb.push(" from metrics");
        qb.push(")");
        qb.push(" select timestamp, name, tags, value");
        qb.push(" from metrics_subset");
        qb.push(" where idx = 1");
        if !self.headers.is_empty() {
            qb.push(" and (");
            for (index, header) in self.headers.iter().enumerate() {
                if index > 0 {
                    qb.push(" or");
                }
                qb.push(" (")
                    .push("name = ")
                    .push_bind(header.name.as_ref())
                    .push(" and ")
                    .push(" tags = ")
                    .push_bind(Json(&header.tags))
                    .push(")");
            }
            qb.push(")");
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

    async fn init_db() -> crate::Client {
        let db = crate::config::Config::memory().build().await.unwrap();
        db.upgrade().await.unwrap();
        db
    }

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
        let db = init_db().await;

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

        let found = super::Command::new(&[expected_header])
            .execute(db.as_ref())
            .await
            .unwrap();

        assert_eq!(found.len(), 1);
        let last = found.last().unwrap();
        assert_eq!(last.timestamp, 9);
        assert_eq!(last.value.as_count().unwrap(), 9);
    }

    #[tokio::test]
    async fn should_find_exact_tags() {
        let db = init_db().await;

        let expected_header = MetricHeader::new("foo").with_tag("hostname", "rambo");
        let expected_events = create_metrics(
            &db,
            expected_header.clone(),
            (0..10).map(|index| (index, MetricValue::count(index))),
        )
        .await;
        assert_eq!(expected_events.len(), 10);

        let not_expected_header = MetricHeader::new("foo")
            .with_tag("hostname", "rambo")
            .with_tag("user", "other");
        let _not_expected_events = create_metrics(
            &db,
            not_expected_header,
            (0..10).map(|index| (index + 2, MetricValue::count(index))),
        )
        .await;

        let found = super::Command::new(&[expected_header])
            .execute(db.as_ref())
            .await
            .unwrap();

        assert_eq!(found.len(), 1);
        let last = found.last().unwrap();
        assert_eq!(last.timestamp, 9);
        assert_eq!(last.value.as_count().unwrap(), 9);
    }

    #[tokio::test]
    async fn should_return_multiple_events() {
        let db = init_db().await;

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

        let found = super::Command::new(&[first_header, second_header])
            .execute(db.as_ref())
            .await
            .unwrap();

        assert_eq!(found.len(), 2);
        let found: HashSet<_> = found.into_iter().map(|item| item.timestamp).collect();
        assert!(found.contains(&19));
        assert!(found.contains(&29));
    }
}
