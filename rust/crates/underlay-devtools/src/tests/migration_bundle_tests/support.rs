use std::path::PathBuf;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

pub(crate) fn temp_dir(prefix: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("time should be monotonic")
        .as_nanos();
    path.push(format!("{prefix}_{}_{}", std::process::id(), nanos));
    std::fs::create_dir_all(&path).expect("temp dir should be created");
    path
}

pub(crate) struct DockerRegistryGuard {
    pub(crate) name: String,
}

impl Drop for DockerRegistryGuard {
    fn drop(&mut self) {
        let _ = Command::new("docker")
            .args(["rm", "-f", &self.name])
            .status();
    }
}

pub(crate) fn pick_free_port() -> u16 {
    std::net::TcpListener::bind(("127.0.0.1", 0))
        .expect("bind ephemeral port")
        .local_addr()
        .expect("local addr should exist")
        .port()
}

pub(crate) fn wait_for_registry(port: u16) {
    let url = format!("http://127.0.0.1:{port}/v2/");
    for _ in 0..40 {
        if let Ok(response) = reqwest::blocking::get(&url) {
            if response.status().is_success() {
                return;
            }
        }
        sleep(Duration::from_millis(250));
    }
    panic!("registry did not become ready on port {port}");
}
