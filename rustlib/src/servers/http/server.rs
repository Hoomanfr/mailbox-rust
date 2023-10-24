use crate::config::vault::ConfigManager;
use axum::{http::StatusCode, response::IntoResponse, routing::*, Router};
use std::net::SocketAddr;
use std::{error::Error, future::Future};
use tower_http::cors::{Any, CorsLayer};
pub struct ApiServer {
    pub port: u16,
    pub cfg: ConfigManager,
    router: Router,
}

impl ApiServer {
    fn new(router: Router) -> ApiServer {
        ApiServer {
            port: 8080,
            cfg: ConfigManager::new(),
            router,
        }
    }

    fn initialize(&self) -> Router {
        let cors = CorsLayer::new().allow_origin(Any);
        let router = self.router();
        let router = router.route("/health-check", get(health_check));
        router.layer(cors)
    }

    pub fn router(&self) -> Router {
        self.router.clone()
    }
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "healthy").into_response()
}

pub async fn listen_and_serve<F>(callback: F) -> Option<Box<dyn Error + Send + Sync>>
where
    F: FnOnce(&ApiServer) -> Router,
{
    let router = Router::new();
    let mut api_server = ApiServer::new(router);
    api_server.router = api_server.initialize();
    api_server.router = callback(&api_server);

    let addr = SocketAddr::from(([0, 0, 0, 0], api_server.port));
    let result = axum::Server::bind(&addr)
        .serve(api_server.router.into_make_service())
        .await;
    match result {
        Ok(_) => None,
        Err(e) => e.into_cause(),
    }
}
