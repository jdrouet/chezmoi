use std::sync::LazyLock;

use axum::response::Html;
use axum::Extension;
use chezmoi_client::view::prelude::View;

use super::error::Error;
use crate::service::dashboard::mi_thermometer::MiThermometerCard;
use crate::service::dashboard::miflora::MifloraCard;
use crate::service::dashboard::system::{SystemCpuCard, SystemMemoryCard, SystemSwapCard};
use crate::service::dashboard::{BuilderContext, Dashboard, Section};

pub(crate) static DASHBOARD: LazyLock<Dashboard> = LazyLock::new(|| {
    Dashboard::default()
        .with_section(
            Section::new("System")
                .with_card(SystemCpuCard::default())
                .with_card(SystemMemoryCard::default())
                .with_card(SystemSwapCard::default()),
        )
        .with_section(
            Section::new("Temperature").with_card(MiThermometerCard::new(
                Some("Living Room"),
                "00:00:00:00:00",
            )),
        )
        .with_section(
            Section::new("Plants")
                .with_card(MifloraCard::new(Some("Orchid"), "00:00:00:00:01"))
                .with_card(MifloraCard::new(Some("Pilea"), "00:00:00:00:02")),
        )
});

pub(super) async fn handle(
    Extension(database): Extension<chezmoi_database::Client>,
) -> Result<Html<String>, Error> {
    let mut ctx = BuilderContext::default();
    let headers = DASHBOARD.collect_latest_metrics();
    let latests = chezmoi_database::metrics::entity::find_latest::Command::new(&headers, None)
        .execute(database.as_ref())
        .await?;
    ctx.add_latests(latests.into_iter());

    let page = DASHBOARD.build_view(ctx).await.unwrap();

    Ok(Html(page.render()))
}
