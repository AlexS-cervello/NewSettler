use scraper::{Html, Selector};

const URL_HACKN: &'static str = "https://news.ycombinator.com/";
const SEL_HACKN: &'static str = "span.titleline>a";
const URL_HABR: &'static str = "https://habr.com";
const SEL_HABR_HREF: &'static str = "article>div>h2>a";
const SEL_HABR_TITLE: &'static str = "article>div>h2>a>span";

async fn get_parsing_vars(
    url: &str,
    selector: &Vec<&'static str>,
) -> Result<(Html, Vec<Selector>), Box<dyn std::error::Error + Send + Sync>> {
    let response = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&response);
    let selectors = selector
        .into_iter()
        .map(|el| Selector::parse(el).unwrap()) // TODO Replace that unwrap
        .collect::<Vec<Selector>>();

    Ok((document, selectors))
}

pub async fn parse_starting_news(
) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let mut result: Vec<String> = vec![];
    let (document, selector) =
        get_parsing_vars(URL_HACKN, &vec![SEL_HACKN]).await?;
    // TODO replace with filter_map
    document
        .select(&selector[0])
        .map(|el| {
            format!(
                "{}\n{}",
                el.inner_html(),
                el.value().attr("href").unwrap_or("").to_string()
            )
        })
        .zip(0..10)
        .for_each(|(val, _)| {
            let compare = &val.replace(|c: char| !c.is_ascii(), "");
            if compare != "Ask HN:" {
                result.push(val);
            }
        });
    Ok(result)
}

pub async fn get_one_new_hackernews(
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let (document, selector) =
        get_parsing_vars(URL_HACKN, &vec![SEL_HACKN]).await?;
    let result = document
        .select(&selector[0])
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
    let compare = &result.replace(|c: char| !c.is_ascii(), "");
    if compare == "Ask HN:" {
        return Ok("".to_string());
    }
    Ok(result)
}

pub async fn get_one_new_habr(
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let (document, selector) =
        get_parsing_vars(URL_HABR, &vec![SEL_HABR_TITLE, SEL_HABR_HREF])
            .await?;
    let title = document
        .select(&selector[0])
        .map(|el| format!("{}", el.inner_html()))
        .next()
        .unwrap_or("".to_string());
    let href = document
        .select(&selector[1])
        .map(|el| {
            format!(
                "{}{}",
                URL_HABR,
                el.value().attr("href").unwrap_or("").to_string()
            )
        })
        .next()
        .unwrap_or("".to_string());
    let result = format!("{}\n{}", title, href);
    Ok(result)
}
