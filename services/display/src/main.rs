use framework::event::Event;
use framework::ipc::receive_loop;
use framework::message::Message;
use framework::service_api::ServiceContext;
use std::thread;
use std::time::Duration;

fn main() {
    let context = ServiceContext::new("display");

    println!("display listening on {}", context.service_socket_path());

    match context.register(vec![Event::DoorOpened]) {
        Ok(()) => println!("display registered for DoorOpened"),
        Err(error) => {
            eprintln!("display failed to register: {error}");
            return;
        }
    }

    let heartbeat_context = context.clone();

    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(5));

        if let Err(error) = heartbeat_context.heartbeat() {
            eprintln!("display failed to send heartbeat: {error}");
        }
    });

    receive_loop(context.service_socket_path(), |message| match message {
        Message::Dispatch { event } => match event {
            Event::DoorOpened => {
                println!("display received DoorOpened -> display on");
            }
            _ => {
                println!("display ignored event: {}", event.name());
            }
        },
        other => {
            println!("display ignored message from {}", other.sender());
        }
    })
    .expect("display failed to receive messages");
}