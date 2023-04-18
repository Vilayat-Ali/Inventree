use crate::routes::user::create::create_user;

#[macro_use]
pub extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![index])
}
