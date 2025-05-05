use std::error::Error;
use tokio::try_join;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let ui_task = tokio::spawn(async {
        if let Err(e) = revi_ui_terminal::start_ui().await {
            eprintln!("UI error: {}", e);
        }
    });

    try_join!(ui_task)?;

    Ok(())
}
