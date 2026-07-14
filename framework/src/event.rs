use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Event {
    DoorOpened,
    EngineStarted,
    FuelLow,
    ReverseGear,
    ServiceDown {
        service_name: String,
    },
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
            Event::ServiceDown { .. } => "ServiceDown",
            Event::Heartbeat => "Heartbeat",
            Event::Shutdown => "Shutdown",
        }
    }

    pub fn same_kind(&self, other: &Event) -> bool {
        matches!(
            (self, other),
            (Event::DoorOpened, Event::DoorOpened)
                | (Event::EngineStarted, Event::EngineStarted)
                | (Event::FuelLow, Event::FuelLow)
                | (Event::ReverseGear, Event::ReverseGear)
                | (Event::ServiceDown { .. }, Event::ServiceDown { .. })
                | (Event::Heartbeat, Event::Heartbeat)
                | (Event::Shutdown, Event::Shutdown)
        )
    }
}