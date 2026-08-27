fn reads_env_in_tests() {
    let _ = std::env::var("DATABASE_URL");
}
