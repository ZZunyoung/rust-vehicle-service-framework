use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Event {
    DoorOpened,
    EngineStarted,
    FuelLow,
    ReverseGear,
    ServiceDown,
    Heartbeat,
    Shutdown,
}

impl Event {
    pub fn name(&self) -> &'static str {
        match self {
            Event::DoorOpened => "DoorOpened",
            Event::EngineStarted => "EngineStarted",
            Event::FuelLow => "FuelLow",
            Event::ReverseGear => "ReverseGear",
            Event::ServiceDown => "ServiceDown",
            Event::Heartbeat => "Heartbeat",
            Event::Shutdown => "Shutdown",
        }
    }
}