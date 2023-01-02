extern crate lazy_static;
extern crate pretty_env_logger;

mod bot;
mod controllers;
mod db;
mod entity;
mod migration;

#[tokio::main]
async fn main() {
    bot::run().await;
}
