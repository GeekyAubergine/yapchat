#![allow(unused)]

use std::sync::Arc;

use dotenvy_macro::dotenv;

use axum::{
    body::Body,
    extract::Request,
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method, Response, StatusCode,
    },
    middleware,
    routing::{get, post},
    Json, Router,
};
use error::DatabaseError;
use infrastructure::appstate::AppStateData;
use routes::router;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    normalize_path::NormalizePathLayer,
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tower_livereload::LiveReloadLayer;
use tracing::Level;
// #![feature(duration_constructors)]
// #[cfg_attr(target_arch = "arm", unstable(feature = "stdarch_aarch32_crc32", issue = "XXXX"))]
// #[cfg_attr(not(target_arch = "arm"), stable(feature = "stdarch_aarch64_crc32", since = "1.80.0"))]
#[macro_use]
extern crate lazy_static;

pub mod domain;
pub mod error;
pub mod infrastructure;
pub mod prelude;
pub mod routes;
pub mod utils;

use prelude::*;

pub mod build_data {
    include!(concat!(env!("OUT_DIR"), "/build_data.rs"));
}

pub fn get_build_date() -> String {
    build_data::BUILD_DATE.to_string()
}

async fn prepare_database() -> Result<Pool<Postgres>> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&dotenv!("DATABASE_URL"))
        .await
        .map_err(DatabaseError::from_connection_error)
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let database = (prepare_database().await?);

    let mut state = AppStateData::new(database).await;

    let state = Arc::new(state);

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_origin("ws://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let services = ServiceBuilder::new()
        .layer(LiveReloadLayer::new())
        .layer(NormalizePathLayer::trim_trailing_slash())
        .layer(cors);

    let asset_files = ServeDir::new("./_assets");
    let static_files = ServeDir::new("./static");
    let cdn = ServeDir::new("./cdn");

    let app = router()
        .with_state(state)
        .nest_service("/static", static_files)
        .nest_service("/assets", asset_files)
        .nest_service("/cdn", cdn)
        .layer(services)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
