pub mod uds;
pub mod tcp;

use crate::message::Message;
use std::io;

pub trait Transport: Send + Sync {
    fn send(&self, target: &str, message: &Message) -> io::Result<()>;

    fn serve(&self, address: &str, handler: Box<dyn FnMut(Message) + Send>) -> io::Result<()>;
}
