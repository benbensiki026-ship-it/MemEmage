use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UserSignup {
    #[validate(length(min = 3, max = 30))]
    pub username: String,
    
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UserLogin {
    #[validate(length(min = 1))]
    pub username: String,
    
    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInfo {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}

impl From<User> for UserInfo {
    fn from(user: User) -> Self {
        UserInfo {
            id: user.id,
            username: user.username,
            email: user.email,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Meme {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub image_url: String,
    pub top_text: Option<String>,
    pub bottom_text: Option<String>,
    pub template_name: Option<String>,
    pub views: i32,
    pub likes: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateMemeRequest {
    #[validate(length(min = 1, max = 100))]
    pub title: String,
    
    pub top_text: Option<String>,
    pub bottom_text: Option<String>,
    pub template_name: Option<String>,
    
    // Base64 encoded image or template selection
    pub image_data: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemeResponse {
    pub id: Uuid,
    pub title: String,
    pub image_url: String,
    pub top_text: Option<String>,
    pub bottom_text: Option<String>,
    pub username: String,
    pub views: i32,
    pub likes: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    pub username: String,
    pub exp: usize, // expiration
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse {
            success: true,
            data: Some(data),
            error: None,
        }
    }
    
    pub fn error(message: String) -> Self {
        ApiResponse {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}
