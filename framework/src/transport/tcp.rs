use super::Transport;
use crate::ipc::{read_message, write_message};
use crate::message::Message;
use std::io;
use std::net::{TcpListener, TcpStream};

pub struct TcpTransport;

impl Transport for TcpTransport {
    fn send(&self, target: &str, message: &Message) -> io::Result<()> {
        let mut stream = TcpStream::connect(target)?;
        write_message(&mut stream, message)
    }

    fn serve(
        &self,
        address: &str,
        mut handler: Box<dyn FnMut(Message) + Send>,
    ) -> io::Result<()> {
        let listener = TcpListener::bind(address)?;

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => match read_message(stream) {
                    Ok(message) => handler(message),
                    Err(error) => eprintln!("TcpTransport failed to read message: {error}"),
                },
                Err(error) => eprintln!("TcpTransport connection failed: {error}"),
            }
        }

        Ok(())
    }
}
