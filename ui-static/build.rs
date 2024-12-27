use std::io::Write;

fn var_name(fname: &str) -> String {
    let mut res = fname.replace("-", "_").replace(".", "_").to_uppercase();
    res.push_str("_PATH");
    res
}

fn hash_file(fname: &str) -> String {
    use sha2::{Digest, Sha256};

    let mut f = std::fs::OpenOptions::new()
        .read(true)
        .open(format!("assets/{fname}"))
        .unwrap();
    let mut hasher = Sha256::new();
    std::io::copy(&mut f, &mut hasher).unwrap();
    let result = hasher.finalize();
    hex::encode(&result[..8])
}

fn hash_name(fname: &str) -> String {
    if let Some((before, after)) = fname.split_once(".") {
        format!("assets/{before}-{}.{after}", hash_file(fname))
    } else {
        fname.to_string()
    }
}

fn main() {
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("src/asset/path.rs")
        .unwrap();
    for fname in std::fs::read_dir("assets")
        .unwrap()
        .filter_map(|f| f.ok())
        .filter_map(|f| f.file_name().into_string().ok())
        .filter(|f| f.ends_with(".css"))
    {
        println!("cargo::rerun-if-changed=assets/{}", fname);
        writeln!(
            &mut f,
            "pub const {}: &str = {:?};",
            var_name(&fname),
            hash_name(&fname)
        )
        .unwrap();
    }
}
