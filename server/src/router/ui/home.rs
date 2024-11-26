use axum::response::Html;
use axum::Extension;
use chezmoi_client::view::prelude::View;
use chezmoi_database::metrics::{MetricHeader, MetricName, MetricTagValue, MetricTags};

use super::error::Error;

pub(super) async fn handle(
    Extension(database): Extension<chezmoi_database::Client>,
) -> Result<Html<String>, Error> {
    let headers = vec![MetricHeader {
        name: MetricName::new("host.system.memory.used"),
        tags: MetricTags::default()
            .with("hostname", MetricTagValue::Text("COMP-YPPVQY3KD7".into())),
    }];
    // let current = now();
    // let before = current - 60 * 60 * 24; // 1h gap
    // let list = chezmoi_database::metrics::aggr::ListAggregation::new(&[], (before, current), 10)
    //     .execute(database.as_ref())
    //     .await?;
    let item = chezmoi_database::metrics::entity::find_latest::Command::new(&headers)
        .execute(database.as_ref())
        .await?;
    println!("item = {item:?}");
    Ok(Html(chezmoi_client::view::home::View::default().render()))
}
