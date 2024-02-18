// Import the reqwest crate
use reqwest::Error;

// Define a function to make an HTTP GET request
async fn make_request() -> Result<(), Error> {
    // Specify the URL you want to make a GET request to
    let url = "https://jsonplaceholder.typicode.com/posts/1";

    // Make the GET request and await the response
    let response = reqwest::get(url).await?;

    // Check if the request was successful
    if response.status().is_success() {
        // Get the response body as text
        let body = response.text().await?;
        println!("Response body: {}", body);
    } else {
        println!("Request failed with status: {}", response.status());
    }

    Ok(())
}

// Define the main function
#[tokio::main]
async fn main() {
    // Call the make_request function and handle any errors
    if let Err(e) = make_request().await {
        eprintln!("Error: {}", e);
    }
}
