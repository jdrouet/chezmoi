use std::path::PathBuf;

use chezmoi_ui_static::asset::*;

const DIR: &str = env!("CARGO_TARGET_TMPDIR");

const ASSETS: [(&str, &str); 4] = [
    ("style-atc-sensor.css", STYLE_ATC_SENSOR_CSS_PATH),
    ("style-global.css", STYLE_GLOBAL_CSS_PATH),
    ("style-line-chart.css", STYLE_LINE_CHART_CSS_PATH),
    ("style-miflora-sensor.css", STYLE_MIFLORA_SENSOR_CSS_PATH),
];

fn write_assets() {
    let original = format!("{}/assets", env!("CARGO_MANIFEST_DIR"));
    let link = PathBuf::from(format!("{DIR}/assets"));
    if !link.exists() {
        std::fs::create_dir(&link).unwrap();
    }

    for (src, dest) in ASSETS {
        std::fs::copy(format!("{original}/{src}"), format!("{DIR}/{dest}")).unwrap();
    }
}

pub fn write(filename: &str, view: String) {
    write_assets();

    std::fs::write(format!("{DIR}/{filename}"), view).unwrap();
}
