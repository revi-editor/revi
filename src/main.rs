use std::error::Error;
use tokio::try_join;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server_task = tokio::spawn(async {
        if let Err(e) = revi_core::start_backend().await {
            eprintln!("Backend error: {}", e);
        }
    });

    let ui_task = tokio::spawn(async {
        if let Err(e) = revi_ui_terminal::start_ui().await {
            eprintln!("UI error: {}", e);
        }
    });

    try_join!(server_task, ui_task)?;

    Ok(())
}
