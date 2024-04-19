use std::error::Error;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://query1.finance.yahoo.com/v7/finance/download/NVDA?period1=1508112000&period2=1713561246&interval=1d&events=history&includeAdjustedClose=true";
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
