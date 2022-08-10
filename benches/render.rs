use criterion::{black_box, criterion_group, criterion_main, Criterion};
use html_types::{prelude::*, Body, Div};
use typed_html::{dom::DOMTree, html, text};

fn demo_template1(name: &str) -> String {
    Body.children((
        Div.children(["title", name]),
        Div.children(["foo", "bar"]),
        Div.end(),
        "baz",
    ))
    .render_to_string()
    .unwrap()
}

fn demo_template2(name: &str) -> String {
    let doc: DOMTree<String> = html!(
        <body>
            <div>
                "title"
                { text!(name) }
            </div>
            <div>
                "foo"
                "bar"
            </div>
            <div />
            "baz"
        </body>
    );
    doc.to_string()
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("html-types", |b| {
        b.iter(|| demo_template1(black_box("lorem ipsum blah blah blah")))
    });
    c.bench_function("typed-html", |b| {
        b.iter(|| demo_template2(black_box("lorem ipsum blah blah blah")))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
