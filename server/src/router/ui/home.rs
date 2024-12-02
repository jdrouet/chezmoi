use std::sync::Arc;

use axum::extract::Query;
use axum::response::Html;
use axum::Extension;
use chezmoi_client::view::dashboard::TimePickerDuration;
use chezmoi_client::view::prelude::View;
use chezmoi_database::helper::now;
use chezmoi_database::metrics::entity::find_latest;

use super::error::Error;
use crate::service::dashboard::{BuilderContext, Dashboard};

#[derive(Debug, Default, serde::Deserialize)]
pub enum TimeDuration {
    #[serde(alias = "1h")]
    OneHour,
    #[serde(alias = "1d")]
    OneDay,
    #[default]
    #[serde(alias = "1w")]
    OneWeek,
    #[serde(alias = "2w")]
    TwoWeeks,
}

impl TimeDuration {
    const fn as_secs(&self) -> u64 {
        match self {
            Self::OneHour => 60 * 60,
            Self::OneDay => 60 * 60 * 24,
            Self::OneWeek => 60 * 60 * 24 * 7,
            Self::TwoWeeks => 60 * 60 * 24 * 7 * 2,
        }
    }
}

impl From<TimeDuration> for TimePickerDuration {
    fn from(value: TimeDuration) -> Self {
        match value {
            TimeDuration::OneHour => Self::OneHour,
            TimeDuration::OneDay => Self::OneDay,
            TimeDuration::OneWeek => Self::OneWeek,
            TimeDuration::TwoWeeks => Self::TwoWeeks,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub(crate) enum QueryParams {
    Duration {
        #[serde(default)]
        duration: TimeDuration,
    },
}

impl QueryParams {
    fn duration(&self) -> Option<TimePickerDuration> {
        self.duration.map(|d| d.into())
    }

    fn window(&self) -> (Option<u64>, Option<u64>) {
        match self {
            Self::Duration { duration } => {
                let current = now();
                (Some(current - duration.as_secs()), Some(current))
            }
        }
    }
}

pub(super) async fn handle(
    Extension(dashboard): Extension<Arc<Dashboard>>,
    Extension(database): Extension<chezmoi_database::Client>,
    Query(params): Query<QueryParams>,
) -> Result<Html<String>, Error> {
    let mut ctx = BuilderContext::default();
    ctx.set_window(params.duration());
    let headers = dashboard.collect_latest_metrics();
    let latests = find_latest::Command::new(&headers, params.window(), None)
        .execute(database.as_ref())
        .await?;
    ctx.add_latests(latests.into_iter());

    let page = dashboard.build_view(ctx).await.unwrap();

    Ok(Html(page.render()))
}
