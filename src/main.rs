
use reqwest::Client;
use reqwest::header::USER_AGENT;
use select::document::Document;



#[tokio::main]
async fn main() {
    
    // Replace the URL with the target webpage
    let url = "https://www.google.de";

    // Make an HTTP GET request
    let client = Client::new();
    let response = client.get(url).header(USER_AGENT, "reqwest").send().await;

    match response {
        Ok(response) => {
            if response.status().is_success() {
                // Parse the HTML content
                let body = response.text().await.unwrap();
                let document = Document::from(body.as_str());

                // Extract specific information (e.g., headlines or links)
                for node in document.find(select::predicate::Name("a")) {
                    // You can customize this based on the HTML structure
                    if let Some(link) = node.attr("href") {
                        println!("Link: {}", link);
                    }
                }
            } else {
                println!("Error: {}", response.status());
            }
        }
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}