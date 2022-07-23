use isekai::build_rocket;
use rocket::launch;

#[launch]
fn rocket() -> _ {
    build_rocket()
}
