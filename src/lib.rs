use std::{borrow::Cow, fmt::Write, ops::Deref};

// pub enum DynList<'a, T: ?Sized> {
//     Owned(Box<[Box<T>]>),
//     Borrowed(&'a [&'a T]),
// }
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

pub trait ElementExt: Element {
    fn render_to_string(&self) -> Result<String, std::fmt::Error> {
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
    fn render(&self, fmt: &mut Formatter) -> std::fmt::Result;
}

#[macro_export]
macro_rules! dyn_list {
    ($($v:expr),* $(,)?) => {
        [$($v),*]
    };
}

// #[macro_export]
// #[doc(hidden)]
// macro_rules! inner_dyn_list {
//     ($($v:expr),* ; box($b:expr) $(, $r:expr)*) => {
//         inner_dyn_list!(
//             $($v,)*
//             $crate::owned!($b);
//             $($r),*
//         )
//     };
//     ($($v:expr),* ; $b:expr $(, $r:expr)*) => {
//         inner_dyn_list!(
//             $($v,)*
//             borrow!($b);
//             $($r),*
//         )
//     };
//     ($($v:expr),* ;) => {
//         [$($v),*]
//     };
// }

// #[macro_export]
// macro_rules! box_dyn_list {
//     ($($v:expr),* $(,)?) => {
//         $crate::DynList::Owned(
//             vec![$(
//                 Box::new($v) as Box<_>
//             ),*].into_boxed_slice()
//         )
//     };
// }

pub struct Body<'a> {
    children: DynList<'a, dyn Element + 'a>,
}
pub struct Div<'a> {
    children: DynList<'a, dyn Element + 'a>,
}
pub struct Text<'a>(pub Cow<'a, str>);

impl<T: AsRef<str> + ?Sized> Element for T {
    fn render(&self, fmt: &mut Formatter) -> std::fmt::Result {
        for _ in 0..fmt.depth {
            fmt.buf.write_char('\t')?;
        }
        writeln!(fmt.buf, "{}", self.as_ref())
    }
}

impl Element for Body<'_> {
    fn render(&self, fmt: &mut Formatter) -> std::fmt::Result {
        for _ in 0..fmt.depth {
            fmt.buf.write_char('\t')?;
        }
        fmt.buf.write_str("<body>\n")?;
        fmt.depth += 1;
        for child in &*self.children {
            child.render(&mut *fmt)?;
        }
        fmt.depth -= 1;
        for _ in 0..fmt.depth {
            fmt.buf.write_char('\t')?;
        }
        fmt.buf.write_str("</body>\n")
    }
}
impl Element for Div<'_> {
    fn render(&self, fmt: &mut Formatter) -> std::fmt::Result {
        for _ in 0..fmt.depth {
            fmt.buf.write_char('\t')?;
        }
        fmt.buf.write_str("<div>\n")?;
        fmt.depth += 1;
        for child in &*self.children {
            child.render(&mut *fmt)?;
        }
        fmt.depth -= 1;
        for _ in 0..fmt.depth {
            fmt.buf.write_char('\t')?;
        }
        fmt.buf.write_str("</div>\n")
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

// fn name1(name: &str) -> DynElem<'_, dyn Element> {
//     // borrow!(name)
//     DynElem::Owned { 0: Box::new(name) }
// }

// fn name(name: &str) -> DynList<'_, dyn Element> {
//     DynList::Borrowed { 0: &[name1(name)] }
// }
