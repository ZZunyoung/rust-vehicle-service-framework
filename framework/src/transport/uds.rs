use super::Transport;
use crate::ipc::{bind_listener, read_message, write_message};
use crate::message::Message;
use std::io;
use std::os::unix::net::UnixStream;

pub struct UdsTransport;

impl Transport for UdsTransport {
    fn send(&self, target: &str, message: &Message) -> io::Result<()> {
        let mut stream = UnixStream::connect(target)?;
        write_message(&mut stream, message)
    }

    fn serve(
        &self,
        address: &str,
        mut handler: Box<dyn FnMut(Message) + Send>,
    ) -> io::Result<()> {
        let listener = bind_listener(address)?;

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => match read_message(stream) {
                    Ok(message) => handler(message),
                    Err(error) => eprintln!("UdsTransport failed to read message: {error}"),
                },
                Err(error) => eprintln!("UdsTransport connection failed: {error}"),
            }
        }

        Ok(())
    }
}
