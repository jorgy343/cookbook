mod recipes;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().attach(recipes::stage())
}
