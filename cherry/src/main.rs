mod db;
pub(crate) mod model;
pub(crate) mod routes;
mod schema;
mod server_utils;

use crate::{
    model::CherryData,
    routes::{graphiql, graphql_handler, health_check},
    schema::{MutationRoot, QueryRoot},
};

use async_graphql::{EmptySubscription, Schema};
use axum::{extract::Extension, routing::get, Router, Server};
use server_utils::shutdown_signal;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let data = Arc::new(RwLock::new(CherryData::load()?));
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(data.clone())
        .finish();

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .route("/health_check", get(health_check))
        .layer(Extension(schema));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal(data))
        .await?;

    Ok(())
}
