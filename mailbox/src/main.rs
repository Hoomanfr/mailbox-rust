mod application;
mod domain;
mod infrastructure;

use crate::application::app::ApplicationFactoryImpl;
use crate::application::routers::get_routers;
use crate::infrastructure::db::DbFactoryImpl;
use axum::{Extension, Router};
use rustlib::servers::http::server::*;

#[tokio::main]
async fn main() {
    listen_and_serve(bootstrap).await;
}

fn bootstrap(api_server: &ApiServer) -> Router {
    let db_factory = futures::executor::block_on(DbFactoryImpl::new(&api_server.cfg));
    let app_factory = ApplicationFactoryImpl::new(db_factory);
    let router = api_server.router();
    get_routers(router).layer(Extension(app_factory.clone()))
}
