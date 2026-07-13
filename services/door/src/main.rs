use framework::event::Event;
use framework::service_api::ServiceContext;

fn main() {
    let context = ServiceContext::new("door");

    match context.publish(Event::DoorOpened) {
        Ok(()) => println!("door published DoorOpened"),
        Err(error) => eprintln!("door failed to publish event: {error}"),
    }
}