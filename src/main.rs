mod errors;

use errors::*;

#[actix_web::main]
async fn main() -> Result {
    #[cfg(debug_assertions)]
    envir::dotenv();

    env_logger::init();

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
    scrape(
        document,
        ".CampaignContent .CardNumbersSticky .CardNumbers--Main span b:nth-child(2)",
    )?
    .trim_end_matches('€')
    .parse()
    .map_err(Error::from)
}

fn objective(document: &scraper::Html) -> Result<i32> {
    let objective = scrape(document, ".CampaignCards .CardNumbers--Goal")?;

    let regex = regex::Regex::new(r#"[^\d]+"#)?;
    regex
        .replace_all(&objective, "")
        .parse()
        .map_err(Error::from)
}

fn scrape(document: &scraper::Html, selector: &str) -> Result<String> {
    let selector = scraper::Selector::parse(selector).map_err(|_| Error::Selector)?;
    let element = document.select(&selector).next().unwrap();
    Ok(element.text().next().unwrap_or_default().to_string())
}
