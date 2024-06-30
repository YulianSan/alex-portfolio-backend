use std::{
    collections::linked_list, env, time::{SystemTime, UNIX_EPOCH}
};

use aguia_vitrais_back::run;
use axum::{http::StatusCode, routing::get, Router};
use dotenvy::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};
mod application;
mod domain;
mod infrastructure;
mod adapters;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:7979").await.unwrap();

    run(listener, "db").await.unwrap();
}

