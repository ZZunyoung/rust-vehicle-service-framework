use framework::config::{
    heartbeat_timeout,
    health_check_interval,
    DEFAULT_RUNTIME_SOCKET_PATH,
};
use framework::ipc::{bind_listener, read_message};
use framework::runtime::Runtime;
use framework::scheduler::Scheduler;
use std::sync::{Arc, Mutex};


fn main() {
    let runtime = Arc::new(Mutex::new(Runtime::new()));

    println!("Runtime listening on {}", DEFAULT_RUNTIME_SOCKET_PATH);

    let listener = bind_listener(DEFAULT_RUNTIME_SOCKET_PATH)
        .expect("failed to bind runtime socket");

    let runtime_for_health = Arc::clone(&runtime);

    Scheduler::spawn_interval("health-check", health_check_interval(), move || {
        let mut runtime = runtime_for_health
            .lock()
            .expect("runtime mutex poisoned");

        runtime.check_health(heartbeat_timeout());
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