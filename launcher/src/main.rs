use framework::ipc::{bind_listener, read_message, DEFAULT_SOCKET_PATH};
use framework::runtime::Runtime;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let runtime = Arc::new(Mutex::new(Runtime::new()));

    println!("Runtime listening on {}", DEFAULT_SOCKET_PATH);

    let listener = bind_listener(DEFAULT_SOCKET_PATH)
        .expect("failed to bind runtime socket");

    let runtime_for_health = Arc::clone(&runtime);

    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(3));

        let mut runtime = runtime_for_health
            .lock()
            .expect("runtime mutex poisoned");

        runtime.check_health(Duration::from_secs(10));
    });

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => match read_message(stream) {
                Ok(message) => {
                    let mut runtime = runtime
                        .lock()
                        .expect("runtime mutex poisoned");

                    runtime.handle_message(message);
                }
                Err(error) => eprintln!("Runtime failed to read message: {error}"),
            },
            Err(error) => eprintln!("Runtime connection failed: {error}"),
        }
    }
}