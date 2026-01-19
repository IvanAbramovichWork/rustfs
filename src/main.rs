
use axum::{
    extract::Path,
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use tokio::fs;
use tracing::info;

#[tokio::main]
async fn main() {
    info!("server started");

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/*path", get(root));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await;
}

// basic handler that responds with a static string
async fn root(Path(path): Path<String>) -> impl IntoResponse {
    let root_path = "/var/lib/rustfs/";
    let full_path = format!("{}{}", root_path, path);
    info!("{}", full_path);
    match fs::read_to_string(&full_path).await {
        Ok(contents) => (StatusCode::OK, contents).into_response(),
        Err(e) => {
            let error_msg = format!("Failed to read file: {}", e);
            let status = if e.kind() == std::io::ErrorKind::NotFound {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            (status, error_msg).into_response()
        }
    }
}
