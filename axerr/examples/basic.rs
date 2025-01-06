use axerr::AxErr;
use axum::{routing::get, Router};

#[derive(AxErr)]
pub enum ErrorType {
    InternalError,

    #[axerr(
        status_code = 404,
        public_message = "The requested resource could not be found"
    )]
    NotFound,

    #[axerr(
        status_code = 403,
        public_message = "You do not have the required permissions to perform this action"
    )]
    Forbidden,
}

#[tokio::main]
pub async fn main() {
    let app: Router<()> = Router::new()
        .route("/error", get(|| async { ErrorType::InternalError }))
        .route("/not-found", get(|| async { ErrorType::NotFound }))
        .route("/forbidden", get(|| async { ErrorType::Forbidden }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap()
}
