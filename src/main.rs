use yahoo_finance_api as yahoo;
use time::OffsetDateTime;

#[tokio::main]
async fn main() {
    let provider = yahoo::YahooConnector::new().unwrap();
    
    // get the latest quotes in 1 minute intervals
    let response = provider.get_latest_quotes("GROWW.NS", "1d").await.unwrap();
    
    // extract just the latest valid quote summary
    // including timestamp, open, close, high, low, volume
    let quote = response.last_quote().unwrap();
    
    // Note: depending on the crate version, you may need to cast timestamp `as i64`
    let time: OffsetDateTime =
        OffsetDateTime::from_unix_timestamp(quote.timestamp.try_into().unwrap()).unwrap();
        
    println!("At {} quote price of Apple was {}", time, quote.close);
}
