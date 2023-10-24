use super::{mailbox_api::*, user_api::*};
use axum::{routing::*, Router};

pub fn get_routers(router: Router) -> Router {
    let router = get_mailbox_routers(router);
    let router = get_user_routers(router);
    router
}

fn get_mailbox_routers(router: Router) -> Router {
    router
        .route("/v1/mailboxes", post(create_mailbox))
        .route("/v1/mailboxes/user/:id", get(find_user_mailboxes))
}

fn get_user_routers(router: Router) -> Router {
    router
        .route("/v1/users", post(create_user))
        .route("/v1/users/:id", get(find_user))
}
