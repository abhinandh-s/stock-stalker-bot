use yahoo_finance_api as yahoo;
use time::OffsetDateTime;

#[tokio::main]
async fn main() {
let telegram_token = std::env::var("TELEGRAM_BOT_TOKEN")?;
    let telegram_chat_id = std::env::var("TELEGRAM_CHAT_ID")?;
    let provider = yahoo::YahooConnector::new().unwrap();
    
    // get the latest quotes in 1 minute intervals
    let response = provider.get_latest_quotes("GROWW.NS", "1d").await.unwrap();
    
    // extract just the latest valid quote summary
    // including timestamp, open, close, high, low, volume
    let quote = response.last_quote().unwrap();
    
    // Note: depending on the crate version, you may need to cast timestamp `as i64`
    let time: OffsetDateTime =
        OffsetDateTime::from_unix_timestamp(quote.timestamp.try_into().unwrap()).unwrap();
        

if let Err(e) = send_to_telegram(&telegram_token, &telegram_chat_id, &quote.close).await {
                    eprintln!("Failed to send {} to Telegram: {}", symbol, e);
                }
}

async fn send_to_telegram(
    token: &str,
    chat_id: &str,
    msg: &str,
) -> anyhow::Result<()> {
    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);

    
    let client = reqwest::Client::new();
    client
        .post(&url)
        .form(&[
            ("chat_id", chat_id),
            ("text", msg),
            ("parse_mode", "Markdown"),
        ])
        .send()
        .await?;

    Ok(())
}