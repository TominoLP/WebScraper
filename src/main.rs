
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
    
    // get user input for deep Flag
    let mut deep = String::new();
    println!("Do you want to fetch all the links in the page? (y/n): ");
    std::io::stdin().read_line(&mut deep).unwrap();
    let deep = deep.trim();
    
    
    // check if deep is y or n
    if deep == "y" {
        fetch_all_links(url).await;
    } else if deep == "n" {
        fetch_links(url).await;
    } else {
        println!("Invalid input for deep flag");
    }
    
}

async fn fetch_links(url: String) {
    
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

async fn fetch_all_links(url: String) {
    
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
                        
                        // check if the link is valid
                        if link.starts_with("http") {
                            println!(" ---> ");
                            fetch_links(link.to_string()).await;
                            println!(" <--- ");
                            
                        }
                        
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
