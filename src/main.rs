use std::{net::{IpAddr, Ipv6Addr,Ipv4Addr,SocketAddr}, env, path::Path, sync::Mutex};
use std::str::FromStr;
use dotenv::dotenv;
use axum::{Router, routing::{get, get_service}, response::{Html, IntoResponse, Response}, extract::{Query}, middleware, http::{StatusCode, Method}, body::{boxed, Body},};
use serde::Deserialize;
use tower_cookies::{CookieManagerLayer};
use tower::{ServiceBuilder, ServiceExt};
use tower_http::trace::TraceLayer;
// use tower_http::services::ServeDir;
use tracing_subscriber::{Registry,Layer,prelude::*};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::cors::{CorsLayer, Any};
use clap::Parser;
use std::fs::File;
use chrono;
pub use self::error::{Error,Result};
mod error;
mod web;
mod yumdb;
mod token;
mod apis;
mod model;
// use std::thread;
#[derive(Parser, Debug)]
#[clap(name = "server", about = "server info")]
struct Opt {
    #[clap(short = 'l', long = "log", default_value = "debug")]
    log_level: String,
    /// set the listen addr
    #[clap(short = 'a', long = "addr", default_value = "localhost")]
    addr: String,
    /// set the listen port
    #[clap(short = 'p', long = "port", default_value = "8080")]
    port: u16,
}
#[derive(Debug,Deserialize)]
struct HelloParams{
    name:Option<String>,
    // role:Option<UserRole::VIEWER(String::de)>
}

#[tokio::main]
async fn main() {
    // let mut buffer = [0; 512];
    let opt = Opt::parse();
    let sock_addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V4(Ipv4Addr::LOCALHOST)),
        opt.port,
    ));
    if !cfg!(debug_assertions){
        let date = chrono::offset::Local::now().date_naive().to_string();
        let file = File::create(Path::new(format!("log/{date}.log").as_str())).unwrap();
        let json_log = tracing_subscriber::fmt()
            .with_writer(Mutex::new(file))
            .finish();
        json_log.init();
    };
    
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level))
    };
    let cors = CorsLayer::new()
    .allow_methods(vec![Method::GET, Method::POST, Method::OPTIONS])
    .allow_origin(Any)
    .allow_credentials(false);
    // tracing_subscriber::fmt::
    // tracing_subscriber::fmt::init();
    let routes_all =Router::new()
    .merge(routes_main())
    .merge(apis::package::routes())
    .merge(web::routes_login::routes())
    .layer(middleware::map_response(main_response_mapper))
    .layer(CookieManagerLayer::new())
    .layer(cors)
    .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
    .fallback_service(routes_static());
    
    // let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    if cfg!(debug_assertions){
        log::info!("->> LISTENING on {}\n", sock_addr);
        // println!("->> LISTENING on {sock_addr}\n");
    }
    axum::Server::bind(&sock_addr)
    .serve(routes_all.into_make_service())
    .await
    .unwrap();
}


fn routes_main()->Router{
    Router::new()
    // .route("/",get(main_handler),)
    .route("/", get_service(ServeFile::new("./static/dist/index.html")))
}
async fn main_response_mapper(res:Response)->Response{
    if cfg!(debug_assertions){
        println!("->> {:<12} - main_response_mapper","RES_MAPPER");
        println!();
    };
    res
}
async fn main_handler(Query(params):Query<HelloParams>)-> impl IntoResponse{
    println!("->> {:<12} - main_handler - {params:?}","HANDLER");
    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
    // ServeFile::new("static/hello.html")
    
    // Html()
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