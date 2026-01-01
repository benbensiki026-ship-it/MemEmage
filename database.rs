use sqlx::{PgPool, postgres::PgPoolOptions};
use uuid::Uuid;
use crate::models::{User, Meme, CreateMemeRequest};

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}

pub async fn create_user(
    pool: &PgPool,
    username: &str,
    email: &str,
    password_hash: &str,
) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (id, username, email, password_hash, created_at, updated_at)
        VALUES ($1, $2, $3, $4, NOW(), NOW())
        RETURNING *
        "#
    )
    .bind(Uuid::new_v4())
    .bind(username)
    .bind(email)
    .bind(password_hash)
    .fetch_one(pool)
    .await?;
    
    Ok(user)
}

pub async fn get_user_by_username(
    pool: &PgPool,
    username: &str,
) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE username = $1"
    )
    .bind(username)
    .fetch_optional(pool)
    .await?;
    
    Ok(user)
}

pub async fn get_user_by_email(
    pool: &PgPool,
    email: &str,
) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1"
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;
    
    Ok(user)
}

pub async fn create_meme(
    pool: &PgPool,
    user_id: Uuid,
    request: &CreateMemeRequest,
    image_url: &str,
) -> Result<Meme, sqlx::Error> {
    let meme = sqlx::query_as::<_, Meme>(
        r#"
        INSERT INTO memes (id, user_id, title, image_url, top_text, bottom_text, template_name, views, likes, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, 0, 0, NOW())
        RETURNING *
        "#
    )
    .bind(Uuid::new_v4())
    .bind(user_id)
    .bind(&request.title)
    .bind(image_url)
    .bind(&request.top_text)
    .bind(&request.bottom_text)
    .bind(&request.template_name)
    .fetch_one(pool)
    .await?;
    
    Ok(meme)
}

pub async fn get_memes(
    pool: &PgPool,
    limit: i64,
    offset: i64,
) -> Result<Vec<Meme>, sqlx::Error> {
    let memes = sqlx::query_as::<_, Meme>(
        "SELECT * FROM memes ORDER BY created_at DESC LIMIT $1 OFFSET $2"
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;
    
    Ok(memes)
}

pub async fn get_meme_by_id(
    pool: &PgPool,
    id: Uuid,
) -> Result<Option<Meme>, sqlx::Error> {
    let meme = sqlx::query_as::<_, Meme>(
        "SELECT * FROM memes WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    
    Ok(meme)
}

pub async fn get_user_memes(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<Meme>, sqlx::Error> {
    let memes = sqlx::query_as::<_, Meme>(
        "SELECT * FROM memes WHERE user_id = $1 ORDER BY created_at DESC"
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;
    
    Ok(memes)
}

pub async fn increment_meme_views(
    pool: &PgPool,
    id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE memes SET views = views + 1 WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    
    Ok(())
}

pub async fn increment_meme_likes(
    pool: &PgPool,
    id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE memes SET likes = likes + 1 WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    
    Ok(())
}
