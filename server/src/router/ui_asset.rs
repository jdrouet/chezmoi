use axum::http::header::{CACHE_CONTROL, CONTENT_TYPE};
use axum::response::{AppendHeaders, IntoResponse};
use axum::routing::get;

macro_rules! css_style {
    ($name:ident, $content:expr) => {
        pub async fn $name() -> impl IntoResponse {
            (
                AppendHeaders([
                    (CONTENT_TYPE, "text/css"),
                    (CACHE_CONTROL, "max-age=604800"),
                ]),
                $content,
            )
        }
    };
}

css_style!(
    style_global_css,
    chezmoi_web_prelude::asset::STYLE_GLOBAL_CSS
);
css_style!(
    style_atc_sensor_css,
    chezmoi_web_prelude::asset::STYLE_ATC_SENSOR_CSS
);
css_style!(
    style_line_chart_css,
    chezmoi_web_prelude::asset::STYLE_LINE_CHART_CSS
);
css_style!(
    style_miflora_sensor_css,
    chezmoi_web_prelude::asset::STYLE_MIFLORA_SENSOR_CSS
);

pub fn create() -> axum::Router {
    axum::Router::new()
        .route(
            &format!("/{}", chezmoi_web_prelude::asset::STYLE_GLOBAL_CSS_PATH),
            get(style_global_css),
        )
        .route(
            &format!("/{}", chezmoi_web_prelude::asset::STYLE_ATC_SENSOR_CSS_PATH),
            get(style_atc_sensor_css),
        )
        .route(
            &format!("/{}", chezmoi_web_prelude::asset::STYLE_LINE_CHART_CSS_PATH),
            get(style_line_chart_css),
        )
        .route(
            &format!(
                "/{}",
                chezmoi_web_prelude::asset::STYLE_MIFLORA_SENSOR_CSS_PATH
            ),
            get(style_miflora_sensor_css),
        )
}
