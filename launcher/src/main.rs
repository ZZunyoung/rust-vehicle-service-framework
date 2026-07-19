use framework::config::{
    heartbeat_timeout,
    health_check_interval,
    new_transport,
    runtime_address,
};
use framework::runtime::Runtime;
use framework::scheduler::Scheduler;
use framework::thread_pool::ThreadPool;
use std::sync::Arc;

fn main() {
    let runtime = Arc::new(Runtime::new());

    let address = runtime_address();
    println!("Runtime listening on {}", address);

    let runtime_for_health = Arc::clone(&runtime);

    Scheduler::spawn_interval("health-check", health_check_interval(), move || {
        runtime_for_health.check_health(heartbeat_timeout());
    });

    let pool = ThreadPool::new(4);

    new_transport()
        .serve(
            &address,
            Box::new(move |message| {
                let runtime = Arc::clone(&runtime);
                pool.execute(move || {
                    runtime.handle_message(message);
                });
            }),
        )
        .expect("runtime failed to serve");
}
