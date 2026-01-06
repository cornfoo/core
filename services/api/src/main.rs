use api::server;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let hostname: &str = "127.0.0.1";
    let port: u16 = 8080;
    let addr = format!("{}:{}", hostname, port);
    let listener = TcpListener::bind(addr)?;

    let server = server::run_http(listener)?;
    server.await.expect("server error");

    Ok(())
}
