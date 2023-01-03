use std::time::Duration;

use crate::bot::DATABASE;
use crate::db::Database;
use scraper::{Html, Selector};
use teloxide::{requests::Requester, types::ChatId, Bot};

pub async fn start_pooling(db: &Database, bot: Bot) {
    let mut last_new = String::new();
    loop {
        // Check if first new is changed
        // if yes then iterate through all chat_ids in db
        // time check = 1 min
        let first_new = get_one_new().await.unwrap_or_else(|err| {
            log::error!("Error getting one new: {}", err);
            "Error getting new".to_string()
        });

        if first_new != last_new {
            last_new = first_new.clone();
            let user_ids = db.get_users_id().await;
            if let Ok(user_list) = user_ids {
                for user_id in user_list {
                    if let Err(err) = bot.send_message(ChatId(user_id), &first_new).await {
                        log::error!("{}", err)
                    }
                }
            } else if let Err(err) = user_ids {
                log::error!("{}", err)
            }
        }
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}

pub async fn handle_start(chat_id: ChatId, bot: Bot) {
    // Send greeting message to user and news
    tokio::spawn(send_first_news(chat_id.clone(), bot.clone()));
    // Insert user to db
    tokio::spawn(async move {
        let db = DATABASE.get().await;
        db.insert_user(&chat_id)
            .await
            .unwrap_or_else(|err| log::error!("{}", err));
    });
}

pub async fn parse_news() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut result: Vec<String> = vec![];
    let response = reqwest::get("https://news.ycombinator.com/").await?.text().await?;

    let document = Html::parse_document(&response);
    let selector = Selector::parse("span.titleline>a")?;
    // TODO replace with filter_map
    document
        .select(&selector)
        .map(|el| {
            format!(
                "{}\n{}",
                el.inner_html(),
                el.value().attr("href").unwrap_or("").to_string()
            )
        })
        .zip(0..10)
        .for_each(|(val, _)| {
            if &val[0..7] != "Ask HN:" {
                result.push(val);
            }
        });
    Ok(result)
}

async fn get_one_new() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let response = reqwest::get("https://news.ycombinator.com/").await?;
    let text = response.text().await?;
    let document = Html::parse_document(&text);
    let selector = Selector::parse("span.titleline>a").unwrap();
    let first = document
        .select(&selector)
        .map(|el| {
            format!(
                "{}\n{}",
                el.inner_html(),
                el.value().attr("href").unwrap_or("").to_string()
            )
        })
        .next()
        .unwrap_or("".to_string());

    // Exception - no data here
    if &first[0..7] == "Ask HN:" {
        return Ok("".to_string())
    }
    Ok(first)
}

async fn send_first_news(chat_id: ChatId, bot: Bot) {
    if let Err(err) = bot.send_message(chat_id, "Hello snikers").await {
        log::error!("Error while sending message occures: {err}")
    }
    let news_list = parse_news().await.expect("Error getting news.");
    for news in news_list {
        if let Err(err) = bot.send_message(chat_id, news).await {
            log::error!("Error while sending message occures: {err}")
        }
    }
}
