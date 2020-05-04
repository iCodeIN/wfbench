use actix_web::{get, post, web, App, HttpServer, Responder};

#[get("/demo")]
async fn demo() -> impl Responder {
    "Hello"
}

#[get("/demo/{id}/{name}/other.html")]
async fn other(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

#[post("/demo/{id}/{name}/other.html")]
async fn other_post(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

#[get("/demo/{id}/{name}/test.html")]
async fn test(info: web::Path<(u16, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

#[get("/demo/{id}/{name}/index.html")]
async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
