use axum::{response::Html, routing::get, Router};
use cercis::prelude::*;
use feed_rs::model;
use tower_livereload::LiveReloadLayer;

mod components;
use components::Feed;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = if cfg!(debug_assertions) {
        Router::new()
            .route("/", get(root))
            .layer(LiveReloadLayer::new())
    } else {
        Router::new().route("/", get(root))
    };

    // run our app with hyper, listening globally on port 3000
    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root<'a>() -> Html<String> {
    match kitchen::refresh_feed("https://xkcd.com/rss.xml").await {
        Ok(feed) => render(feed),
        Err(error) => panic!("{error:?}"),
    }
}

fn render(feed: model::Feed) -> Html<String> {
    let title = match &feed.title {
        Some(t) => t.content.clone(),
        None => "Untitled".to_string(),
    };

    Html(
        rsx!(
        doctype {}
        html {
            head {
                meta { charset: "UTF-8" }
                title { "{title}" }
            }
            body {
                h1 {
                    "Caterer"
                }
                nav {
                    h2 {
                        "Feeds"
                    }
                    "{title}"
                }
                Feed {
                    feed: feed
                }
            }
        })
        .render(),
    )
}
