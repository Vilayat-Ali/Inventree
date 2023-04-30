pub use actix_web::{get, HttpRequest, HttpResponse, Responder};

#[get("/get")]
pub async fn get_all_products() -> impl Responder {
    HttpResponse::Ok().json(common::Response::<String>::new(
        200,
        true,
        "Hello World!".to_string(),
        None,
    ))
}

#[get("/get/:product_id")]
pub async fn get_product() -> impl Responder {
    HttpResponse::Ok().json(common::Response::<String>::new(
        200,
        true,
        "Hello World!".to_string(),
        None,
    ))
}
