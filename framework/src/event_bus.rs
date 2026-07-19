use crate::event::Event;
use crate::ipc::send_message;
use crate::message::Message;
use crate::registry::ServiceInfo;

pub struct EventBus {
    subscribers: Vec<ServiceInfo>,
}

impl EventBus {
    pub fn new(subscribers: Vec<ServiceInfo>) -> Self {
        Self { subscribers }
    }

    pub fn publish(&self, event: Event) {
        println!("EventBus received event: {}", event.name());

        for subscriber in &self.subscribers {
            let message = Message::Dispatch {
                event: event.clone(),
            };

            match send_message(&subscriber.socket_path, &message) {
                Ok(()) => {
                    println!(
                        "EventBus dispatch: {} -> {}",
                        event.name(),
                        subscriber.name
                    );
                }
                Err(error) => {
                    eprintln!(
                        "EventBus failed to dispatch {} -> {}: {}",
                        event.name(),
                        subscriber.name,
                        error
                    );
                }
            }
        }
    }
}