#[macro_use]
extern crate rocket;

use rocket::{response::Redirect, http::Status};
use redis::Commands;

#[get("/<redir>")]
async fn redirect(redir: String) -> Result<Redirect, Status> {
    let client = redis::Client::open("redis://127.0.0.1").unwrap();
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

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![redirect])
}
