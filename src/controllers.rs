use std::time::Duration;

use crate::bot::DATABASE;
use crate::db::Database;
use crate::parsing_data::{
    get_one_new_habr, get_one_new_hackernews, parse_starting_news,
};
use teloxide::{requests::Requester, types::ChatId, Bot};

pub async fn start_pooling(db: &Database, bot: Bot) {
    let mut last_new_hackernew = String::new();
    let mut last_new_habr = String::new();
    loop {
        // Every minute checks for hacker new
        send_last_new_hackernews(db, &bot, &mut last_new_hackernew)
            .await
            .unwrap_or_else(|err| log::error!("{}", err));
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}

pub async fn handle_start(chat_id: ChatId, bot: Bot) {
    // Send greeting message to user and news
    tokio::spawn(async move {
        if let Err(err) = bot.send_message(chat_id, "Hello snikers").await {
            log::error!("Error while sending message occures: {err}")
        }
        let news_list = parse_starting_news().await.unwrap_or_else(|err| {
            log::error!("Error getting news from hackernews: {}", err);
            vec![]
        });
        for news in news_list {
            if let Err(err) = bot.send_message(chat_id, news).await {
                log::error!("Error while sending message occures: {err}")
            }
        }
    });
    // Insert user to db
    tokio::spawn(async move {
        let db = DATABASE.get().await;
        db.insert_user(&chat_id)
            .await
            .unwrap_or_else(|err| log::error!("{}", err));
    });
}

async fn send_last_new_hackernews(
    db: &Database,
    bot: &Bot,
    last_new: &mut String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let first_new = get_one_new_hackernews().await.unwrap_or_else(|err| {
        log::error!("Error getting one new from hackernews: {}", err);
        "Error getting new from hackernews".to_string()
    });

    if first_new.as_str() != last_new {
        *last_new = first_new.clone();
        let user_ids = db.get_users_id().await;
        if let Ok(user_list) = user_ids {
            for user_id in user_list {
                if let Err(err) =
                    bot.send_message(ChatId(user_id), &first_new).await
                {
                    log::error!("{}", err)
                }
            }
        } else if let Err(err) = user_ids {
            log::error!("{}", err)
        }
    }
    Ok(())
}
