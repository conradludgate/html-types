use std::{borrow::Cow, rc::Rc, sync::Arc};

use crate::fmt::{Formatter, Result};

pub trait Element {
    fn render(&self, fmt: &mut Formatter) -> Result;
}

impl<'a, E: Element + ?Sized> Element for &'a E {
    fn render(&self, fmt: &mut Formatter) -> Result {
        E::render(self, fmt)
    }
}

impl<E: Element + ?Sized> Element for Box<E> {
    fn render(&self, fmt: &mut Formatter) -> Result {
        E::render(self, fmt)
    }
}

impl<E: Element + Clone + ?Sized> Element for Cow<'_, E> {
    fn render(&self, fmt: &mut Formatter) -> Result {
        E::render(self, fmt)
    }
}

impl<E: Element + ?Sized> Element for Rc<E> {
    fn render(&self, fmt: &mut Formatter) -> Result {
        E::render(self, fmt)
    }
}

impl<E: Element + ?Sized> Element for Arc<E> {
    fn render(&self, fmt: &mut Formatter) -> Result {
        E::render(self, fmt)
    }
}
