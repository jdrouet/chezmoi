use axum::http::header::{CACHE_CONTROL, CONTENT_TYPE};
use axum::response::{AppendHeaders, IntoResponse};

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

css_style!(style_global_css, chezmoi_ui_static::asset::STYLE_GLOBAL_CSS);
css_style!(
    style_atc_sensor_css,
    chezmoi_ui_static::asset::STYLE_ATC_SENSOR_CSS
);
css_style!(
    style_line_chart_css,
    chezmoi_ui_static::asset::STYLE_LINE_CHART_CSS
);
css_style!(
    style_miflora_sensor_css,
    chezmoi_ui_static::asset::STYLE_MIFLORA_SENSOR_CSS
);
