pub use actix_web::{HttpResponse, Responder};

pub async fn create_product() -> impl Responder {
    HttpResponse::Ok().json(common::Response::<String>::new(
        200,
        true,
        "Hello World!".to_string(),
        None,
    ))
}
