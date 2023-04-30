pub use actix_web::{post, HttpResponse, Responder};

#[post("/create")]
pub async fn create_product() -> impl Responder {
    HttpResponse::Ok().json(common::Response::<String>::new(
        200,
        true,
        "Hello World!".to_string(),
        None,
    ))
}
