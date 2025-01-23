#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://xkcd.com/rss.xml")
        .await?
        .bytes()
        .await?;
    let feed = feed_rs::parser::parse(resp.as_ref()).unwrap();
    println!("{feed:#?}");
    Ok(())
}
