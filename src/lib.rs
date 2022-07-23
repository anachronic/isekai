#[macro_use]
extern crate rocket;

use rocket::{Rocket, Build};
use rocket::http::Status;
use rocket::response::Redirect;
use redis::Commands;

#[get("/<redir>")]
async fn redirect(redir: String) -> Result<Redirect, Status> {
    let url = std::env::var("REDIS_URL").unwrap();

    let client = redis::Client::open(url).unwrap();
    let mut con = client.get_connection().unwrap();

    let something: Result<String, _> = con.get(&redir);

    match something {
        Err(_) => {
            println!("Redis errored");
            return Err(Status::NotFound)
        },
        Ok(url) => {
            return Ok(Redirect::permanent(url))
        }
    }
}

pub fn build_rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![redirect])
}
