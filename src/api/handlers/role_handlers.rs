use crate::api::AppState;
use crate::application::dtos::role_dto::CreateRoleRequest;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::sync::Arc;
use crate::shared::errors::error::Error;


pub async fn create_role_handler(
    State(app_state): State<Arc<AppState>>,
    Json(request): Json<CreateRoleRequest>,
) -> Result<impl IntoResponse, Error> {
    Ok((
        StatusCode::CREATED,
        Json(app_state.role_service.create_role(request).await?),
    ))
}
