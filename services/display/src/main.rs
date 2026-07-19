use framework::event::Event;
use framework::message::Message;
use framework::service_api::ServiceContext;
use std::thread;
use framework::config::{heartbeat_interval, new_transport};
use std::io;

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
        thread::sleep(heartbeat_interval());

        if let Err(error) = heartbeat_context.heartbeat() {
            eprintln!("display failed to send heartbeat: {error}");
        }
    });

    let service_socket_path = context.service_socket_path().to_string();

    thread::spawn(move || {
        new_transport().serve(&service_socket_path, Box::new(|message| match message {
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
        }))
        .expect("display failed to receive messages");
    });

    println!("display running. Press Enter to shutdown.");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read shutdown input");

    match context.shutdown() {
        Ok(()) => println!("display shutdown requested"),
        Err(error) => eprintln!("display failed to send shutdown: {error}"),
    }
}