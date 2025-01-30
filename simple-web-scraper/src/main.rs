use reqwest;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://uk.finance.yahoo.com/quote/%5EGSPC/";

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .build()?;

    let mut ticker_name = String::new();
    let mut price = String::new();

    let response = client.get(url).send().await?;
    let body = response.text().await?;
    let document = Html::parse_document(&body);

    let ticker_selector = Selector::parse("h1.yf-xxbei9").unwrap(); // Selector for the ticker name
    let price_selector = Selector::parse(r#"fin-streamer[data-testid="qsp-price"]"#).unwrap(); // Selector for the price

    if let Some(element) = document.select(&ticker_selector).next() {
        ticker_name = element.text().collect::<Vec<_>>().join("");
    }

    for element in document.select(&price_selector) {
        price = element
            .value()
            .attr("data-value")
            .unwrap_or("Not found")
            .to_string();
    }

    println!("Ticker: {}\nPrice: {}", ticker_name, price);

    Ok(())
}
