use std::time::Duration;

use crate::bot::DATABASE;
use crate::db::{Database, Error};
use crate::parsing_data::{
    get_one_new_habr, get_one_new_hackernews, parse_starting_news,
};
use teloxide::{requests::Requester, types::ChatId, Bot};

pub async fn start_pooling(db: &Database, bot: Bot) {
    let mut last_new_hackernew = String::new();
    let mut last_new_habr = String::new();
    let mut count: u16 = 1;
    loop {
        // Every minute checks for news
        let timer = tokio::spawn(tokio::time::sleep(Duration::from_secs(60)));

        send_last_new_hackernews(db, &bot, &mut last_new_hackernew)
            .await
            .unwrap_or_else(|err| log::error!("{}", err));

        if count % 60 == 0 {
            send_last_new_habr(db, &bot, &mut last_new_habr)
                .await
                .unwrap_or_else(|err| log::error!("{}", err));
            count = 0;
        }
        count += 1;
        timer.await.unwrap();
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
) -> Result<(), Error> {
    let first_new = get_one_new_hackernews().await?;
    if first_new.as_str() != last_new {
        *last_new = first_new.clone();
        let user_ids = db.get_users_id().await?;
        for user_id in user_ids {
            bot.send_message(ChatId(user_id), &first_new).await?;
        }
    }
    Ok(())
}

async fn send_last_new_habr(
    db: &Database,
    bot: &Bot,
    last_new: &mut String,
) -> Result<(), Error> {
    let first_new = get_one_new_habr().await?;
    if first_new.as_str() != last_new {
        *last_new = first_new.clone();
        let user_ids = db.get_users_id().await?;
        for user_id in user_ids {
            bot.send_message(ChatId(user_id), &first_new).await?;
        }
    }
    Ok(())
}
