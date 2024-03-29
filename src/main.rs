mod handler;
mod error;
mod deserialize;

use actix_web::{middleware::Logger, web::{self, Data}, HttpServer, App};
use dotenv::dotenv;
use std::env;


// define app state information
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct ChannelInfo {
    channel_id_: String,
    channel_secret_: String,
    channel_access_token_: String,
}

impl ChannelInfo {
    pub fn new() -> ChannelInfo {
        dotenv().ok();
        ChannelInfo {
            channel_id_: env::var("CHANNEL_ID").unwrap(),
            channel_secret_: env::var("CHANNEL_SECRET").unwrap(),
            channel_access_token_: env::var("CHANNEL_ACCESS_TOKEN").unwrap(),
        }
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // import dotenv value
    let appinfo = Data::new(ChannelInfo::new());

    // logger set
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at http://localhost:8080");

    // build server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::clone(&appinfo))
            .route("/webhook", web::get().to(handler::get_test))
            .route("/webhook", web::post().to(handler::post_test))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    
}



