use revi_core::server;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:9001";
    if let Err(e) = server::run_server(addr).await {
        eprintln!("Server error: {}", e);
    }
}
