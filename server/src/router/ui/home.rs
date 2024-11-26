use axum::response::Html;
use axum::Extension;
use chezmoi_client::view::prelude::View;
use chezmoi_database::metrics::MetricHeader;

use super::error::Error;

pub(super) async fn handle(
    Extension(database): Extension<chezmoi_database::Client>,
) -> Result<Html<String>, Error> {
    let headers =
        vec![MetricHeader::new("host.system.memory.used").with_tag("hostname", "COMP-YPPVQY3KD7")];
    let item = chezmoi_database::metrics::entity::find_latest::Command::new(&headers)
        .execute(database.as_ref())
        .await?;
    println!("item = {item:?}");
    Ok(Html(chezmoi_client::view::home::View::default().render()))
}
