use crate::config::AuthConfig;
use crate::shared::api_response::ApiResponse;
use axum::http::Uri;
use axum::{
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i64,
    pub email: String,
    pub exp: usize,
}

#[derive(Clone)]
pub struct AuthMiddleware {
    pub auth_config: AuthConfig,
}

impl AuthMiddleware {
    pub fn new(auth_config: AuthConfig) -> Self {
        Self { auth_config }
    }

    fn is_excluded_path(&self, uri: &Uri) -> bool {
        let path = uri.path();
        self.auth_config.exclude_paths.iter().any(|excluded_path| {
            // 精确匹配或者前缀匹配
            path == excluded_path
                || (excluded_path.ends_with("*")
                    && path.starts_with(excluded_path.trim_end_matches('*')))
        })
    }
    pub async fn auth_middleware(
        &self,
        request: Request,
        next: Next,
    ) -> Result<Response, Response> {
        let uri = request.uri().clone();
        // 检查当前路径是否在排除列表中
        if self.is_excluded_path(&uri) {
            return Ok(next.run(request).await);
        }
        // 获取请求头中的Authorization字段
        let auth_header = request
            .headers()
            .get("x-auth-token")
            .and_then(|header| header.to_str().ok())
            // .and_then(|header| header.strip_prefix("Bearer "))
            .ok_or_else(|| {
                // 缺少token时返回统一格式错误
                let error_response =
                    ApiResponse::<()>::error_with_code(401, "Missing authorization token");

                Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(serde_json::to_string(&error_response).unwrap().into())
                    .unwrap()
            })?;

        // 验证JWT token
        let token = auth_header;
        let jwt_secret = env::var("JWT_SECRET")
            .unwrap_or_else(|_| "default_secret".to_string());

        let decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());
        let validation = Validation::default();

        match decode::<Claims>(token, &decoding_key, &validation) {
            Ok(_) => {
                // Token有效，继续处理请求
                Ok(next.run(request).await)
            }
            Err(_) => {
                log::error!("Invalid or expired token: {}", token);
                // Token无效，返回401和统一格式的错误信息
                let error_response =
                    ApiResponse::<()>::error_with_code(401, "Invalid or expired token");

                let response = Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(serde_json::to_string(&error_response).unwrap().into())
                    .unwrap();

                Err(response)
            }
        }
    }
}

// 为了向后兼容，保留原有的函数
pub async fn auth_middleware(
    request: Request,
    next: Next,
) -> Result<Response, Response> {
    // 默认排除路径配置
    let auth_config = AuthConfig {
        exclude_paths: vec![],
    };

    let middleware = AuthMiddleware::new(auth_config);
    middleware.auth_middleware(request, next).await
}