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

impl ServiceRegistry {
    pub fn new() -> Self {
        Self {
            services: Vec::new(),
        }
    }

    pub fn register(&mut self, name: &str, subscriptions: Vec<Event>, socket_path: &str) {
        if let Some(service) = self.services.iter_mut().find(|service| service.name == name) {
            service.subscriptions = subscriptions;
            service.socket_path = socket_path.to_string();
            service.last_heartbeat = Instant::now();
            return;
        }

        let service = ServiceInfo {
            name: name.to_string(),
            subscriptions,
            socket_path: socket_path.to_string(),
            last_heartbeat: Instant::now(),
            status: ServiceStatus::Alive,
        };

        self.services.push(service);
    }

    pub fn subscribers_for(&self, event: &Event) -> Vec<ServiceInfo> {
        self.services
            .iter()
            .filter(|service| service.subscriptions.contains(event))
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
}