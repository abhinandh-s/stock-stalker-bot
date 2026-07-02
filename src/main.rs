const SYMBOLS: [&str; 2] = ["GROWW", "ITC"]; 

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let telegram_token = std::env::var("TELEGRAM_BOT_TOKEN")?;
    let telegram_chat_id = std::env::var("TELEGRAM_CHAT_ID")?;

    // Initialize and prime the client exactly ONCE before the loop
    let client = nse_quote::NseClient::connect().await?;

    for symbol in SYMBOLS {
        // Pass the shared client by reference
        match fetch_todays_result(&client, symbol).await {
            Ok(res) => {
                // Handle Telegram send errors gracefully instead of ignoring them
                if let Err(e) = send_to_telegram(&telegram_token, &telegram_chat_id, &res).await {
                    eprintln!("Failed to send {} to Telegram: {}", symbol, e);
                }
            }
            Err(e) => {
                // If fetching fails, we log it and move to the next symbol instead of crashing
                eprintln!("Failed to fetch {}: {}", symbol, e);
            }
        }
    }

    Ok(())
}

// Accept a reference to the initialized client
async fn fetch_todays_result(
    client: &nse_quote::NseClient, 
    symbol: &str
) -> anyhow::Result<nse_quote::Response> {
    let result = client.quote_equity(symbol).await?;
    Ok(result)
}


async fn send_to_telegram(
    token: &str,
    chat_id: &str,
    response: &nse_quote::Response,
) -> anyhow::Result<()> {
    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);

    // 1. Safely extract the symbol, falling back to "N/A" if it's missing
    let symbol = response
        .info
        .as_ref()
        .and_then(|i| i.symbol.as_deref())
        .unwrap_or("N/A");

    // 2. Safely extract the price data, falling back to 0.0 if missing
    let open = response
        .price_info
        .as_ref()
        .and_then(|p| p.open)
        .unwrap_or(0.0);

    let prev_close = response
        .price_info
        .as_ref()
        .and_then(|p| p.previous_close)
        .unwrap_or(0.0);

    let close = response
        .price_info
        .as_ref()
        .and_then(|p| p.close)
        .unwrap_or(0.0);

    let change = response
        .price_info
        .as_ref()
        .and_then(|p| p.change)
        .unwrap_or(0.0);

    let msg = format!(
        "# {}\n\n{}\n{}\n{}\n{}",
         symbol,
        open,
        prev_close,
        close,
        change
    );

    let client = reqwest::Client::new();
    client
        .post(&url)
        .form(&[
            ("chat_id", chat_id),
            ("text", msg.as_str()),
            ("parse_mode", "Markdown"),
        ])
        .send()
        .await?;

    Ok(())
}
