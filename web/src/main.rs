#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    kitchen::parse("https://xkcd.com/rss.xml").await
}
