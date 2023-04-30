pub use actix_web::{delete, HttpResponse, Responder};

#[delete("/delete")]
pub async fn delete_all() -> impl Responder {
    HttpResponse::Ok().json(common::Response::<String>::new(
        200,
        true,
        "Hello World!".to_string(),
        None,
    ))
}

#[delete("/delete/:product_id")]
pub async fn delete_product() -> impl Responder {
    HttpResponse::Ok().json(common::Response::<String>::new(
        200,
        true,
        "Hello World!".to_string(),
        None,
    ))
}
