use crate::fmt::{Formatter, Result};

pub trait Element {
    fn render(&self, fmt: &mut Formatter) -> Result;
}
impl<'a, E: Element> Element for &'a E {
    fn render(&self, fmt: &mut Formatter) -> Result {
        E::render(self, fmt)
    }
}
impl<E: Element> Element for Box<E> {
    fn render(&self, fmt: &mut Formatter) -> Result {
        E::render(self, fmt)
    }
}
