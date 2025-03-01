use reqwest::Client;
use tokio::time::Instant;

async fn download_url(client: &Client, url: &str) -> Result<serde_json::Value, reqwest::Error> {
    let response: reqwest::Response = client.get(url).send().await?;
    let content_length: u64 = response.content_length().unwrap_or(0);
    println!("Read {} from {}", content_length, url);
    let json:serde_json::Value = response.json::<serde_json::Value>().await?;
    Ok(json)
}

async fn download_all_urls(urls: Vec<String>) {
    let client:Client = Client::new();
    let mut tasks: Vec<tokio::task::JoinHandle<Option<_>>> = Vec::new();

    for url in urls {
        let client_ref: Client = client.clone();
        let task: tokio::task::JoinHandle<Option<_>> = tokio::spawn(async move {
            match download_url(&client_ref, &url).await {
                Ok(response) => Some(response),
                Err(err) => {
                    eprintln!("Error downloading {}: {}", url, err);
                    None
                }
            }
        });
        tasks.push(task);
    }

    let results: Vec<Result<Option<serde_json::Value>, tokio::task::JoinError>> = futures::future::join_all(tasks).await;
    println!("{}", "#".repeat(100));
    for result in results {
        if let Ok(Some(response)) = result {
            println!("{:?}", response);
        }
    }
}

#[tokio::main]
async fn main() {
    let urls: Vec<String> = (0..10)
        .map(|i: i32| format!{"http://localhost:5555/{}", i})
        .collect();

    let start_time: Instant = Instant::now();
    download_all_urls(urls).await;
    let duration: std::time::Duration = start_time.elapsed();

    println!("Downloaded in {:?} seconds", duration.as_secs_f64());
}
