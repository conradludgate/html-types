use std::fmt::Write;

use crate::{Element, ElementList};

pub struct Formatter<'a> {
    depth: usize,
    pub(crate) buf: &'a mut (dyn Write + 'a),
}

pub type Result<T = (), E = std::fmt::Error> = std::result::Result<T, E>;

impl<'a> Formatter<'a> {
    pub(crate) fn render_tabs(&mut self) -> Result {
        for _ in 0..self.depth {
            self.buf.write_str("  ")?;
        }
        Ok(())
    }
    pub(crate) fn write_str(&mut self, s: &str) -> Result {
        self.render_tabs()?;
        self.buf.write_str(s)?;
        self.buf.write_char('\n')
    }
    pub fn start(&mut self, element: &'static str) -> Result<ElementFmt<'_, 'a>> {
        self.render_tabs()?;
        write!(self.buf, "<{element}")?;
        self.depth += 1;
        Ok(ElementFmt {
            fmt: self,
            element,
            children: 0,
        })
    }
}

#[must_use]
pub struct ElementFmt<'a, 'b> {
    fmt: &'a mut Formatter<'b>,
    element: &'static str,
    children: usize,
}

impl ElementFmt<'_, '_> {
    pub fn render_child(mut self, child: &dyn Element) -> Result<Self> {
        if self.children == 0 {
            self.fmt.buf.write_str(">\n").unwrap();
        }
        self.children += 1;
        child.render(self.fmt)?;
        Ok(self)
    }

    #[inline]
    pub fn render_children(self, children: &impl ElementList) -> Result<Self> {
        children.render(self)
    }

    pub fn close(self) -> Result {
        self.fmt.depth -= 1;
        // close the tag approriately
        if self.children > 0 {
            self.fmt.render_tabs()?;
            self.fmt.buf.write_str("</")?;
            self.fmt.buf.write_str(self.element)?;
            self.fmt.buf.write_str(">\n")
        } else {
            self.fmt.buf.write_str("/>\n")
        }
    }
}

pub trait ElementExt: Element {
    fn render_to_string(&self) -> Result<String> {
        let mut buf = String::new();
        self.render(&mut Formatter {
            depth: 0,
            buf: &mut buf,
        })?;
        Ok(buf)
    }
}
impl<E: Element> ElementExt for E {}
