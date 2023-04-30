use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use env::BackendConfig;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(common::Response::<String>::new(
        200,
        true,
        "Hello World!".to_string(),
        None,
    ))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // envs
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
