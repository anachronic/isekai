use isekai;
use rocket::http::Status;
use rocket::local::blocking::Client;
use redis::Commands;

fn flush_redis() {
    let url = std::env::var("REDIS_URL").expect("set the redis url");
    let client = redis::Client::open(url).unwrap();
    let mut conn = client.get_connection().unwrap();

    redis::cmd("flushall").execute(&mut conn);
}

fn set_url(key: &str, val: &str) {
    let url = std::env::var("REDIS_URL").expect("set the redis url");
    let client = redis::Client::open(url).unwrap();
    let mut conn = client.get_connection().unwrap();

    conn.set::<&str, &str, ()>(key, val).ok();
}

#[test]
fn test_notfound_at_root() {
    flush_redis();
    let client = Client::tracked(isekai::build_rocket()).unwrap();

    let response = client.get("/").dispatch();

    assert_eq!(response.status(), Status::NotFound)
}

#[test]
fn test_fails_with_non_set_key() {
    flush_redis();

    let client = Client::tracked(isekai::build_rocket()).unwrap();
    let response = client.get("/hello_world").dispatch();

    assert_eq!(response.status(), Status::NotFound)
}

#[test]
fn test_resolves_to_set_key_in_redis() {
    flush_redis();
    set_url("ddg", "https://duckduckgo.com");

    let client = Client::tracked(isekai::build_rocket()).unwrap();

    let response = client.get("/ddg").dispatch();

    assert_eq!(response.status(), Status::PermanentRedirect)
}
