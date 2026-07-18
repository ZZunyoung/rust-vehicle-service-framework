use framework::ipc::{bind_listener, read_message, send_message};
use framework::message::Message;
use std::thread;

#[test]
fn send_and_receive_over_uds() {
    let socket_path = std::env::temp_dir()
        .join(format!("vehicle-test-{}.sock", std::process::id()))
        .to_str()
        .unwrap()
        .to_string();

    let listener = bind_listener(&socket_path).expect("failed to bind test socket");

    let receiver = thread::spawn(move || {
        let stream = listener
            .incoming()
            .next()
            .expect("no incoming connection")
            .expect("connection failed");

        read_message(stream).expect("failed to read message")
    });

    let sent = Message::Heartbeat {
        service_name: "door".to_string(),
    };
    send_message(&socket_path, &sent).expect("failed to send message");

    let received = receiver.join().expect("receiver thread panicked");
    assert_eq!(received, sent);
}
