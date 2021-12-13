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

#[derive(Debug, sqlx::FromRow)]
pub struct AccessToken {
    pub uuid: Uuid,
    pub user_id: String,
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i32,
    pub scope: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct RefreshToken {
    // TODO created_at and updated_at: datetime
    pub uuid: uuid::Uuid,
    pub user_id: String,
    pub refresh_token: String,
    pub token_type: String,
    pub scope: String,
}
