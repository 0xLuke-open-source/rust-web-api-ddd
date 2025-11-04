// 菜单相关处理器
use crate::api::app_state::AppState; // 新增导入
use crate::application::dtos::menu_dto::CreateMenuRequest;
use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::sync::Arc;
use crate::shared::errors::error::Error;

pub async fn create_menu_handler(
    State(app_state): State<Arc<AppState>>,
    Json(request): Json<CreateMenuRequest>,
) -> Result<impl IntoResponse, Error> {
    Ok((
        StatusCode::CREATED,
        Json(app_state.menu_service.create_menu(request).await?),
    ))
}

pub async fn get_menu_by_id(
    State(app_state): State<Arc<AppState>>,
    axum::extract::Path(id): axum::extract::Path<i64>,
) -> Result<impl IntoResponse, Error> {
    Ok((
        StatusCode::OK,
        Json(app_state.menu_service.get_menu_by_id(id).await?),
    ))
}

pub async fn get_all_menus(
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    Ok((
        StatusCode::OK,
        Json(app_state.menu_service.get_all_menus().await?),
    ))
}
