const DIR: &str = env!("CARGO_TARGET_TMPDIR");
pub const STYLE_PATH: &str = "style.css";

fn write_assets() {
    let target = format!("{DIR}/{STYLE_PATH}");
    std::fs::write(&target, chezmoi_client::asset::STYLE_CSS_CONTENT).unwrap();
}

pub fn write<V: chezmoi_client::view::prelude::View>(filename: &str, view: V) {
    write_assets();

    std::fs::write(format!("{DIR}/{filename}"), view.render()).unwrap();
}
