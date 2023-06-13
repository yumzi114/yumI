use std::{net::SocketAddr, env};
use dotenv::dotenv;
use axum::{Router, routing::{get, get_service}, response::{Html, IntoResponse, Response}, extract::Query, middleware,};
use serde::Deserialize;
use tower_cookies::{CookieManagerLayer};
use tower_http::services::ServeDir;
pub use self::error::{Error,Result};
mod error;
mod web;
mod yumdb;
mod token;
// use std::thread;
enum UserRole {
    OWNER(String),
    ADMIN(String),
    USER(String),
    VIEWER(String),
}
#[derive(Debug,Deserialize)]
struct HelloParams{
    name:Option<String>,
    // role:Option<UserRole::VIEWER(String::de)>
}
#[tokio::main]
async fn main() {
    // let mut buffer = [0; 512];
    // dotenv().ok();
    let routes_all =Router::new()
    .merge(routes_main())
    .merge(web::routes_login::routes())
    .layer(middleware::map_response(main_response_mapper))
    .layer(CookieManagerLayer::new())
    .fallback_service(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
    .serve(routes_all.into_make_service())
    .await
    .unwrap();
}
fn routes_main()->Router{
    Router::new()
    .route("/",get(main_handler),)
}
async fn main_response_mapper(res:Response)->Response{
    println!("->> {:<12} - main_response_mapper","RES_MAPPER");
    println!();
    res
}
async fn main_handler(Query(params):Query<HelloParams>)-> impl IntoResponse{
    println!("->> {:<12} - main_handler - {params:?}","HANDLER");
    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
}
fn routes_static()->Router{
    Router::new().nest_service("/static", get_service(ServeDir::new("./static/")))
}

// use std::{
//     fs,
//     io::{prelude::*, BufReader},
//     net::{TcpListener, TcpStream},
//     thread,
//     time::Duration,
// };
// use yumi::ThreadPool;
// use std::net::{TcpStream, TcpListener};
// fn main() {
//     let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
//     let pool = ThreadPool::new(4);

//     for stream in listener.incoming() {
//         let stream = stream.unwrap();

//         pool.execute(|| {
//             handle_connection(stream);
//         });
//     }
//     println!("Shutting down.");
// }

// // --snip--

// fn handle_connection(mut stream: TcpStream) {
//     let buf_reader = BufReader::new(&mut stream);
//     let request_line = buf_reader.lines().next().unwrap().unwrap();

//     let (status_line, filename) = match &request_line[..] {
//         "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
//         "GET /sleep HTTP/1.1" => {
//             ("HTTP/1.1 200 OK", "hello.html")
//         }
//         _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
//     };
//     let contents = fs::read_to_string(filename).unwrap();
//     let length = contents.len();

//     let response =
//         format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
//     stream.write_all(response.as_bytes()).unwrap();
// }