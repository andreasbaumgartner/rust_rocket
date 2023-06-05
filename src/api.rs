use reqwest::Error;


const api_key: = env!("API_KEY")


#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!("https://api.github.com/repos/{api_key}/",);
    println!("{}", request_url);
    let response = reqwest::get(&request_url).await?;

    Ok(())
}

