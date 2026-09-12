use axum::extract::Json;
use axum::http::StatusCode;

use crate::models::web::auth::{
    AuthRequest, create_account::CreateAccountResponse, login::LoginResponse,
};

pub async fn foo() {
    println!("Auth")
}

pub async fn login(Json(payload): Json<AuthRequest>) -> (StatusCode, Json<LoginResponse>) {
    println!("login with: {:#?}", payload);

    (
        StatusCode::OK,
        Json(LoginResponse {
            jwt: "jwt_placeholder".into(),
        }),
    )
}

pub async fn create_account(
    Json(payload): Json<AuthRequest>,
) -> (StatusCode, Json<CreateAccountResponse>) {
    println!("create account with: {:#?}", payload);

    (
        StatusCode::OK,
        Json(CreateAccountResponse {
            jwt: "jwt_placeholder".into(),
        }),
    )
}
