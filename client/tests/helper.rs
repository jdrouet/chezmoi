const DIR: &str = env!("CARGO_TARGET_TMPDIR");

fn write_assets() {
    let original = format!("{}/assets", env!("CARGO_MANIFEST_DIR"));
    let link = format!("{DIR}/assets");
    #[allow(deprecated)]
    if let Err(err) = std::fs::soft_link(original, link) {
        println!("something went wrong with link: {err:}");
    }
}

pub fn write<V: chezmoi_client::view::prelude::View>(filename: &str, view: V) {
    write_assets();

    std::fs::write(format!("{DIR}/{filename}"), view.render()).unwrap();
}
