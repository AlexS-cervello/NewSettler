use crate::controllers::{handle_start, start_pooling};
use crate::db::Database;
use crate::error::Error;
use async_once::AsyncOnce;
use lazy_static::lazy_static;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use teloxide::{prelude::*, types::Me, utils::command::BotCommands};

const LOG_FILENAME: &'static str = "settler.log";

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

fn set_token() -> Result<(), Error> {
    let token = std::env::var("BOT_TOKEN")?;
    std::env::set_var("TELOXIDE_TOKEN", token);
    Ok(())
}

pub async fn run() -> Result<(), Error> {
    set_token()?;
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build(LOG_FILENAME)?;
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))?;
    log4rs::init_config(config)?;
    println!("Logging to {LOG_FILENAME}");
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
    Ok(())
}

async fn message_handler(
    bot: Bot,
    msg: Message,
    me: Me,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
