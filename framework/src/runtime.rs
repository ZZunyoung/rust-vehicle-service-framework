use crate::event::Event;
use crate::event_bus::EventBus;
use crate::message::Message;
use crate::registry::{RegisterResult, ServiceRegistry};
use std::time::Duration;
use std::sync::{Arc, RwLock};

pub struct Runtime {
    registry: Arc<RwLock<ServiceRegistry>>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            registry: Arc::new(RwLock::new(ServiceRegistry::new())),
        }
    }

    pub fn register_service(
        &self,
        name: &str,
        subscriptions: Vec<Event>,
        socket_path: &str,
    ) -> RegisterResult {
        self.registry.write().unwrap().register(name, subscriptions, socket_path)
    }

    pub fn publish(&self, event: Event) {
        let subscribers = self.registry.read().unwrap().subscribers_for(&event);
        let event_bus = EventBus::new(subscribers);
        event_bus.publish(event);
    }

    pub fn handle_message(&self, message: Message) {
        match message {
            Message::Register {
                service_name,
                subscriptions,
                socket_path,
            } => {
                let result = self.register_service(&service_name, subscriptions, &socket_path);

                match result {
                    RegisterResult::New => println!("Runtime registered new service: {}", service_name),
                    RegisterResult::Updated => println!("Runtime updated service: {}", service_name),
                    RegisterResult::Recovered => println!("Runtime recovered service: {}", service_name),
                }

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
                self.mark_heartbeat(&service_name);
            }
            Message::Shutdown { service_name } => {
                self.unregister_service(&service_name);
            }
        }
    }

    pub fn service_count(&self) -> usize {
        self.registry.read().unwrap().service_count()
    }

    pub fn print_services(&self) {
        println!("Registered services:");

        let services = self.registry.read().unwrap().services().to_vec();
        
        for service in services {
            let subscriptions = service
                .subscriptions
                .iter()
                .map(|event| event.name())
                .collect::<Vec<_>>()
                .join(", ");

            println!(
                "- {} [{}] at {} ({:?})",
                service.name,
                subscriptions,
                service.socket_path,
                service.status
            );
        }
    }

    pub fn mark_heartbeat(&self, name: &str) {
        if self.registry.write().unwrap().mark_heartbeat(name) {
            println!("Runtime heartbeat updated: {}", name);
        } else {
            println!("Runtime received heartbeat from unknown service: {}", name);
        }
    }

    pub fn check_health(&self, timeout: Duration) {
        let down_services = self.registry.write().unwrap().mark_timed_out_services_down(timeout);

        for service in down_services {
            println!("Runtime detected service down: {}", service.name);

            self.publish(Event::ServiceDown {
                service_name: service.name,
            });

            self.print_services();
        }
    }

    pub fn unregister_service(&self, name: &str) {
        match self.registry.write().unwrap().unregister(name) {
            Some(_) => {
                println!("Runtime unregistered service: {}", name);
                self.print_services();
            }
            None => {
                println!("Runtime received shutdown from unknown service: {}", name);
            }
        }
    }
}