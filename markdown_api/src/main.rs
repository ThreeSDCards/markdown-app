#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

mod endpoints;
mod model;
mod database;
mod cors;

use crate::cors::*;

use endpoints::*;
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index,
    ep_get_documents_id,
    ep_get_documents,
    ep_post_documents,
    ep_put_documents,
    ep_del_documents])
    .attach(CORS)
}