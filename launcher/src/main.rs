use framework::ipc::{bind_listener, read_message, DEFAULT_SOCKET_PATH};
use framework::runtime::Runtime;

fn main() {
    let mut runtime = Runtime::new();

    println!("Registered services: {}", runtime.service_count());
    println!("Runtime listening on {}", DEFAULT_SOCKET_PATH);

    let listener = bind_listener(DEFAULT_SOCKET_PATH)
        .expect("failed to bind runtime socket");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => match read_message(stream) {
                Ok(message) => runtime.handle_message(message),
                Err(error) => eprintln!("Runtime failed to read message: {error}"),
            },
            Err(error) => eprintln!("Runtime connection failed: {error}"),
        }
    }
}