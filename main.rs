use actix_web::{web, App, HttpServer, middleware};
use actix_cors::Cors;
use actix_files::Files;
use dotenv::dotenv;
use std::env;

use mememage_backend::database;
use mememage_backend::handlers::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://mememage:password@localhost/mememage".to_string());
    
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    
    // Create database pool
    log::info!("Connecting to database...");
    let pool = database::create_pool(&database_url)
        .await
        .expect("Failed to create database pool");
    
    log::info!("Starting MemEmage server at {}:{}", host, port);
    
    // Start HTTP server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .wrap(cors)
            // API routes
            .service(
                web::scope("/api")
                    .route("/health", web::get().to(health_check))
                    // Auth routes
                    .route("/auth/signup", web::post().to(signup))
                    .route("/auth/login", web::post().to(login))
                    // Meme routes
                    .route("/memes", web::post().to(create_meme))
                    .route("/memes", web::get().to(get_memes))
                    .route("/memes/{id}", web::get().to(get_meme))
                    .route("/memes/{id}/like", web::post().to(like_meme))
                    .route("/memes/user/my-memes", web::get().to(get_user_memes))
            )
            // Static files
            .service(Files::new("/uploads", "./uploads"))
            // Frontend (if built)
            .service(Files::new("/", "./frontend/dist").index_file("index.html"))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
