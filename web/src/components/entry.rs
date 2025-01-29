use cercis::prelude::*;
use feed_rs::model;

#[component]
pub fn Entry(entry: model::Entry) -> Element {
    let content = format!("{entry:?}");

    let title = match entry.title {
        Some(ref t) => rsx!(
            h3 {"{t.content} : Caterer"}
            p { "{content}" }
        ),
        _ => rsx!(h3 {"Untitled"}),
    };
    title
}
