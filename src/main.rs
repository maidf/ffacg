
use axum::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;

use crate::route::user::{login, register};



mod model;
mod repo;
mod route;
mod service;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let listener = TcpListener::bind("127.0.0.1:9901").await.unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://postgres:1234@db.pzlymqpqpfvajyryrfnn.supabase.co:5432/postgres")
        .await
        .expect("can't connect to db");
    let app = Router::new()
        .route("/register", get(register))
        .route("/login", get(login))
        .with_state(pool);

    axum::serve(listener, app).await.unwrap();
}