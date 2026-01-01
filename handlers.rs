use actix_web::{web, HttpResponse, HttpRequest};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;
use std::fs;
use std::path::Path;

use crate::models::*;
use crate::database;
use crate::auth::{hash_password, verify_password, create_jwt, decode_jwt, extract_token_from_header};
use crate::image_ffi::MemeProcessor;

// Health check endpoint
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse::success("MemEmage API is running"))
}

// User signup
pub async fn signup(
    pool: web::Data<PgPool>,
    user_data: web::Json<UserSignup>,
) -> HttpResponse {
    // Validate input
    if let Err(errors) = user_data.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            format!("Validation error: {:?}", errors)
        ));
    }
    
    // Check if username already exists
    match database::get_user_by_username(&pool, &user_data.username).await {
        Ok(Some(_)) => {
            return HttpResponse::Conflict().json(ApiResponse::<()>::error(
                "Username already exists".to_string()
            ));
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                format!("Database error: {}", e)
            ));
        }
        _ => {}
    }
    
    // Check if email already exists
    match database::get_user_by_email(&pool, &user_data.email).await {
        Ok(Some(_)) => {
            return HttpResponse::Conflict().json(ApiResponse::<()>::error(
                "Email already exists".to_string()
            ));
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                format!("Database error: {}", e)
            ));
        }
        _ => {}
    }
    
    // Hash password
    let password_hash = match hash_password(&user_data.password) {
        Ok(hash) => hash,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                format!("Failed to hash password: {}", e)
            ));
        }
    };
    
    // Create user
    match database::create_user(&pool, &user_data.username, &user_data.email, &password_hash).await {
        Ok(user) => {
            let token = match create_jwt(&user.id.to_string(), &user.username) {
                Ok(t) => t,
                Err(e) => {
                    return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                        format!("Failed to create token: {}", e)
                    ));
                }
            };
            
            HttpResponse::Created().json(ApiResponse::success(AuthResponse {
                token,
                user: user.into(),
            }))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                format!("Failed to create user: {}", e)
            ))
        }
    }
}

// User login
pub async fn login(
    pool: web::Data<PgPool>,
    login_data: web::Json<UserLogin>,
) -> HttpResponse {
    // Validate input
    if let Err(errors) = login_data.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            format!("Validation error: {:?}", errors)
        ));
    }
    
    // Get user
    let user = match database::get_user_by_username(&pool, &login_data.username).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
                "Invalid credentials".to_string()
            ));
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                format!("Database error: {}", e)
            ));
        }
    };
    
    // Verify password
    match verify_password(&login_data.password, &user.password_hash) {
        Ok(true) => {
            let token = match create_jwt(&user.id.to_string(), &user.username) {
                Ok(t) => t,
                Err(e) => {
                    return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                        format!("Failed to create token: {}", e)
                    ));
                }
            };
            
            HttpResponse::Ok().json(ApiResponse::success(AuthResponse {
                token,
                user: user.into(),
            }))
        }
        Ok(false) => {
            HttpResponse::Unauthorized().json(ApiResponse::<()>::error(
                "Invalid credentials".to_string()
            ))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
                format!("Failed to verify password: {}", e)
            ))
        }
    }
}

// Helper function to extract user from JWT
fn get_user_from_request(req: &HttpRequest) -> Result<Claims, String> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or("Missing authorization header")?;
    
    let token = extract_token_from_header(auth_header)
        .ok_or("Invalid authorization header format")?;
    
    decode_jwt(token).map_err(|e| format!("Invalid token: {}", e))
}

// Create meme
pub async fn create_meme(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    meme_data: web::Json<CreateMemeRequest>,
) -> HttpResponse {
    // Authenticate user
    let claims = match get_user_from_request(&req) {
        Ok(c) => c,
        Err(e) => {
            return HttpResponse::Unauthorized().json(ApiResponse::<()>::error(e));
        }
    };
    
    // Validate input
    if let Err(errors) = meme_data.validate() {
        return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            format!("Validation error: {:?}", errors)
        ));
    }
    
    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                "Invalid user ID".to_string()
            ));
        }
    };
    
    // Generate file paths
    let meme_id = Uuid::new_v4();
    let input_path = format!("uploads/templates/default.jpg");
    let output_filename = format!("{}.jpg", meme_id);
    let output_path = format!("uploads/memes/{}", output_filename);
    
    // Ensure directories exist
    fs::create_dir_all("uploads/memes").ok();
    
    // Process meme using C++ library
    let processor = MemeProcessor::new();
    let top_text = meme_data.top_text.as_deref().unwrap_or("");
    let bottom_text = meme_data.bottom_text.as_deref().unwrap_or("");
    
    if let Err(e) = processor.create_classic_meme(&input_path, top_text, bottom_text, &output_path) {
        return HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            format!("Failed to process meme: {}", e)
        ));
    }
    
    // Save to database
    let image_url = format!("/uploads/memes/{}", output_filename);
    match database::create_meme(&pool, user_id, &meme_data, &image_url).await {
        Ok(meme) => HttpResponse::Created().json(ApiResponse::success(meme)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            format!("Failed to save meme: {}", e)
        )),
    }
}

// Get all memes
pub async fn get_memes(
    pool: web::Data<PgPool>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> HttpResponse {
    let limit: i64 = query.get("limit").and_then(|l| l.parse().ok()).unwrap_or(20);
    let offset: i64 = query.get("offset").and_then(|o| o.parse().ok()).unwrap_or(0);
    
    match database::get_memes(&pool, limit, offset).await {
        Ok(memes) => HttpResponse::Ok().json(ApiResponse::success(memes)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            format!("Failed to fetch memes: {}", e)
        )),
    }
}

// Get meme by ID
pub async fn get_meme(
    pool: web::Data<PgPool>,
    meme_id: web::Path<Uuid>,
) -> HttpResponse {
    match database::get_meme_by_id(&pool, *meme_id).await {
        Ok(Some(meme)) => {
            // Increment view count
            let _ = database::increment_meme_views(&pool, *meme_id).await;
            HttpResponse::Ok().json(ApiResponse::success(meme))
        }
        Ok(None) => HttpResponse::NotFound().json(ApiResponse::<()>::error(
            "Meme not found".to_string()
        )),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            format!("Database error: {}", e)
        )),
    }
}

// Get user's memes
pub async fn get_user_memes(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let claims = match get_user_from_request(&req) {
        Ok(c) => c,
        Err(e) => {
            return HttpResponse::Unauthorized().json(ApiResponse::<()>::error(e));
        }
    };
    
    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::BadRequest().json(ApiResponse::<()>::error(
                "Invalid user ID".to_string()
            ));
        }
    };
    
    match database::get_user_memes(&pool, user_id).await {
        Ok(memes) => HttpResponse::Ok().json(ApiResponse::success(memes)),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            format!("Failed to fetch memes: {}", e)
        )),
    }
}

// Like meme
pub async fn like_meme(
    pool: web::Data<PgPool>,
    meme_id: web::Path<Uuid>,
) -> HttpResponse {
    match database::increment_meme_likes(&pool, *meme_id).await {
        Ok(_) => HttpResponse::Ok().json(ApiResponse::success("Meme liked")),
        Err(e) => HttpResponse::InternalServerError().json(ApiResponse::<()>::error(
            format!("Failed to like meme: {}", e)
        )),
    }
}
