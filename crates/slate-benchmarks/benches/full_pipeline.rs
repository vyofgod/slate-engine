//! Full Pipeline Integration Benchmarks

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use slate_kernel::Kernel;
use slate_dispatcher::WebCall;
use slate_ais::NodeId;

fn bench_dispatcher(c: &mut Criterion) {
    let mut group = c.benchmark_group("dispatcher");
    
    group.bench_function("create_element", |b| {
        b.iter(|| {
            let call = WebCall::CreateElement {
                node: black_box(NodeId(1)),
                tag: black_box("div"),
            };
            black_box(slate_dispatcher::dispatch(call).unwrap());
        });
    });
    
    group.bench_function("inline_style", |b| {
        b.iter(|| {
            let call = WebCall::SetInlineStyle {
                node: black_box(NodeId(1)),
                css: black_box("width:200px;height:100px;background:red"),
            };
            black_box(slate_dispatcher::dispatch(call).unwrap());
        });
    });
    
    group.finish();
}

fn bench_kernel(c: &mut Criterion) {
    let mut group = c.benchmark_group("kernel");
    
    group.bench_function("submit_single", |b| {
        let mut kernel = Kernel::new();
        b.iter(|| {
            let call = WebCall::CreateElement {
                node: black_box(NodeId(1)),
                tag: black_box("div"),
            };
            black_box(kernel.submit(call).unwrap());
        });
    });
    
    group.bench_function("submit_batch_10", |b| {
        let mut kernel = Kernel::new();
        b.iter(|| {
            for i in 0..10 {
                let call = WebCall::CreateElement {
                    node: NodeId(i),
                    tag: "div",
                };
                kernel.submit(call).unwrap();
            }
        });
    });
    
    group.finish();
}

criterion_group!(benches, bench_dispatcher, bench_kernel);
criterion_main!(benches);
