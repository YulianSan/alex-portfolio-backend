use aguia_vitrais_back::run;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:7979").await.unwrap();

    run(listener, "db").await.unwrap();
}

