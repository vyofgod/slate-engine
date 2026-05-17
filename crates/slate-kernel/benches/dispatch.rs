//! Nanosecond-latency bench of the Dispatcher hot path.
//!
//! Run with: `cargo bench -p slate-kernel`

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use slate_ais::NodeId;
use slate_dispatcher::{dispatch, WebCall};

fn bench_create_element(c: &mut Criterion) {
    c.bench_function("dispatch/create_element", |b| {
        b.iter(|| {
            let stream = dispatch(WebCall::CreateElement {
                node: black_box(NodeId(1)),
                tag:  black_box("div"),
            })
            .unwrap();
            black_box(stream);
        });
    });
}

fn bench_inline_style(c: &mut Criterion) {
    c.bench_function("dispatch/inline_style", |b| {
        b.iter(|| {
            let stream = dispatch(WebCall::SetInlineStyle {
                node: black_box(NodeId(1)),
                css:  black_box("width:200px;height:100px;background:red;border-width:2"),
            })
            .unwrap();
            black_box(stream);
        });
    });
}

fn bench_append_child(c: &mut Criterion) {
    c.bench_function("dispatch/append_child", |b| {
        b.iter(|| {
            let stream = dispatch(WebCall::AppendChild {
                parent: black_box(NodeId(1)),
                child:  black_box(NodeId(2)),
                index:  black_box(0),
            })
            .unwrap();
            black_box(stream);
        });
    });
}

criterion_group!(
    benches,
    bench_create_element,
    bench_inline_style,
    bench_append_child
);
criterion_main!(benches);
