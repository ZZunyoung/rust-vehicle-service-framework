use crate::event::Event;

#[derive(Debug, Clone)]
pub struct ServiceInfo {
    pub name: String,
    pub subscriptions: Vec<Event>,
    pub socket_path: String,
}

#[derive(Debug, Clone)]
pub struct ServiceRegistry {
    services: Vec<ServiceInfo>,
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
            return;
        }

        let service = ServiceInfo {
            name: name.to_string(),
            subscriptions,
            socket_path: socket_path.to_string(),
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
}