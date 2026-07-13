use crate::event::Event;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Message {
    Register {
        service_name: String,
        subscriptions: Vec<Event>,
        socket_path: String,
    },
    Publish {
        service_name: String,
        event: Event,
    },
    Dispatch {
        event: Event,
    },
    Heartbeat {
        service_name: String,
    },
    Shutdown {
        service_name: String,
    },
}

impl Message {
    pub fn sender(&self) -> &str {
        match self {
            Message::Register { service_name, .. } => service_name,
            Message::Publish { service_name, .. } => service_name,
            Message::Dispatch { .. } => "runtime",
            Message::Heartbeat { service_name } => service_name,
            Message::Shutdown { service_name } => service_name,
        }
    }
}