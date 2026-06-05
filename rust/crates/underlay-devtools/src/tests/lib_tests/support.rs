pub(super) fn temp_dir(prefix: &str) -> std::path::PathBuf {
    let mut path = std::env::temp_dir();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time should be monotonic")
        .as_nanos();
    path.push(format!("{prefix}_{}_{}", std::process::id(), nanos));
    std::fs::create_dir_all(&path).expect("temp dir should be created");
    path
}
