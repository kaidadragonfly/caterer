use std::error::Error;

use feed_rs::{
    parser,
    model::Feed,
};


pub async fn refresh_feed(url: &str) -> Result<Feed, Box<dyn Error>> {
    let resp = reqwest::get(url)
        .await?
        .bytes()
        .await?;
    Ok(parser::parse(resp.as_ref())?)
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
