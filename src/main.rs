pub mod controllers;
pub mod models;

use axum::{response::IntoResponse, routing::get, Router};
use axum_login::AuthManagerLayerBuilder;
use models::user::Auth;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use time::Duration;
use tower_sessions::{fred::prelude::*, Expiry, RedisStore, SessionManagerLayer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ////////////////// DB //////////////////////////////
    dotenvy::dotenv().expect("error loading .env");
    let db_url: String = env::var("DATABASE_URL").unwrap();

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("error connection to db");

    sqlx::migrate!("./migrations").run(&db_pool).await?;
    ////////////////// REDIS //////////////////////////////
    let pool = RedisPool::new(RedisConfig::default(), None, None, None, 6).unwrap();

    let redis_conn = pool.connect();
    pool.wait_for_connect().await.unwrap();

    let session_store = RedisStore::new(pool);

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::days(1)));

    let auth = Auth { db: db_pool };
    let auth_layer = AuthManagerLayerBuilder::new(auth, session_layer).build();

    let app = Router::new()
        .route("/", get(handler))
        .merge(controllers::auth::router())
        .layer(auth_layer);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    redis_conn.await??;

    Ok(())
}

async fn handler() -> impl IntoResponse {
    "hello"
}
