use crate::components::util::ContentOr;
use cercis::prelude::*;
use feed_rs::model;

#[component]
pub fn Entry(entry: model::Entry) -> Element {
    let title = entry.title.content_or("Untitled");
    let href = entry.links.first().map(|l| l.href.as_str());
    let body = entry.summary.content_or("(empty)");
    rsx!(
        h3 {
            if let Some(href) = href {
                a { href: "{href}", "{title}"}
            } else {
                "{title}"
            }
        }

        p { "ID: {entry.id}" }

        div { r"{body}" }
    )
}
