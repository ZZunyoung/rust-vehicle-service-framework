use crate::message::Message;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;

pub const DEFAULT_SOCKET_PATH: &str = "/tmp/vehicle-framework.sock";

pub fn encode_message(message: &Message) -> Result<String, serde_json::Error> {
    serde_json::to_string(message)
}

pub fn decode_message(raw: &str) -> Result<Message, serde_json::Error> {
    serde_json::from_str(raw)
}

pub fn send_message(socket_path: &str, message: &Message) -> io::Result<()> {
    let mut stream = UnixStream::connect(socket_path)?;
    let raw = encode_message(message).map_err(io::Error::other)?;

    stream.write_all(raw.as_bytes())?;
    stream.write_all(b"\n")?;

    Ok(())
}

pub fn bind_listener(socket_path: &str) -> io::Result<UnixListener> {
    if Path::new(socket_path).exists() {
        fs::remove_file(socket_path)?;
    }

    UnixListener::bind(socket_path)
}

pub fn read_message(stream: UnixStream) -> io::Result<Message> {
    let mut reader = BufReader::new(stream);
    let mut raw = String::new();

    reader.read_line(&mut raw)?;

    decode_message(raw.trim_end()).map_err(io::Error::other)
}

pub fn receive_loop<F>(socket_path: &str, mut handler: F) -> io::Result<()>
where
    F: FnMut(Message),
{
    let listener = bind_listener(socket_path)?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => match read_message(stream) {
                Ok(message) => handler(message),
                Err(error) => eprintln!("Failed to read service message: {error}"),
            },
            Err(error) => eprintln!("Service connection failed: {error}"),
        }
    }

    Ok(())
}