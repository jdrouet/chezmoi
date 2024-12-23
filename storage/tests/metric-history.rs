use chezmoi_entity::metric::{Metric, MetricHeader};

mod helper;

#[tokio::test]
async fn should_fetch_history() {
    let client = helper::create_client().await;
    assert!(client.ping().await.is_ok());
    helper::create_metrics(
        &client,
        [
            Metric::new(1, MetricHeader::new("foo"), 0.0),
            Metric::new(3, MetricHeader::new("foo"), 1.0),
            Metric::new(5, MetricHeader::new("foo"), 2.0),
            Metric::new(7, MetricHeader::new("foo"), 3.0),
            Metric::new(9, MetricHeader::new("foo"), 4.0),
            Metric::new(11, MetricHeader::new("foo"), 5.0), // out of queried window
            Metric::new(5, MetricHeader::new("bar"), 5.0),  // should not get be returned
        ]
        .iter(),
    )
    .await;
    let result = chezmoi_storage::metric::history(
        client.as_ref(),
        [MetricHeader::new("foo")].iter(),
        3,
        (0, 10),
    )
    .await
    .unwrap();
    assert_eq!(result.len(), 3);
    assert_eq!(result[0].timestamp, 2);
    assert_eq!(result[0].value, 0.5);
    assert_eq!(result[1].timestamp, 5);
    assert_eq!(result[1].value, 2.0);
    assert_eq!(result[2].timestamp, 8);
    assert_eq!(result[2].value, 3.5);
}
