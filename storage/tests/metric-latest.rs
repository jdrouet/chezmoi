use chezmoi_entity::metric::{Metric, MetricHeader};

mod helper;

#[tokio::test]
async fn should_fetch_single_by_name() {
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
    let result =
        chezmoi_storage::metric::latest(client.as_ref(), &[MetricHeader::new("foo")], (0, 10))
            .await
            .unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].timestamp, 9);
    assert_eq!(result[0].value, 4.0);
}

#[tokio::test]
async fn should_fetch_multiple_by_name() {
    let client = helper::create_client().await;
    assert!(client.ping().await.is_ok());
    helper::create_metrics(
        &client,
        [
            Metric::new(1, MetricHeader::new("foo"), 0.0),
            Metric::new(3, MetricHeader::new("foo"), 1.0),
            Metric::new(5, MetricHeader::new("bar"), 2.0),
            Metric::new(7, MetricHeader::new("bar"), 3.0),
            Metric::new(9, MetricHeader::new("baz"), 4.0),
            Metric::new(11, MetricHeader::new("baz"), 5.0), // out of queried window
            Metric::new(5, MetricHeader::new("chat"), 5.0), // should not get be returned
        ]
        .iter(),
    )
    .await;
    let result = chezmoi_storage::metric::latest(
        client.as_ref(),
        &[MetricHeader::new("foo"), MetricHeader::new("bar")],
        (0, 10),
    )
    .await
    .unwrap();
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].timestamp, 7);
    assert_eq!(result[0].value, 3.0);
    assert_eq!(result[1].timestamp, 3);
    assert_eq!(result[1].value, 1.0);
}

#[tokio::test]
async fn should_fetch_single_with_tags() {
    let client = helper::create_client().await;
    assert!(client.ping().await.is_ok());
    helper::create_metrics(
        &client,
        [
            Metric::new(4, MetricHeader::new("foo").with_tag("host", "a"), 0.0),
            Metric::new(5, MetricHeader::new("foo").with_tag("host", "b"), 1.0),
            Metric::new(6, MetricHeader::new("bar").with_tag("host", "a"), 5.0),
        ]
        .iter(),
    )
    .await;
    let result = chezmoi_storage::metric::latest(
        client.as_ref(),
        &[MetricHeader::new("foo").with_tag("host", "a")],
        (0, 10),
    )
    .await
    .unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].timestamp, 4);
    assert_eq!(result[0].value, 0.0);
}

#[tokio::test]
async fn should_fetch_multiple_with_tags() {
    let client = helper::create_client().await;
    assert!(client.ping().await.is_ok());
    helper::create_metrics(
        &client,
        [
            Metric::new(1, MetricHeader::new("foo").with_tag("host", "a"), 0.0),
            Metric::new(2, MetricHeader::new("foo").with_tag("host", "a"), 0.0),
            Metric::new(3, MetricHeader::new("foo").with_tag("host", "b"), 1.0),
            Metric::new(4, MetricHeader::new("bar").with_tag("host", "a"), 2.0),
            Metric::new(5, MetricHeader::new("bar").with_tag("host", "b"), 3.0),
            Metric::new(6, MetricHeader::new("baz").with_tag("host", "a"), 4.0),
            Metric::new(7, MetricHeader::new("baz").with_tag("host", "b"), 5.0),
        ]
        .iter(),
    )
    .await;
    let result = chezmoi_storage::metric::latest(
        client.as_ref(),
        &[
            MetricHeader::new("foo").with_tag("host", "a"),
            MetricHeader::new("bar").with_tag("host", "b"),
        ],
        (0, 10),
    )
    .await
    .unwrap();
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].timestamp, 5);
    assert_eq!(result[0].header.name, "bar");
    assert_eq!(result[0].value, 3.0);
    assert_eq!(result[1].timestamp, 2);
    assert_eq!(result[1].header.name, "foo");
    assert_eq!(result[1].value, 0.0);
}
