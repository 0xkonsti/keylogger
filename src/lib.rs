mod key_logger;
mod message;

pub use key_logger::KeyLogger;
pub use message::{Message, MessageBuilder, MessageType, Payload};

pub const HOST: &str = "127.0.0.1";
pub const PORT: u16 = 42069;
