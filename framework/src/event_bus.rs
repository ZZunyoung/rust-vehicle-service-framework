use crate::event::Event;
use crate::ipc::send_message;
use crate::message::Message;
use crate::registry::ServiceRegistry;

pub struct EventBus {
    registry: ServiceRegistry,
}

impl EventBus {
    pub fn new(registry: ServiceRegistry) -> Self {
        Self { registry }
    }

    pub fn publish(&self, event: Event) {
        println!("EventBus received event: {}", event.name());

        let subscribers = self.registry.subscribers_for(&event);

        for subscriber in subscribers {
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