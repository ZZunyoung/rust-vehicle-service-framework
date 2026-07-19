use crate::event::Event;
use crate::config::{new_transport, runtime_address, service_address};
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
            runtime_socket_path: runtime_address(),
            service_socket_path: service_address(service_name),
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

        new_transport().send(&self.runtime_socket_path, &message)
    }   

    pub fn publish(&self, event: Event) -> io::Result<()> {
        let message = Message::Publish {
            service_name: self.service_name.clone(),
            event,
        };

        new_transport().send(&self.runtime_socket_path, &message)
    }

    pub fn service_socket_path(&self) -> &str {
        &self.service_socket_path
    }

    pub fn heartbeat(&self) -> io::Result<()> {
        let message = Message::Heartbeat {
            service_name: self.service_name.clone(),
        };

        new_transport().send(&self.runtime_socket_path, &message)
    }

    pub fn shutdown(&self) -> io::Result<()> {
        let message = Message::Shutdown {
            service_name: self.service_name.clone(),
        };

        new_transport().send(&self.runtime_socket_path, &message)
    }
}