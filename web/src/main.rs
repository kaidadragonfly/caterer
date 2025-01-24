use cercis::prelude::*;
use feed_rs::model::Feed;
use axum::{
    response::Html,
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(root));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root<'a>() -> Html<String> {
    match kitchen::refresh_feed("https://xkcd.com/rss.xml").await {
        Ok(feed) => render(feed),
        Err(error) => panic!("{error:?}")
    }
}

fn render(feed: Feed) -> Html<String> {
    let f = format!("{feed:#?}");
    Html(rsx!(p {"{f}"}).render())
}
