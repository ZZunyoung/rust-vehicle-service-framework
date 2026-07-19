use crate::transport::tcp::TcpTransport;
use crate::transport::uds::UdsTransport;
use crate::transport::Transport;
use std::time::Duration;

pub const DEFAULT_RUNTIME_SOCKET_PATH: &str = "/tmp/vehicle-framework.sock";

pub fn service_socket_path(service_name: &str) -> String {
    format!("/tmp/vehicle-framework-{service_name}.sock")
}

pub enum TransportKind {
    Uds,
    Tcp,
}

pub fn active_transport_kind() -> TransportKind {
    match std::env::var("TRANSPORT").as_deref() {
        Ok("tcp") => TransportKind::Tcp,
        _ => TransportKind::Uds,
    }
}

pub fn new_transport() -> Box<dyn Transport> {
    match active_transport_kind() {
        TransportKind::Uds => Box::new(UdsTransport),
        TransportKind::Tcp => Box::new(TcpTransport),
    }
}

pub fn runtime_address() -> String {
    match active_transport_kind() {
        TransportKind::Uds => DEFAULT_RUNTIME_SOCKET_PATH.to_string(),
        TransportKind::Tcp => "127.0.0.1:9000".to_string(),
    }
}

pub fn service_address(service_name: &str) -> String {
    match active_transport_kind() {
        TransportKind::Uds => service_socket_path(service_name),
        TransportKind::Tcp => format!("127.0.0.1:{}", tcp_port(service_name)),
    }
}

fn tcp_port(service_name: &str) -> u16 {
    match service_name {
        "display" => 9001,
        "logger" => 9002,
        _ => 9010,
    }
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