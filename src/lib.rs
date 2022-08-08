use std::{borrow::Cow, fmt::Write, ops::Deref};

pub type DynList<'a, T> = DynElem<'a, [DynElem<'a, T>]>;
pub enum DynElem<'a, T: ?Sized + 'a> {
    Owned(Box<T>),
    Borrowed(&'a T),
}

impl<'a, T: ?Sized + 'a> Deref for DynElem<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            DynElem::Owned(o) => o,
            DynElem::Borrowed(b) => b,
        }
    }
}

macro_rules! o {
    ($b:expr) => {
        $crate::DynElem::Owned::<'_, _> { 0: Box::new($b) }
    };
}

macro_rules! b {
    ($b:expr) => {
        $crate::DynElem::Borrowed::<'_, _> { 0: &$b }
    };
}

pub struct Formatter<'a> {
    depth: usize,
    buf: &'a mut (dyn Write + 'a),
}

type Result<T = (), E = std::fmt::Error> = std::result::Result<T, E>;

impl<'a> Formatter<'a> {
    fn render_tabs(&mut self) -> Result {
        for _ in 0..self.depth {
            self.buf.write_str("  ")?;
        }
        Ok(())
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

pub struct ElementFmt<'a, 'b> {
    fmt: &'a mut Formatter<'b>,
    element: &'static str,
    children: usize,
}

impl Drop for ElementFmt<'_, '_> {
    fn drop(&mut self) {
        self.fmt.depth -= 1;
        // close the tag approriately
        if self.children > 0 {
            self.fmt.render_tabs().unwrap();
            self.fmt.buf.write_str("</").unwrap();
            self.fmt.buf.write_str(self.element).unwrap();
            self.fmt.buf.write_str(">\n").unwrap();
        } else {
            self.fmt.buf.write_str("/>\n").unwrap();
        }
    }
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

pub trait Element {
    fn render(&self, fmt: &mut Formatter) -> Result;
}

pub struct Body<'a> {
    children: DynList<'a, dyn Element + 'a>,
}
pub struct Div<'a> {
    children: DynList<'a, dyn Element + 'a>,
}
pub struct Text<'a>(pub Cow<'a, str>);

impl<T: AsRef<str> + ?Sized> Element for T {
    fn render(&self, fmt: &mut Formatter) -> Result {
        fmt.render_tabs()?;
        fmt.buf.write_str(self.as_ref())?;
        fmt.buf.write_char('\n')
    }
}

impl Element for Body<'_> {
    fn render(&self, fmt: &mut Formatter) -> Result {
        self.children
            .iter()
            .try_fold(fmt.start("body")?, |f, e| f.render_child(&**e))?;
        Ok(())
    }
}

impl Element for Div<'_> {
    fn render(&self, fmt: &mut Formatter) -> Result {
        self.children
            .iter()
            .try_fold(fmt.start("div")?, |f, e| f.render_child(&**e))?;
        Ok(())
    }
}

fn foo(name: &str) -> String {
    Body {
        children: b!([
            b!(Div {
                children: b!([b!("title"), b!(name)])
            }),
            b!(Div {
                children: b!([b!("foo"), b!("bar")])
            }),
            b!(Div { children: b!([]) }),
            b!("baz"),
        ]),
    }
    .render_to_string()
    .unwrap()
}

#[test]
fn test() {
    println!("{}", foo("hello"))
}
