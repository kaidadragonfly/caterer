use cercis::prelude::*;
use feed_rs::model;

use crate::components::Entry;

#[component]
pub fn Feed(feed: model::Feed) -> Element {
    let href = match &feed.links[..] {
        [l] | [l, ..] => Some(l.clone().href),
        _ => None,
    };
    let description = match feed.description {
        Some(ref t) => match href {
            Some(href) => rsx!(p {
                a {
                    href: "{href}",

                    "{t.content}"
                }
            }),
            None => rsx!(p { "{t.content}" }),
        },
        _ => rsx!(),
    };

    rsx!(
    section {
        h2 {
            "FEED title!"
        }
        description
            main {
                class: "feed",

                for entry in feed.entries.clone() {
                    Entry { entry: entry }
                }
            }
    })
}
