use std::sync::Arc;

use axum::{
    Extension, Router, debug_handler,
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use clap::Parser;
use service::Service;
use state::State;
use uuid::Uuid;

mod cli;
mod service;
mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();
    let state = State::load(&args.state)?;
    let service = Service::new(args.data_dir, state)?;

    let app = Router::new()
        .route("/", get(root))
        .route("/paste", post(post_paste))
        .route("/paste/{id}", get(get_paste))
        .layer(Extension(Arc::new(service)));

    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    let fut = axum::serve(listener, app);
    println!("Listening on {address}");
    fut.await.unwrap();

    Ok(())
}

async fn root() -> &'static str {
    "Hello!"
}

async fn get_paste(Path(paste_id): Path<Uuid>) -> String {
    format!("Got GET /paste/{paste_id} request")
}

#[debug_handler]
async fn post_paste(Extension(service): Extension<Arc<Service>>, body: String) -> Response {
    match service.create(body, None) {
        Ok(id) => id.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
