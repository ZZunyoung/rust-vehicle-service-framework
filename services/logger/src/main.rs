use framework::event::Event;
use framework::ipc::receive_loop;
use framework::message::Message;
use framework::service_api::ServiceContext;
use std::thread;
use std::time::Duration;

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

    let heartbeat_context = context.clone();

    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(5));

        if let Err(error) = heartbeat_context.heartbeat() {
            eprintln!("logger failed to send heartbeat: {error}");
        }
    });

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