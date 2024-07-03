use tokio::net::TcpListener;
mod infrastructure;
mod adapters;
mod application;
mod domain;
pub mod schema;

pub async fn run(listener: TcpListener, db_name: &str) -> Result<(), std::io::Error> {
    Ok(infrastructure::server(listener, db_name).await?)
}
