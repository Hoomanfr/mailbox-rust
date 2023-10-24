use axum::{Extension, Json, response::IntoResponse, http::StatusCode, extract::Path};

use super::{mailbox_app::{ MailboxRequest}, app::{ AppFactory}};

pub async fn create_mailbox(
    app: Extension<AppFactory>,
    Json(r): Json<MailboxRequest>,
) -> impl IntoResponse {
    let result = app.mailbox_app().create(r).await;
    match result{
        Ok(id) => (StatusCode::OK, id).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}

pub async fn find_user_mailboxes(
    app: Extension<AppFactory>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let result = app.mailbox_app().find_by_user_id(id).await;
    match result{
        Ok(Some(mailboxes)) => (StatusCode::OK, Json(mailboxes)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json("mailboxes not found")).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}