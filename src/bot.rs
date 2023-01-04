use crate::controllers::{handle_start, start_pooling};
use crate::db::Database;
use async_once::AsyncOnce;
use lazy_static::lazy_static;
use std::error::Error;
use teloxide::{prelude::*, types::Me, utils::command::BotCommands};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text")]
    Help,
    #[command(description = "start")]
    Start,
}

lazy_static! {
    pub static ref DATABASE: AsyncOnce<Database> =
        AsyncOnce::new(async { Database::new().await.unwrap() });
}

fn set_token() {
    let token = std::env::var("BOT_TOKEN");
    if let Ok(token) = token {
        std::env::set_var("TELOXIDE_TOKEN", token);
    } else {
        println!("Environment variable BOT_TOKEN is not set");
        std::process::exit(0);
    }
}
pub async fn run() {
    set_token();
    pretty_env_logger::init();
    log::info!("Starting NewSettler Bot");

    DATABASE.get().await.apply_migrations().await.unwrap();

    let bot = Bot::from_env();
    tokio::spawn(start_pooling(DATABASE.get().await, bot.clone()));

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler));
    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn message_handler(
    bot: Bot,
    msg: Message,
    me: Me,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(text) = msg.text() {
        match BotCommands::parse(text, me.username()) {
            Ok(Command::Help) => {
                bot.send_message(
                    msg.chat.id,
                    Command::descriptions().to_string(),
                )
                .await?;
            }
            Ok(Command::Start) => handle_start(msg.chat.id, bot).await,
            Err(_) => {
                bot.send_message(msg.chat.id, "Command not found.").await?;
            }
        }
    }
    Ok(())
}
