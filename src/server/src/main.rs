use actix_web::{web, App, HttpServer};

// routes
use server::routes::products::create::create_product;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // envs
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api")
                .service(
                    web::scope("/product")
                        .route("/create", web::post().to(create_product))
                        .route("/get", web::get().to(create_product))
                        .route("/update", web::put().to(create_product))
                        .route("/delete", web::delete().to(create_product)),
                )
                .service(
                    web::scope("/user")
                        .route("/register", web::post().to(create_product))
                        .route("/login", web::get().to(create_product)),
                ),
        )
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
