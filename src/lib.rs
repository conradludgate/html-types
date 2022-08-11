pub use element::Element;
pub use element_list::ElementList;
use fmt::{Formatter, Result};
use tag::{Parent, Tag};

pub mod prelude {
    pub use crate::fmt::ElementExt;
    pub use crate::tag::{ParentExt, TagExt};
}

mod element;
mod element_list;
pub mod fmt;
pub mod tag;
pub mod attributes;

pub struct Body;
pub struct Div;

impl Tag for Div {
    const NAME: &'static str = "div";
}
impl Parent for Div {}
impl Tag for Body {
    const NAME: &'static str = "body";
}
impl Parent for Body {}

impl Element for str {
    fn render(&self, fmt: &mut Formatter) -> Result {
        fmt.write_str(self)
    }
}

impl Element for String {
    fn render(&self, fmt: &mut Formatter) -> Result {
        fmt.write_str(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    use insta::assert_snapshot;

    fn demo_template(name: &str) -> impl Element + '_ {
        Body.children((
            Div.children(["title", name]),
            Div.children(["foo", "bar"]),
            Div.end(),
            "baz",
        ))
    }

    #[test]
    fn demo() {
        assert_snapshot!(demo_template("hello").render_to_string().unwrap());
    }
}
