use chrono::{NaiveDate, Utc};
use chrono::TimeZone;
use reqwest::Error as ReqwestError;
use std::error::Error;
use std::io::Error as IoError;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let msg = "Failed to generate date";
    let date1 = NaiveDate::from_ymd_opt(2023, 01, 01).expect(msg);
    let date2 = NaiveDate::from_ymd_opt(2024, 01, 01).expect(msg);
    
    let url = build_yahoo_finance_url_from_dates("NVDA", date1, date2, "1d", true);
    let bytes = download_stock_data(&url).await?;

    let path = "output/stock_data.csv"; 
    write_stock_data(&bytes, &path).await?;

    Ok(())
}

async fn write_stock_data(
    bytes: &[u8],
    path: &str,
) -> Result<(), IoError> {
    let mut file = File::create(path).await?;
    file.write_all(&bytes).await?;

    Ok(())
}

async fn download_stock_data(
    url: &str,
) -> Result<Vec<u8>, ReqwestError> {
    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    } else {
        Err(response.error_for_status().err().unwrap())
    }
}

fn build_yahoo_finance_url_from_dates(
    symbol: &str,
    date1: NaiveDate,
    date2: NaiveDate,
    interval: &str,
    include_adjusted_close: bool,
) -> String {
    let msg = "Failed to generate timestamp from date";
    let datetime1 = date1.and_hms_opt(0, 0, 0).expect(msg);
    let datetime2 = date2.and_hms_opt(0, 0, 0).expect(msg);
    
    let period1 = Utc.from_utc_datetime(&datetime1).timestamp() as u64;
    let period2 = Utc.from_utc_datetime(&datetime2).timestamp() as u64;
    build_yahoo_finance_url(symbol, period1, period2, interval, include_adjusted_close)
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
