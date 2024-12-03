use std::sync::Arc;

use axum::extract::Query;
use axum::response::Html;
use axum::Extension;
use chezmoi_client::view::prelude::View;
use chezmoi_database::helper::now;
use chezmoi_database::metrics::entity::find_latest;

use super::error::Error;
use crate::service::dashboard::{BuilderContext, Dashboard};

#[derive(Debug, serde::Deserialize)]
pub enum TimeDuration {
    #[serde(alias = "1d")]
    OneDay,
    #[serde(alias = "1w")]
    OneWeek,
    #[serde(alias = "2w")]
    TwoWeeks,
}

impl TimeDuration {
    const fn as_secs(&self) -> u64 {
        match self {
            Self::OneDay => 60 * 60 * 24,
            Self::OneWeek => 60 * 60 * 24 * 7,
            Self::TwoWeeks => 60 * 60 * 24 * 7 * 2,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub(crate) enum QueryParams {
    Duration {
        duration: TimeDuration,
    },
    Window {
        // minimum
        #[serde(default)]
        after: Option<u64>,
        // maximum
        #[serde(default)]
        before: Option<u64>,
    },
}

impl QueryParams {
    fn window(&self) -> (Option<u64>, Option<u64>) {
        match self {
            Self::Duration { duration } => {
                let current = now();
                (Some(current - duration.as_secs()), Some(current))
            }
            Self::Window { after, before } => (*after, *before),
        }
    }
}

pub(super) async fn handle(
    Extension(dashboard): Extension<Arc<Dashboard>>,
    Extension(database): Extension<chezmoi_database::Client>,
    Query(params): Query<QueryParams>,
) -> Result<Html<String>, Error> {
    let mut ctx = BuilderContext::default();
    let headers = dashboard.collect_latest_metrics();
    let latests = find_latest::Command::new(&headers, params.window(), None)
        .execute(database.as_ref())
        .await?;
    ctx.add_latests(latests.into_iter());

    let page = dashboard.build_view(ctx).await.unwrap();

    Ok(Html(page.render()))
}
