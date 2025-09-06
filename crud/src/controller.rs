use axum::{
    http::StatusCode,
    extract::{Path, Extension},
    Json,
};
use crate::model::{User, UserInfo};
use crate::user_service::UserService;
use std::sync::Arc;

pub async fn list_users(
    Extension(service): Extension<Arc<UserService>>,
) -> Result<Json<Vec<User>>, StatusCode> {
    match service.list_users().await {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_user_by_id(
    Extension(service): Extension<Arc<UserService>>,
    Path(id): Path<i32>,
) -> Result<Json<User>, StatusCode> {
    match service.get_user_by_id(id).await {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn create_user(
    Extension(service): Extension<Arc<UserService>>,
    Json(user): Json<UserInfo>,
) -> StatusCode {
    match service.create_user(user).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn update_user(
    Extension(service): Extension<Arc<UserService>>,
    Path(id): Path<i32>,
    Json(user): Json<UserInfo>,
) -> StatusCode {
    match service.update_user(id, user).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn delete_user(
    Extension(service): Extension<Arc<UserService>>,
    Path(id): Path<i32>,
) -> StatusCode {
    match service.delete_user(id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
