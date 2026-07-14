use std::time::Duration;

pub const DEFAULT_RUNTIME_SOCKET_PATH: &str = "/tmp/vehicle-framework.sock";

pub fn service_socket_path(service_name: &str) -> String {
    format!("/tmp/vehicle-framework-{service_name}.sock")
}

pub fn health_check_interval() -> Duration {
    Duration::from_secs(3)
}

pub fn heartbeat_interval() -> Duration {
    Duration::from_secs(5)
}

pub fn heartbeat_timeout() -> Duration {
    Duration::from_secs(10)
}