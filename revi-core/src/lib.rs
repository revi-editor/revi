pub mod server;

pub async fn start_backend() -> tokio::io::Result<()> {
    server::run_server("127.0.0.1:9001").await
}
