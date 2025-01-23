pub async fn parse(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(url)
        .await?
        .bytes()
        .await?;
    let feed = feed_rs::parser::parse(resp.as_ref()).unwrap();
    println!("{feed:#?}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
