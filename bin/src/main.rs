use std::net::SocketAddr;
use axum::ServiceExt;
use futures::executor::block_on;
mod multiplex_service;
use crate::multiplex_service::MultiplexService;
use config::CONFIG;


#[tokio::main]
async fn main() {
    // initialize tracing
    //tracing_subscriber::fmt::init();

    dotenv::dotenv().ok();
    // for (k, v) in std::env::vars() {
    //     println!("{}: {}", k, v);
    // }

    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init();
    //log::info!("insert a new row into the database ");

    match std::env::var("DATABASE_URL") {
        Ok(data) => println!("option i some, data = {:?}", data),
        Err(e) => println!("err env"),
    }

    //libe::json::toString();

    //let re = libe::mysql::mysql();
    //println!("ok");
    //println!("{re:?}");

    // if let Err(err) = block_on(re) {
    //     panic!("{}", err);
    // }

    common::db::find().await.unwrap();

    // build the http service
    let app = api::router().await;

    // build the grpc service
    let grpc = grpc::init_grpc();

    // combine them into one service
    let service = MultiplexService::new(app, grpc);


    let addr = SocketAddr::from(([0, 0, 0, 0], CONFIG.server.port));
    // run it with hyper on localhost:3000
    // axum::Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();

    hyper::Server::bind(&addr)
        .serve(tower::make::Shared::new(service))
        .await
        .unwrap();


}
// 处理器


