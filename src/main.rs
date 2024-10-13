use axum::{
    extract::Extension,
    routing::{get,post},
    Router,
};
use std::env;

use crate::repositories::{TodoRepository,TodoRepositoryForMemory};

use handlers::create_todo;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {

    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG",log_level);
    tracing_subscriber::fmt::init();

    let repository = TodoRepositoryForMemory::new();
    let app = create_app(repository);
    let addr = SocketAddr::from(([127 , 0 , 0 ,1],3000));
    tracing::debug!("listening on {}",addr);

    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

fn create_app<T: TodoRepository>(repository:T) -> Router {
    Router::new()
        .route("/",get(root))
        .route("/todos",post(create_todo::<T>))
        .layer(Extension(Arc::new(repository)))
}


async fn root() -> &'static str {
    "Hello world"
}


