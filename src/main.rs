use axum::{http::StatusCode, routing::get, Router};
use dotenvy::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};
mod auth;
use crate::auth::structs::claims::Claims;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new().route("/", get(generate_jwt));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:7979").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn generate_jwt() -> Result<(StatusCode, String), (StatusCode, String)> {
    Ok((
        StatusCode::OK,
        encode(
            &Header::default(),
            &Claims {
                user_id: String::from("1"),
                exp: 3600,
            },
            &EncodingKey::from_secret("secret".as_ref()),
        )
        .unwrap(),
    ))
}
