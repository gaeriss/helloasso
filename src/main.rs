mod errors;

use errors::*;

#[actix_web::main]
async fn main() -> Result {
    envir::init();

    let ip = envir::get("LISTEN_IP")?;
    let port = envir::get("LISTEN_PORT")?;
    let bind = format!("{}:{}", ip, port);

    actix_web::HttpServer::new(move || {
        actix_web::App::new().route("/campaign.json", actix_web::web::get().to(campaign))
    })
    .bind(&bind)?
    .run()
    .await?;

    Ok(())
}

async fn campaign() -> Result<actix_web::HttpResponse> {
    const URL: &str = "https://www.helloasso.com/associations/la-brasserie-communale-extraordinaire/collectes/achat-de-la-brasserie";

    let html = attohttpc::get(URL).send()?.text()?;
    let document = scraper::Html::parse_document(&html);

    let response = actix_web::HttpResponse::Ok().json(serde_json::json!({
        "funded": funded(&document)?,
        "objective": objective(&document)?,
    }));

    Ok(response)
}

fn funded(document: &scraper::Html) -> Result<i32> {
    scrape_number(
        document,
        ".CampaignContent .CardNumbersSticky .CardNumbers--Main span b:nth-child(2)",
    )
}

fn objective(_: &scraper::Html) -> Result<i32> {
    Ok(18_000)
}

fn scrape_number(document: &scraper::Html, selector: &str) -> Result<i32> {
    static REGEX: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(
        || regex::Regex::new(r#"[^\d^\.]+"#).unwrap()
    );

    let number = scrape(document, selector)?;

    REGEX
        .replace_all(&number, "")
        .parse::<f32>()
        .map(|x| x as i32)
        .map_err(Error::from)
}

fn scrape(document: &scraper::Html, selector: &str) -> Result<String> {
    let selector = scraper::Selector::parse(selector).map_err(|_| Error::Selector)?;
    let element = document.select(&selector).next().unwrap();
    Ok(element.text().next().unwrap_or_default().to_string())
}
