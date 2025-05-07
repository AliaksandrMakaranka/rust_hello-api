mod handlers;
mod models;
mod tests;

use actix_web::{web, App, HttpServer};
use std::sync::Arc;
use handlers::users::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = Arc::new(AppState::new());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(handlers::hello::hello)
            .service(handlers::users::get_users)
            .service(handlers::users::get_user)
            .service(handlers::users::create_user)
            .service(handlers::health::health_check)
    })
    .bind("127.0.0.1:3033")?
    .run()
    .await
}
