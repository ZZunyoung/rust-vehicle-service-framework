use framework::event::Event;
use framework::config::new_transport;
use framework::message::Message;
use framework::service_api::ServiceContext;
use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    let context = ServiceContext::new("logger");

    println!("logger listening on {}", context.service_socket_path());

    match context.register(vec![
        Event::DoorOpened,
        Event::ServiceDown {
            service_name: "*".to_string(),
        },
    ]) {
        Ok(()) => println!("logger registered for DoorOpened, ServiceDown"),
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

    let service_socket_path = context.service_socket_path().to_string();

    thread::spawn(move || {
        new_transport().serve(&service_socket_path, Box::new(|message| match message {
            Message::Dispatch { event } => match event {
                Event::DoorOpened => {
                    println!("logger recorded event: DoorOpened");
                }
                Event::ServiceDown { service_name } => {
                    println!("logger recorded event: ServiceDown({service_name})");
                }
                other => {
                    println!("logger ignored event: {}", other.name());
                }
            },
            other => {
                println!("logger ignored message from {}", other.sender());
            }
        }))
        .expect("logger failed to receive messages");
    })
    ;

    println!("logger running. Press Enter to shutdown.");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read shutdown input");

    match context.shutdown() {
        Ok(()) => println!("logger shutdown requested"),
        Err(error) => eprintln!("logger failed to send shutdown: {error}"),
    }
}
