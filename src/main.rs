use async_recursion::async_recursion;
use futures::{stream, StreamExt};
use html::get_links;
use reqwest::Client;
use std::error::Error;

extern crate select;

mod html;


struct Crawler {
    urls_left: Vec<String>,
    websites: Vec<String>,
}

struct Website {
    host: String,
}

#[tokio::main]
async fn main() {
    let url = "https://crawler-test.com/";

    let mut cralwed_urls = vec![];
    crawl_url(url, &mut cralwed_urls , 3, 0).await;
}

#[async_recursion]
async fn crawl_url(
    url: &str,
    mut crawled_urls: &mut Vec<String>,
    max_depth: u32,
    current_dept: u32,
) -> Result<(), Box<dyn Error>> {
    let res = reqwest::get(url).await?;

    // TODO: check if mimetype
    // TODO: flatten statement
    // TODO: head requests
    // TODO: v1
    // TODO v2 test baar

    if !res.status().is_success() {
        eprintln!("{} returned status code {}", url, res.status().as_u16());
    }

    if res.status().is_success() {
        let body = res.text().await?;
        let new_urls = get_links(&body);

        if current_dept < max_depth {
            for new_url in new_urls.iter() {
                if !crawled_urls.contains(&new_url) {

                    crawled_urls.push(String::from(new_url));
                    eprintln!("{}", new_url);

                    if let Ok(mut result) =
                        crawl_url(&new_url, &mut crawled_urls, max_depth, current_dept + 1).await
                    {
                    }
                }
            }
        }
    }

    Ok(())
}
