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
    let service = Arc::new(Service::new(args.data_dir, state)?);

    let app = Router::new()
        .route("/", get(root))
        .route("/paste", post(post_paste))
        .route("/paste/{id}", get(get_paste))
        .layer(Extension(Arc::clone(&service)));

    let address = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    let fut =
        axum::serve(listener, app).with_graceful_shutdown(shutdown_signal(service, args.state.clone()));

    println!("Listening on {address}");
    fut.await.unwrap();

    Ok(())
}

async fn shutdown_signal(service: Arc<Service>, path: std::path::PathBuf) {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Couldn't install Ctrl+C handler")
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Couldn't install signal handler")
            .recv()
            .await
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => finalize_state(service, path.as_path()),
        _ = terminate => finalize_state(service, path.as_path()),
    }
}

fn finalize_state(service: Arc<Service>, path: &std::path::Path) {
    match service.dump_state(path) {
        Ok(()) => println!("State dump performed successfully"),
        Err(e) => println!("Couldn't dump the state: {}", e),
    }
}

async fn root() -> &'static str {
    "Hello!"
}

async fn get_paste(
    Extension(service): Extension<Arc<Service>>,
    Path(paste_id): Path<Uuid>,
) -> Response {
    match service.read(&paste_id) {
        Ok(content) => content.into_response(),
        Err(e) => (StatusCode::NOT_FOUND, e.to_string()).into_response(),
    }
}

#[debug_handler]
async fn post_paste(Extension(service): Extension<Arc<Service>>, body: String) -> Response {
    match service.create(body, None) {
        Ok(id) => id.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
