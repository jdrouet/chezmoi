const DIR: &str = env!("CARGO_TARGET_TMPDIR");

fn write_assets() {
    let original = format!("{}/assets", env!("CARGO_MANIFEST_DIR"));
    let link = format!("{DIR}/assets");
    #[allow(deprecated)]
    if let Err(err) = std::fs::soft_link(&original, &link) {
        eprintln!("something went wrong with link: {err:}");
    }
}

pub fn write(filename: &str, view: String) {
    write_assets();

    std::fs::write(format!("{DIR}/{filename}"), view).unwrap();
}
