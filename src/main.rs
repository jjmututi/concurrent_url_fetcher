use reqwest::Client;
use std::time::Instant;
use futures::future::join_all;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();

    // List of URLs to fetch concurrently
    let urls = vec![
        "https://www.rust-lang.org",
        "https://www.github.com",
        "https://www.google.com",
        "https://www.wikipedia.org",
        "https://www.reddit.com",
    ];

    println!("🚀 Fetching {} URLs concurrently...\n", urls.len());

    // Create an HTTP client
    let client = Client::new();

    // Create a vector to hold all the futures
    let fetches: Vec<_> = urls
        .into_iter()
        .map(|url| {
            let client = &client;
            async move {
                match client.get(url).send().await {
                    Ok(response) => {
                        let status = response.status();
                        let content_type = response
                            .headers()
                            .get("content-type")
                            .and_then(|ct| ct.to_str().ok())
                            .map(|s| s.to_string());
                        (url, Some(status), content_type)
                    }
                    Err(e) => {
                        eprintln!("❌ Error fetching {}: {}", url, e);
                        (url, None, None)
                    }
                }
            }
        })
        .collect();

    // Execute all fetches concurrently and wait for all to complete
    let results = join_all(fetches).await;

    // Print results
    println!("📊 Results:\n");
    println!("{:<40} | {:10} | {}", "URL", "Status", "Content-Type");
    println!("{}", "-".repeat(70));

    for (url, status, content_type) in results {
        let status_str = match status {
            Some(s) => s.as_str().to_string(),
            None => "ERROR".to_string(),
        };
        let content_type_str = content_type.unwrap_or_else(|| "unknown".to_string());
        println!("{:<40} | {:10} | {}", url, status_str, content_type_str);
    }

    let duration = start.elapsed();
    println!("\n✅ Completed in: {:?}", duration);

    Ok(())
}