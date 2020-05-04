#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/demo")]
fn demo() -> &'static str {
    "Hello"
}

#[get("/demo/<id>/<name>/other.html")]
fn other(id: u32, name: String) -> String {
    format!("Hello {}! id:{}", name, id)
}

#[post("/demo/<id>/<name>/other.html")]
fn other_post(id: u32, name: String) -> String {
    format!("Hello {}! id:{}", name, id)
}

#[get("/demo/<id>/<name>/test.html")]
fn test(id: u32, name: String) -> String {
    format!("Hello {}! id:{}", name, id)
}

#[get("/demo/<id>/<name>/index.html")]
fn index(id: u32, name: String) -> String {
    format!("Hello {}! id:{}", name, id)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![demo, other, other_post, test, index])
        .launch();
}
