use actix_web::{test, App, http, web};
use crate::handlers::{hello, users, health};
use crate::models::CreateUserRequest;
use std::sync::Arc;
use crate::handlers::users::AppState;

#[actix_web::test]
async fn test_hello_endpoint() {
    let app = test::init_service(App::new().service(hello::hello)).await;
    let req = test::TestRequest::get().uri("/hello").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    assert_eq!(body, "Hello world from RUST with love");
}

#[actix_web::test]
async fn test_health_check() {
    let app = test::init_service(App::new().service(health::health_check)).await;
    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    assert!(body_str.contains("healthy"));
    assert!(body_str.contains("version"));
}

#[actix_web::test]
async fn test_get_users_empty() {
    let app_state = Arc::new(AppState::new());
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .service(users::get_users)
    ).await;
    
    let req = test::TestRequest::get().uri("/users").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    assert_eq!(body, "[]");
}

#[actix_web::test]
async fn test_create_and_get_user() {
    let app_state = Arc::new(AppState::new());
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .service(users::create_user)
            .service(users::get_user)
            .service(users::get_users)
    ).await;

    // Create a user
    let user = CreateUserRequest {
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
    };
    
    let create_req = test::TestRequest::post()
        .uri("/users")
        .set_json(&user)
        .to_request();
    
    let create_resp = test::call_service(&app, create_req).await;
    assert!(create_resp.status().is_success());

    // Get the created user
    let get_req = test::TestRequest::get().uri("/users/1").to_request();
    let get_resp = test::call_service(&app, get_req).await;
    assert!(get_resp.status().is_success());
    
    let body = test::read_body(get_resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    assert!(body_str.contains("Test User"));
    assert!(body_str.contains("test@example.com"));

    // Verify user is in the list
    let list_req = test::TestRequest::get().uri("/users").to_request();
    let list_resp = test::call_service(&app, list_req).await;
    assert!(list_resp.status().is_success());
    
    let list_body = test::read_body(list_resp).await;
    let list_body_str = std::str::from_utf8(&list_body).unwrap();
    assert!(list_body_str.contains("Test User"));
    assert!(list_body_str.contains("test@example.com"));
}

#[actix_web::test]
async fn test_get_nonexistent_user() {
    let app_state = Arc::new(AppState::new());
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .service(users::get_user)
    ).await;
    
    let req = test::TestRequest::get().uri("/users/999").to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
    let body = test::read_body(resp).await;
    assert_eq!(body, "User not found");
}

#[actix_web::test]
async fn test_create_user_invalid_data() {
    let app_state = Arc::new(AppState::new());
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(app_state))
            .service(users::create_user)
    ).await;
    
    // Test with empty name
    let invalid_user = CreateUserRequest {
        name: "".to_string(),
        email: "test@example.com".to_string(),
    };
    
    let req = test::TestRequest::post()
        .uri("/users")
        .set_json(&invalid_user)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success()); // Note: We might want to add validation later
} 