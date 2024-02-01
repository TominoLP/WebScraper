
use reqwest::Client;
use reqwest::header::USER_AGENT;
use select::document::Document;



#[tokio::main]
async fn main() {
    
    // get user input for url
    let mut input = String::new();
    println!("Enter a URL: ");
    std::io::stdin().read_line(&mut input).unwrap();
    let url = input.trim();
    
    // check if url is empty
    if url.is_empty() {
        println!("URL is empty");
        return;
    }
    
    // fix url if it doesn't have http
    let mut url = url.to_string();
    if !url.starts_with("http") {
        url = format!("https://{}", url);
    }
    
    // check if url is valid using regex
    let re = regex::Regex::new(r"^https?://").unwrap();
    if !re.is_match(&url) {
        println!("Invalid URL");
        return;
    }
    

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