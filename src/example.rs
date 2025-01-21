use std::error::Error;
use reqwest::{Client, header};

#[tokio::main]
pub async fn crawler() -> Result<String, Box<dyn Error>> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .gzip(true)
        .brotli(true)
        .deflate(true)
        .pool_idle_timeout(Some(std::time::Duration::from_secs(30)))
        .pool_max_idle_per_host(5)
        .tcp_keepalive(std::time::Duration::from_secs(60))
        .build()?;

    let mut headers = header::HeaderMap::new();
    headers.insert(header::ACCEPT_ENCODING, header::HeaderValue::from_static("gzip, deflate, br"));
    headers.insert(header::CONNECTION, header::HeaderValue::from_static("keep-alive"));
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"
        ),
    );

    let url = "https://example.com";
    let response = match client.get(url).headers(headers.clone()).send().await {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("Initial request failed: {}", e);
            println!("Retrying with HTTP/1.1 only...");
           
            let http1_client = Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .http1_only()
                .build()?;
           
            http1_client.get(url).headers(headers).send().await?
        }
    };
   
    if !response.status().is_success() {
        return Err(format!("Request failed with status: {}", response.status()).into());
    }

    let bytes = response.bytes().await?;
    let html = String::from_utf8_lossy(&bytes).to_string();
    Ok(html)
}