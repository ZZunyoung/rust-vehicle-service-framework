use framework::event::Event;
use framework::ipc::receive_loop;
use framework::message::Message;
use framework::service_api::ServiceContext;

fn main() {
    let context = ServiceContext::new("logger");

    println!("logger listening on {}", context.service_socket_path());

    match context.register(vec![Event::DoorOpened]) {
        Ok(()) => println!("logger registered for DoorOpened"),
        Err(error) => {
            eprintln!("logger failed to register: {error}");
            return;
        }
    }

    receive_loop(context.service_socket_path(), |message| match message {
        Message::Dispatch { event } => {
            println!("logger recorded event: {}", event.name());
        }
        other => {
            println!("logger ignored message from {}", other.sender());
        }
    })
    .expect("logger failed to receive messages");
}