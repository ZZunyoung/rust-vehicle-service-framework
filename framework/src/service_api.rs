use crate::event::Event;
use crate::config::{service_socket_path, DEFAULT_RUNTIME_SOCKET_PATH};
use crate::ipc::send_message;
use crate::message::Message;
use std::io;

#[derive(Clone)]
pub struct ServiceContext {
    service_name: String,
    runtime_socket_path: String,
    service_socket_path: String,
}

impl ServiceContext {
    pub fn new(service_name: &str) -> Self {
        Self {
            service_name: service_name.to_string(),
            runtime_socket_path: DEFAULT_RUNTIME_SOCKET_PATH.to_string(),
            service_socket_path: service_socket_path(service_name),
        }
    }

    pub fn service_name(&self) -> &str {
        &self.service_name
    }

    pub fn register(&self, subscriptions: Vec<Event>) -> io::Result<()> {
        let message = Message::Register {
            service_name: self.service_name.clone(),
            subscriptions,
            socket_path: self.service_socket_path.clone(),
        };

        send_message(&self.runtime_socket_path, &message)
    }   

    pub fn publish(&self, event: Event) -> io::Result<()> {
        let message = Message::Publish {
            service_name: self.service_name.clone(),
            event,
        };

        send_message(&self.runtime_socket_path, &message)
    }

    pub fn service_socket_path(&self) -> &str {
        &self.service_socket_path
    }

    pub fn heartbeat(&self) -> io::Result<()> {
        let message = Message::Heartbeat {
            service_name: self.service_name.clone(),
        };

        send_message(&self.runtime_socket_path, &message)
    }

    pub fn shutdown(&self) -> io::Result<()> {
        let message = Message::Shutdown {
            service_name: self.service_name.clone(),
        };

        send_message(&self.runtime_socket_path, &message)
    }
}