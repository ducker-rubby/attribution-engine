use serde::Serialize;

#[derive(Serialize)]
pub struct CreateAccountResponse {
    pub jwt: String,
}
