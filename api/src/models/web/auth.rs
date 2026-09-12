use serde::Deserialize;

pub mod create_account;
pub mod login;

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    pub email: String,
    pub password: String,
}
