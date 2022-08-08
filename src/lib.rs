pub use element::Element;
pub use element_list::ElementList;
use fmt::{ElementExt, Formatter, Result};

mod element;
mod element_list;
pub mod fmt;

pub struct Body<C> {
    children: C,
}
pub struct Div<C> {
    children: C,
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

impl<C: ElementList> Element for Body<C> {
    fn render(&self, fmt: &mut Formatter) -> Result {
        self.children.render(fmt.start("body")?).map(|_| ())
    }
}

impl<C: ElementList> Element for Div<C> {
    fn render(&self, fmt: &mut Formatter) -> Result {
        self.children.render(fmt.start("div")?).map(|_| ())
    }
}

fn demo_template(name: &str) -> impl Element + '_ {
    Body {
        children: (
            Div {
                children: ["title", name],
            },
            Div {
                children: ["foo", "bar"],
            },
            Div { children: () },
            "baz",
        ),
    }
}

#[test]
fn test() {
    println!("{}", demo_template("hello").render_to_string().unwrap())
}
