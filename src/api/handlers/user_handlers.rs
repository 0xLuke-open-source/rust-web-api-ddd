// 用户相关处理器
use crate::api::app_state::AppState;
use crate::shared::api_response::ApiResponse;
use crate::application::dtos::user_dto::{CreateUserRequest, LoginRequest};
use crate::{domain::entities::UpdateUser, shared::errors::error::Error};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use hyper::HeaderMap;
use std::sync::Arc;

pub async fn create_user_handler(
    State(app_state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(request): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, Error> {
    // 验证请求头中的token
    if let Some(auth_token) = headers.get("x-auth-token") {
        println!(
            "Auth Token: {}",
            auth_token.to_str().unwrap_or("Invalid token")
        );
    }
    if let Some(client_id) = headers.get("x-client-id") {
        println!(
            "Client ID: {}",
            client_id.to_str().unwrap_or("Unknown client")
        );
    }
    // 从AppState中获取user_service
    let user_response = app_state.user_service.create_user(request).await?;
    Ok((StatusCode::CREATED, Json(user_response)))
}

pub async fn get_user_handler(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, Error> {
    let user_response = app_state.user_service.get_user(id).await?;
    Ok((StatusCode::OK, Json(user_response)))
}

pub async fn update_user_handler(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(update_user): Json<UpdateUser>,
) -> Result<impl IntoResponse, Error> {
    let user_response = app_state.user_service.update_user(id, update_user).await?;
    Ok((StatusCode::OK, Json(user_response)))
}

pub async fn delete_user_handler(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, Error> {
    app_state.user_service.delete_user(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_all_user_handler(
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    let user_responses = app_state.user_service.get_all_users().await?;
    Ok((StatusCode::OK, Json(user_responses)))
}

pub async fn login_handler(
    State(app_state): State<Arc<AppState>>,
    Json(login_request): Json<LoginRequest>,
) -> Result<impl IntoResponse, Error> {
    match app_state.user_service.login(login_request).await {
        Ok(login_response) => {
            let response = ApiResponse::success_with_msg(login_response, "登录成功");
            Ok(Json(response))
        }
        Err(e) => Err(e),
    }
}
