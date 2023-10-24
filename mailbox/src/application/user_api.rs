use axum::{Extension, Json, response::IntoResponse, http::StatusCode, extract::Path};
use super::{user_app::{ UserRequest}, app::AppFactory};

pub async fn create_user(
    app: Extension<AppFactory>,
    Json(r): Json<UserRequest>,
) -> impl IntoResponse {
    let result = app.user_app().create(r).await;
    match result{
        Ok(id) => (StatusCode::OK, id).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}

pub async fn find_user(
    app: Extension<AppFactory>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let result = app.user_app().find_by_id(id).await;
    match result{
        Ok(Some(user)) => (StatusCode::OK, Json(user)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Json("user not found")).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}