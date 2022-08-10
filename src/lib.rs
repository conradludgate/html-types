pub use element::Element;
pub use element_list::ElementList;
use fmt::{ElementFmt, Formatter, Result};

mod element;
mod element_list;
pub mod fmt;

pub struct Body;
pub struct Div;

pub trait Tag {
    const NAME: &'static str;

    fn fmt<'a, 'b>(&self, fmt: &'a mut Formatter<'b>) -> Result<ElementFmt<'a, 'b>> {
        fmt.start(Self::NAME)
    }
}
pub trait Parent: Tag {}

impl Tag for Div {
    const NAME: &'static str = "div";
}
impl Parent for Div {}
impl Tag for Body {
    const NAME: &'static str = "body";
}
impl Parent for Body {}

pub struct WithChildren<T, C> {
    tag: T,
    children: C,
}

pub trait ChildrenExt: Parent + Sized {
    fn children<C: ElementList>(self, children: C) -> WithChildren<Self, C> {
        WithChildren {
            tag: self,
            children,
        }
    }
}
impl<P: Parent> ChildrenExt for P {}

pub struct Closed<T>(T);
pub trait TagExt: Tag + Sized {
    fn end(self) -> Closed<Self> {
        Closed(self)
    }
}
impl<T: Tag> TagExt for T {}

impl<T: Tag> Element for Closed<T> {
    fn render(&self, fmt: &mut Formatter) -> Result {
        self.0.fmt(fmt).map(|_| ())
    }
}

impl<P: Parent, C: ElementList> Element for WithChildren<P, C> {
    fn render(&self, fmt: &mut Formatter) -> Result {
        self.children.render(self.tag.fmt(fmt)?).map(|_| ())
    }
}

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
    use crate::fmt::ElementExt;

    use super::*;
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

