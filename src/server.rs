use __core__::{Message, MessageType, HOST, PORT};
use rdev::{EventType, Key};
use tokio::net::{tcp::OwnedReadHalf, TcpListener, TcpStream};

const TRACING_LEVEL: tracing::Level = tracing::Level::DEBUG;

struct Server;

impl Server {
    pub fn new() -> Server {
        use tracing_subscriber::fmt::format::FmtSpan;
        tracing_subscriber::fmt()
            .with_max_level(TRACING_LEVEL)
            .compact()
            .with_span_events(FmtSpan::FULL)
            .init();
        Server
    }

    async fn serve(&self) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("Starting server on {}:{}", HOST, PORT);
        let listener = TcpListener::bind((HOST, PORT)).await?;
        tracing::info!("Server started");

        loop {
            tokio::select! {
                _ = tokio::signal::ctrl_c() => break,
                result = listener.accept() => {
                    let (socket, addr) = result?;
                    tracing::info!("Accepted connection from {}", addr);
                    tokio::spawn(Self::handle_connection(socket));
                }
            }
        }

        tracing::info!("Shutting down server");
        Ok(())
    }

    async fn handle_connection(socket: TcpStream) {
        let socket_addr = socket.peer_addr().unwrap();
        let (reader, _) = socket.into_split();

        let recv_h = tokio::spawn(Self::handle_receive(reader));

        recv_h.await.unwrap();

        tracing::info!("Closed connection from {}", socket_addr);
    }

    async fn handle_receive(mut reader: OwnedReadHalf) {
        loop {
            tokio::select! {
                valid = Message::has_header_start(&mut reader) => {
                    if !valid {
                        continue;
                    }
                    let message = Message::receive(&mut reader).await;
                    match message {
                        Ok(message) => {
                            tracing::info!("Received message: {:?}", message.message_type());

                            match message.message_type() {
                                MessageType::Auth => {},
                                MessageType::Text => {
                                    let payload = message.payload();
                                    //tracing::info!("Text: {:?}", payload.get_data()[0]);
                                    tracing::info!("Text: {:?}", String::from_utf8_lossy(&payload.get_data()[0]));
                                },
                                MessageType::Key => {
                                    let payload = message.payload();
                                    let data = payload.get_data();
                                    let key: Key = bincode::deserialize(&data[0]).unwrap();
                                    let action: EventType = bincode::deserialize(&data[1]).unwrap();
                                    tracing::info!("Key: {:?} {:?}", key, action);
                                },
                                MessageType::Disconnect => {
                                    tracing::info!("Disconnecting");
                                    break;
                                },
                                _ => {},
                            }
                        }
                        Err(e) => {
                            tracing::error!("Error: {}", e);
                            break;
                        }
                    }
                },
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let server = Server::new();
    if let Err(e) = server.serve().await {
        tracing::error!("Error: {:?}", e);
    }
}
