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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_returns_correct_string() {
        assert_eq!(Event::DoorOpened.name(), "DoorOpened");
        assert_eq!(Event::FuelLow.name(), "FuelLow");
    }
    #[test]
    fn same_kind_returns_true_for_same_event() {
        let event1 = Event::DoorOpened;
        let event2 = Event::DoorOpened;
        let event3 = Event::ServiceDown { service_name: "a".to_string() };

        assert!(event1.same_kind(&event2));
        assert!(event3.same_kind(&Event::ServiceDown { service_name: "b".to_string() }));
        assert!(!event1.same_kind(&event3));
    }
}