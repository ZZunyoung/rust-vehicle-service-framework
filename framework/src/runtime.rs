use crate::event::Event;
use crate::event_bus::EventBus;
use crate::message::Message;
use crate::registry::ServiceRegistry;

pub struct Runtime {
    registry: ServiceRegistry,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            registry: ServiceRegistry::new(),
        }
    }

    pub fn register_service(&mut self, name: &str, subscriptions: Vec<Event>, socket_path: &str) {
        self.registry.register(name, subscriptions, socket_path);
    }

    pub fn publish(&self, event: Event) {
        let event_bus = EventBus::new(self.registry.clone());

        event_bus.publish(event);
    }

    pub fn handle_message(&mut self, message: Message) {
        match message {
            Message::Register {
                service_name,
                subscriptions,
                socket_path,
            } => {
                self.register_service(&service_name, subscriptions, &socket_path);
                println!("Runtime registered service: {}", service_name);
                self.print_services();
            }
            Message::Publish {
                service_name,
                event,
            } => {
                println!(
                    "Runtime received publish from {}: {}",
                    service_name,
                    event.name()
                );

                self.publish(event);
            }
            Message::Dispatch { event } => {
                println!(
                    "Runtime ignored dispatch message: {}",
                    event.name()
                );
            }
            Message::Heartbeat { service_name } => {
                println!("Runtime received heartbeat from {}", service_name);
            }
            Message::Shutdown { service_name } => {
                println!("Runtime received shutdown from {}", service_name);
            }
        }
    }

    pub fn service_count(&self) -> usize {
        self.registry.service_count()
    }

    pub fn print_services(&self) {
        println!("Registered services:");

        for service in self.registry.services() {
            let subscriptions = service
                .subscriptions
                .iter()
                .map(|event| event.name())
                .collect::<Vec<_>>()
                .join(", ");

            println!(
                "- {} [{}] at {}",
                service.name,
                subscriptions,
                service.socket_path
            );
        }
    }
}