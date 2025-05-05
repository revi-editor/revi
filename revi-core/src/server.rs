use futures::{SinkExt, StreamExt};
use revi_protocol::{Command, Event};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tungstenite::protocol::Message;

pub async fn run_server(addr: &str) -> tokio::io::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    eprintln!("WebSocket server listening on {}", addr);

    loop {
        let (stream, client_addr) = listener.accept().await?;
        eprintln!("New connection from {}", client_addr);

        tokio::spawn(handle_connection(stream, client_addr));
    }
}

async fn handle_connection(stream: tokio::net::TcpStream, addr: SocketAddr) {
    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("WebSocket handshake failed: {}", e);
            return;
        }
    };

    eprintln!("WebSocket connection established: {}", addr);
    let (mut write, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        match msg {
            Ok(message) => {
                if message.is_text() {
                    let text = message.to_text().unwrap();
                    eprintln!("Received from {}: {}", addr, text);

                    let Ok(command) = serde_json::from_str(&text) else {
                        let event = Event::Error {
                            message: format!("Failed to parse command: {}", text),
                        };
                        let json = serde_json::to_string(&event).unwrap();
                        write.send(Message::Text(json.into())).await.unwrap();
                        continue;
                    };
                    match command {
                        Command::InsertText {
                            buffer_id,
                            position,
                            text,
                        } => {
                            // Update buffer, then broadcast a BufferUpdated event
                            // let updated_content = do_insert(buffer_id, position, &text);
                            let event = Event::BufferUpdated {
                                buffer_id,
                                content: format!("{:?}: {}", &position, &text),
                            };
                            let json = serde_json::to_string(&event).unwrap();
                            write.send(Message::Text(json.into())).await.unwrap();
                        }
                        Command::OpenFile { path } => todo!(),
                        Command::DeleteText { buffer_id, range } => todo!(),
                        Command::MoveCursor {
                            buffer_id,
                            direction,
                        } => todo!(),
                        Command::SaveFile { buffer_id } => todo!(),
                        Command::Subscribe { buffer_id } => {
                            let event = Event::BufferUpdated {
                                buffer_id,
                                content: "You have been subscribed".to_string(),
                            };
                            let json = serde_json::to_string(&event).unwrap();
                            write.send(Message::Text(json.into())).await.unwrap();
                        }
                    }

                    // Echo back for now
                    //if let Err(e) = write.send(message).await {
                    //    eprintln!("Send error: {}", e);
                    //    break;
                    //}
                }
            }
            Err(e) => {
                eprintln!("Error receiving message from {}: {}", addr, e);
                break;
            }
        }
    }

    eprintln!("Connection closed: {}", addr);
}
