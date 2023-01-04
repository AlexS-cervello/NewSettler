use scraper::{Html, Selector};

async fn get_parsing_vars(
    url: &str,
    selector: &'static str,
) -> Result<(Html, Selector), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&response);
    let selector = Selector::parse(selector)?;

    Ok((document, selector))
}

pub async fn parse_starting_news(
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut result: Vec<String> = vec![];
    let (document, selector) =
        get_parsing_vars("https://news.ycombinator.com/", "span.titleline>a")
            .await?;
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

pub async fn get_one_new_hackernews(
) -> Result<String, Box<dyn std::error::Error>> {
    let (document, selector) =
        get_parsing_vars("https://news.ycombinator.com/", "span.titleline>a")
            .await?;
    let result = document
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
    if &result[0..7] == "Ask HN:" {
        return Ok("".to_string());
    }
    Ok(result)
}

pub async fn get_one_new_habr() -> Result<String, Box<dyn std::error::Error>> {
    let (document, selector) =
        get_parsing_vars("https://habr.com", "article>div>h2>a").await?;
    let result = document
        .select(&selector)
        .map(|el| format!("{}", el.inner_html()))
        .next()
        .unwrap_or("".to_string());
    Ok(result)
}
