extern crate lazy_static;
extern crate pretty_env_logger;

mod bot;
mod controllers;
mod db;
mod entity;
mod error;
mod migration;
mod parsing_data;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    bot::run().await;
}
