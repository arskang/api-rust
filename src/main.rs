use actix_web::{App, HttpServer};
mod services;
mod helpers;
mod models;
mod database;

// Convierte el main en una función asyncrona
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(services::version)
        .service(services::get_users)
        .service(services::get_user)
        .service(services::create_user)
        .service(services::update_user)
        .service(services::delete_user)
        .service(services::get_error_404)
        .service(services::post_error_404)
        .service(services::put_error_404)
        .service(services::delete_error_404)
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
}
