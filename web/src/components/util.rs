use feed_rs::model;

pub trait ContentOr {
    fn content_or<'a>(&'a self, or: &'a str) -> &'a str;
}

impl ContentOr for Option<model::Text> {
    fn content_or<'a>(&'a self, or: &'a str) -> &'a str {
        match self {
            Some(ref t) => t.content.as_ref(),
            _ => or,
        }
    }
}
