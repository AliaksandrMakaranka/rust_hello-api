use actix_web::{web, App, HttpServer, Responder};

async fn hello() -> impl Responder {
    "Hello world from RUST with love"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route("/hello", web::get().to(hello))
    })
        .bind("127.0.0.1:3033")?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, http};

    #[actix_web::test]
    async fn test_hello_endpoint() {
        let app = test::init_service(App::new().route("/hello", web::get().to(hello))).await;
        let req = test::TestRequest::get().uri("/hello").to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), http::StatusCode::OK);

        let body = test::read_body(resp).await;
        assert_eq!(body, "Hello world from RUST with love");
    }
}
