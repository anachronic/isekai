#[macro_use]
extern crate rocket;

use rocket::response::Redirect;

#[get("/<redir>")]
async fn redirect(redir: String) -> Redirect {
    println!("called with {}", redir);

    Redirect::permanent("https://anachronic.io/wiki")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![redirect])
}
