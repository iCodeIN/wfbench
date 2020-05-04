use warp::Filter;

#[tokio::main(max_threads=10000)]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let demo = warp::path!("demo")
        .map(|| "Hello");

    let index = warp::path!("demo" / u32 / String / "index.html")
        .map(|id, name| format!("Hello {}! id:{}", name, id));

    let other = warp::path!("demo" / u32 / String / "other.html")
        .map(|id, name| format!("Hello {}! id:{}", name, id));

    let other_post = warp::path!("demo" / u32 / String / "other.html")
        .map(|id, name| format!("Hello {}! id:{}", name, id));

    let test = warp::path!("demo" / u16 / String / "test.html")
        .map(|id, name| format!("Hello {}! id:{}", name, id));

    let get = warp::get().and(demo.or(other).or(test).or(index));
    let post = warp::post().and(other_post);
    let routes = post.or(get);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 8080))
        .await;
}
