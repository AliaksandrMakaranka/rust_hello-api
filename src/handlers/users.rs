use actix_web::{get, post, web, HttpResponse, Responder};
use crate::models::{User, CreateUserRequest};
use std::sync::Mutex;
use std::sync::Arc;

// Thread-safe state management
pub struct AppState {
    users: Mutex<Vec<User>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            users: Mutex::new(Vec::new()),
        }
    }
}

#[get("/users")]
pub async fn get_users(data: web::Data<Arc<AppState>>) -> impl Responder {
    let users = data.users.lock().unwrap().clone();
    HttpResponse::Ok().json(users)
}

#[get("/users/{id}")]
pub async fn get_user(
    data: web::Data<Arc<AppState>>,
    id: web::Path<u32>
) -> impl Responder {
    let user_id = id.into_inner();
    let users = data.users.lock().unwrap();
    let user = users.iter().find(|u| u.id == user_id);
    
    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("User not found"),
    }
}

#[post("/users")]
pub async fn create_user(
    data: web::Data<Arc<AppState>>,
    user: web::Json<CreateUserRequest>
) -> impl Responder {
    let mut users = data.users.lock().unwrap();
    let new_user = User {
        id: users.len() as u32 + 1,
        name: user.name.clone(),
        email: user.email.clone(),
    };
    
    users.push(new_user.clone());
    HttpResponse::Created().json(new_user)
} 