use cercis::prelude::*;
use feed_rs::model;

#[component]
pub fn Feed(feed: model::Feed) -> Element {
    let f = format!("{feed:#?}");
    rsx!(p {
        class: "feed",
        "{f}"
    })
}
