use crate::event::Event;
use std::time::Instant;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ServiceInfo {
    pub name: String,
    pub subscriptions: Vec<Event>,
    pub socket_path: String,
    pub last_heartbeat: Instant,
    pub status: ServiceStatus,
}

#[derive(Debug, Clone)]
pub struct ServiceRegistry {
    services: Vec<ServiceInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceStatus {
    Alive,
    Down,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegisterResult {
    New,
    Updated,
    Recovered,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
        }
    }

    pub fn register(
        &mut self,
        name: &str,
        subscriptions: Vec<Event>,
        socket_path: &str,
    ) -> RegisterResult {
        if let Some(service) = self.services.iter_mut().find(|service| service.name == name) {
            let was_down = service.status == ServiceStatus::Down;

            service.subscriptions = subscriptions;
            service.socket_path = socket_path.to_string();
            service.last_heartbeat = Instant::now();
            service.status = ServiceStatus::Alive;

            return if was_down {
                RegisterResult::Recovered
            } else {
                RegisterResult::Updated
            };
        }

        let service = ServiceInfo {
            name: name.to_string(),
            subscriptions,
            socket_path: socket_path.to_string(),
            last_heartbeat: Instant::now(),
            status: ServiceStatus::Alive,
        };

        self.services.push(service);

        RegisterResult::New
    }

    pub fn subscribers_for(&self, event: &Event) -> Vec<ServiceInfo> {
        self.services
            .iter()
            .filter(|service| {
                service.status == ServiceStatus::Alive
                    && service
                        .subscriptions
                        .iter()
                        .any(|subscription| subscription.same_kind(event))
            })
            .cloned()
            .collect()
    }

    pub fn services(&self) -> &[ServiceInfo] {
        &self.services
    }

    pub fn service_count(&self) -> usize {
        self.services.len()
    }

    pub fn mark_heartbeat(&mut self, name: &str) -> bool {
        if let Some(service) = self.services.iter_mut().find(|service| service.name == name) {
            service.last_heartbeat = Instant::now();
            service.status = ServiceStatus::Alive;
            return true;
        }

        false
    }

    pub fn mark_timed_out_services_down(&mut self, timeout: Duration) -> Vec<ServiceInfo> {
        let mut timed_out = Vec::new();

        for service in &mut self.services {
            if service.status == ServiceStatus::Alive && service.last_heartbeat.elapsed() > timeout {
                service.status = ServiceStatus::Down;
                timed_out.push(service.clone());
            }
        }

        timed_out
    }

    pub fn unregister(&mut self, name: &str) -> Option<ServiceInfo> {
        let index = self.services.iter().position(|service| service.name == name)?;

        Some(self.services.remove(index))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_returns_new_then_updated() {
        let mut registry = ServiceRegistry::new();

        let result1 = registry.register(
            "door",
            vec![Event::DoorOpened],
            "/tmp/door.sock",
        );
        assert_eq!(result1, RegisterResult::New);

        let result2 = registry.register(
            "door",
            vec![Event::DoorOpened, Event::EngineStarted],
            "/tmp/door.sock",
        );
        assert_eq!(result2, RegisterResult::Updated);
    }

    #[test]
    fn register_returns_recovered_after_down() {
        let mut registry = ServiceRegistry::new();

        let _result1 = registry.register(
            "door",
            vec![Event::DoorOpened],
            "/tmp/door.sock",
        );

        // Mark the service as down
        registry.mark_timed_out_services_down(Duration::from_secs(0));

        let result2 = registry.register(
            "door",
            vec![Event::DoorOpened, Event::EngineStarted],
            "/tmp/door.sock",
        );
        assert_eq!(result2, RegisterResult::Recovered);
    }

    #[test]
    fn subscribers_for_returns_correct_services() {
        let mut registry = ServiceRegistry::new();

        registry.register(
            "display",
            vec![Event::DoorOpened],
            "/tmp/display.sock",
        );

        registry.register(
            "door",
            vec![],
            "/tmp/door.sock",
        );

        let sub = registry.subscribers_for(&Event::DoorOpened);
        assert_eq!(sub.len(), 1);
        assert_eq!(sub[0].name, "display");
    }

    #[test]
    fn subscribers_for_returns_empty_for_down_services(){
        let mut registry = ServiceRegistry::new();

        registry.register(
            "display",
            vec![Event::DoorOpened],
            "/tmp/display.sock",
        );

        registry.mark_timed_out_services_down(Duration::from_secs(0));

        let sub = registry.subscribers_for(&Event::DoorOpened);
        assert_eq!(sub.len(), 0);
    }

    #[test]
    fn mark_timed_out_services_down_marks_correct_services_as_down() {
        let mut registry = ServiceRegistry::new();

        registry.register(
            "display",
            vec![Event::DoorOpened],
            "/tmp/display.sock",
        );

        let timed_out = registry.mark_timed_out_services_down(Duration::from_secs(0));
        assert_eq!(timed_out.len(), 1);

        let timed_out = registry.mark_timed_out_services_down(Duration::from_secs(0));
        assert_eq!(timed_out.len(), 0);
    }
}