use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/movie.swf")]
async fn movie() -> impl Responder {
    let swf_data = include_bytes!("movie.swf");
    HttpResponse::Ok()
        .content_type("application/x-shockwave-flash")
        .body(swf_data.to_vec())
}

pub fn start_internal_server() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let _server = HttpServer::new(|| App::new().service(index).service(movie))
        .listen(listener)
        .unwrap()
        .run();
    println!("Server is running on port {}", port);
    let _server_handle = tokio::spawn(_server);
    port
}
