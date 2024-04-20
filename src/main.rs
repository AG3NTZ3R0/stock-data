use std::error::Error;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = build_yahoo_finance_url("NVDA", 1508112000, 1713561246, "1d", true);

    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let bytes = response.bytes().await?;
        let mut file = File::create("output/stock_data.csv").await?;
        file.write_all(&bytes).await?;
        println!("Successfully downloaded CSV");
    } else {
        println!("Failed to download CSV: {}", response.status());
    }

    Ok(())
}

fn build_yahoo_finance_url(
    symbol: &str,
    period1: u64, 
    period2: u64, 
    interval: &str, 
    include_adjusted_close: bool,
) -> String {
    format!(
        "https://query1.finance.yahoo.com/v7/finance/download/{}?period1={}&period2={}&interval={}&events=history&includeAdjustedClose={}",
        symbol,
        period1,
        period2,
        interval,
        include_adjusted_close
    )
}
