# Stock Data

## Description
A Rust crate for efficiently downloading historical stock data from Yahoo Finance, featuring asynchronous operations with reqwest and tokio for optimal performance.

## Example
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let msg = "Failed to generate date";
    let date1 = NaiveDate::from_ymd_opt(2023, 01, 01).expect(msg);
    let date2 = NaiveDate::from_ymd_opt(2024, 01, 01).expect(msg);

    let url = build_yahoo_finance_url_from_dates("AMZN", date1, date2, "1d", true);
    let bytes = download_stock_data(&url).await?;

    let path = "output/stock_data.csv"; 
    write_stock_data(&bytes, &path).await?;

    Ok(())
}
```