//! Flexbox Layout Benchmarks

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use slate_layout::flexbox::{FlexLayout, FlexDirection, JustifyContent, AlignItems};
use slate_layout::{Constraints, LayoutNode, Size};
use slate_ais::NodeId;

fn create_flex_container(child_count: usize) -> LayoutNode {
    let mut container = LayoutNode::new(NodeId(0));
    
    for i in 0..child_count {
        let mut child = LayoutNode::new(NodeId(i as u32 + 1));
        child.set_size(Size::new(100.0, 50.0));
        container.add_child(child);
    }
    
    container
}

fn bench_flex_row_layout(c: &mut Criterion) {
    let mut group = c.benchmark_group("flexbox/row");
    
    for child_count in [5, 10, 20, 50].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(child_count),
            child_count,
            |b, &count| {
                let mut container = create_flex_container(count);
                let constraints = Constraints::tight(Size::new(800.0, 600.0));
                
                b.iter(|| {
                    let layout = FlexLayout::new(
                        FlexDirection::Row,
                        JustifyContent::FlexStart,
                        AlignItems::Stretch,
                    );
                    black_box(layout.layout(&mut container, constraints));
                });
            },
        );
    }
    
    group.finish();
}

fn bench_flex_column_layout(c: &mut Criterion) {
    let mut group = c.benchmark_group("flexbox/column");
    
    for child_count in [5, 10, 20, 50].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(child_count),
            child_count,
            |b, &count| {
                let mut container = create_flex_container(count);
                let constraints = Constraints::tight(Size::new(800.0, 600.0));
                
                b.iter(|| {
                    let layout = FlexLayout::new(
                        FlexDirection::Column,
                        JustifyContent::FlexStart,
                        AlignItems::Stretch,
                    );
                    black_box(layout.layout(&mut container, constraints));
                });
            },
        );
    }
    
    group.finish();
}

fn bench_flex_wrap(c: &mut Criterion) {
    let mut group = c.benchmark_group("flexbox/wrap");
    
    let mut container = create_flex_container(30);
    let constraints = Constraints::tight(Size::new(500.0, 600.0));
    
    group.bench_function("wrap_enabled", |b| {
        b.iter(|| {
            let layout = FlexLayout::new(
                FlexDirection::Row,
                JustifyContent::FlexStart,
                AlignItems::Stretch,
            );
            black_box(layout.layout(&mut container, constraints));
        });
    });
    
    group.finish();
}

fn bench_justify_content(c: &mut Criterion) {
    let mut group = c.benchmark_group("flexbox/justify");
    
    let mut container = create_flex_container(10);
    let constraints = Constraints::tight(Size::new(800.0, 600.0));
    
    let justify_modes = vec![
        ("flex_start", JustifyContent::FlexStart),
        ("flex_end", JustifyContent::FlexEnd),
        ("center", JustifyContent::Center),
        ("space_between", JustifyContent::SpaceBetween),
        ("space_around", JustifyContent::SpaceAround),
        ("space_evenly", JustifyContent::SpaceEvenly),
    ];
    
    for (name, mode) in justify_modes {
        group.bench_function(name, |b| {
            b.iter(|| {
                let layout = FlexLayout::new(
                    FlexDirection::Row,
                    mode,
                    AlignItems::Stretch,
                );
                black_box(layout.layout(&mut container, constraints));
            });
        });
    }
    
    group.finish();
}

fn bench_align_items(c: &mut Criterion) {
    let mut group = c.benchmark_group("flexbox/align");
    
    let mut container = create_flex_container(10);
    let constraints = Constraints::tight(Size::new(800.0, 600.0));
    
    let align_modes = vec![
        ("flex_start", AlignItems::FlexStart),
        ("flex_end", AlignItems::FlexEnd),
        ("center", AlignItems::Center),
        ("baseline", AlignItems::Baseline),
        ("stretch", AlignItems::Stretch),
    ];
    
    for (name, mode) in align_modes {
        group.bench_function(name, |b| {
            b.iter(|| {
                let layout = FlexLayout::new(
                    FlexDirection::Row,
                    JustifyContent::FlexStart,
                    mode,
                );
                black_box(layout.layout(&mut container, constraints));
            });
        });
    }
    
    group.finish();
}

fn bench_nested_flex(c: &mut Criterion) {
    let mut group = c.benchmark_group("flexbox/nested");
    
    // Create nested flex containers
    let mut root = LayoutNode::new(NodeId(0));
    
    for i in 0..5 {
        let mut container = LayoutNode::new(NodeId(i * 10));
        
        for j in 0..5 {
            let mut child = LayoutNode::new(NodeId(i * 10 + j + 1));
            child.set_size(Size::new(50.0, 50.0));
            container.add_child(child);
        }
        
        root.add_child(container);
    }
    
    let constraints = Constraints::tight(Size::new(800.0, 600.0));
    
    group.bench_function("2_levels", |b| {
        b.iter(|| {
            let layout = FlexLayout::new(
                FlexDirection::Column,
                JustifyContent::FlexStart,
                AlignItems::Stretch,
            );
            black_box(layout.layout(&mut root, constraints));
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_flex_row_layout,
    bench_flex_column_layout,
    bench_flex_wrap,
    bench_justify_content,
    bench_align_items,
    bench_nested_flex
);
criterion_main!(benches);
