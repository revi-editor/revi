pub mod app;

use crossterm::event::{Event as CEvent, KeyCode};
use futures_util::{SinkExt, StreamExt};
use revi_protocol::{Command, Event};
use std::time::Duration;
use tokio::{sync::mpsc, time};
use tokio_tungstenite::connect_async;
use tungstenite::protocol::Message;

#[derive(Debug)]
enum AppEvent {
    BufferUpdated(String),
    Tick,
}

pub async fn start_ui() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, mut rx) = mpsc::channel(100);

    // Connect to backend
    let (ws_stream, _) = connect_async("ws://127.0.0.1:9001").await?;
    let (mut write, mut read) = ws_stream.split();

    write
        .send(Message::Text(
            r#"{"type":"Subscribe","buffer_id":1}"#.into(),
        ))
        .await?;

    let tx_clone = tx.clone();
    tokio::spawn(async move {
        while let Some(Ok(msg)) = read.next().await {
            eprintln!("Client Received: '{msg}'");
            if msg.is_text() {
                let text = msg.into_text().unwrap();
                let Ok(event) = serde_json::from_str::<Event>(&text) else {
                    continue;
                };

                match event {
                    Event::FileOpened { buffer_id, content } => todo!(),
                    Event::BufferUpdated {
                        buffer_id: _,
                        content,
                    } => {
                        let _ = tx_clone
                            .send(AppEvent::BufferUpdated(content.to_string()))
                            .await;
                    }
                    Event::CursorMoved {
                        buffer_id,
                        position,
                    } => todo!(),
                    Event::FileSaved { buffer_id } => todo!(),
                    Event::Error { message } => todo!(),
                }
            }
        }
    });

    let tx_clone = tx.clone();
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(250));
        loop {
            interval.tick().await;
            let _ = tx_clone.send(AppEvent::Tick).await;
        }
    });

    // Terminal setup
    let mut revi = app::ReviTerminal::new()?;

    loop {
        revi.draw()?;

        if let Some(event) = rx.recv().await {
            match event {
                AppEvent::BufferUpdated(content) => revi.set_buffer(content),
                AppEvent::Tick => {}
            }
        }

        if let Some(CEvent::Key(key)) = revi.poll_input(Duration::from_millis(10))? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    Ok(())
}
