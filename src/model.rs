use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct RegistrationToken {
    pub uuid: Uuid,
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i32,
    pub scope: String,
}
