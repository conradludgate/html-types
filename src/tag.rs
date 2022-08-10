use crate::{
    fmt::{ElementFmt, Formatter},
    Element, ElementList, Result,
};

/// A basic tag element - eg `<tag> ... </tag>` or `<tag />`.
pub trait Tag {
    const NAME: &'static str;

    fn open_and_close(&self, fmt: &mut Formatter) -> Result {
        fmt.open_and_close(Self::NAME)
    }
    fn open<'a, 'b>(&self, fmt: &'a mut Formatter<'b>) -> Result<ElementFmt<'a, 'b>> {
        fmt.open(Self::NAME)
    }
}

pub trait TagExt: Tag + Sized {
    /// Closes this tag without adding any children
    fn end(self) -> Closed<Self> {
        Closed(self)
    }
}
impl<T: Tag> TagExt for T {}

/// A tag element that can have children.
pub trait Parent: Tag {}

pub trait ParentExt: Parent + Sized {
    /// Adds children to this tag
    fn children<C: ElementList>(self, children: C) -> WithChildren<Self, C> {
        WithChildren {
            tag: self,
            children,
        }
    }
}
impl<P: Parent> ParentExt for P {}

pub struct Closed<T>(T);
impl<T: Tag> Element for Closed<T> {
    fn render(&self, fmt: &mut Formatter) -> Result {
        self.0.open_and_close(fmt)
    }
}

/// An [`Tag`] [`Element`] that has [children elements](`ElementList`).
///
/// T must be a [`Parent`]
pub struct WithChildren<T, C> {
    tag: T,
    children: C,
}
impl<P: Parent, C: ElementList> Element for WithChildren<P, C> {
    fn render(&self, fmt: &mut Formatter) -> Result {
        let Self { tag, children } = self;
        tag.open(fmt)?.render_children(children)?.close()
    }
}
