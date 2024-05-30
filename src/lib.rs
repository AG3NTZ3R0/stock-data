use chrono::{NaiveDate, TimeZone, Utc};
use reqwest::Error as ReqwestError;
use reqwest::header::{HeaderMap, USER_AGENT};
use std::io::Error as IoError;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

/// Writes the provided bytes to the specified file path.
///
/// # Arguments
///
/// * `bytes` - The bytes to be written to the file.
/// * `path` - The path of the file to write the bytes to.
///
/// # Errors
///
/// Returns an `IoError` if there was an error writing to the file.
pub async fn write_stock_data(bytes: &[u8], path: &str) -> Result<(), IoError> {
    let mut file = File::create(path).await?;
    file.write_all(&bytes).await?;
    Ok(())
}

/// Downloads stock data from the specified URL.
///
/// # Arguments
///
/// * `url` - The URL to download the stock data from.
///
/// # Errors
///
/// Returns a `ReqwestError` if there was an error downloading the data.
pub async fn download_stock_data(url: &str) -> Result<Vec<u8>, ReqwestError> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.82 Safari/537.36".parse().unwrap());

    let client = reqwest::Client::new();
    let response = client.get(url)
        .headers(headers)
        .send()
        .await?;

    if response.status().is_success() {
        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    } else {
        Err(response.error_for_status().unwrap_err())
    }
}

/// Builds a Yahoo Finance URL for downloading stock data based on the provided dates.
///
/// # Arguments
///
/// * `symbol` - The stock symbol.
/// * `date1` - The start date.
/// * `date2` - The end date.
/// * `interval` - The interval between data points (e.g., "1d" for daily).
/// * `include_adjusted_close` - Whether to include the adjusted close price.
///
/// # Returns
///
/// The generated Yahoo Finance URL.
pub fn build_yahoo_finance_url_from_dates(
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

/// Builds a Yahoo Finance URL for downloading stock data based on the provided timestamps.
///
/// # Arguments
///
/// * `symbol` - The stock symbol.
/// * `date1` - The start timestamp.
/// * `date2` - The end timestamp.
/// * `interval` - The interval between data points (e.g., "1d" for daily).
/// * `include_adjusted_close` - Whether to include the adjusted close price.
///
/// # Returns
///
/// The generated Yahoo Finance URL.
pub fn build_yahoo_finance_url(
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use std::fs::File;
    use std::io::Read;

    #[tokio::test]
    async fn test_download_and_write_stock_data() {
        let symbol = "AMZN";
        let date1 = NaiveDate::from_ymd_opt(2021, 1, 1).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2022, 1, 1).unwrap();
        let interval = "1d";
        let include_adjusted_close = true;

        let url = build_yahoo_finance_url_from_dates(symbol, date1, date2, interval, include_adjusted_close);

        // Test downloading stock data
        let data = download_stock_data(&url).await.unwrap();
        assert!(!data.is_empty());

        // Test writing stock data to a file
        let path = "test_data.csv";
        write_stock_data(&data, path).await.unwrap();

        // Read the written file and verify its content
        let mut file = File::open(path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        assert!(!content.is_empty());

        // Clean up the test file
        std::fs::remove_file(path).unwrap();
    }
}