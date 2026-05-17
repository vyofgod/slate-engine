//! CSS Selector Matching Benchmarks

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use slate_css::selector::{Selector, SelectorMatcher, Specificity};
use slate_css::cascade::CascadeEngine;
use slate_dom::{Document, Element, ElementData};
use std::sync::Arc;

fn create_test_element(tag: &str, classes: Vec<&str>, id: Option<&str>) -> Element {
    let mut attrs = std::collections::HashMap::new();
    
    if !classes.is_empty() {
        attrs.insert("class".to_string(), classes.join(" "));
    }
    
    if let Some(id_val) = id {
        attrs.insert("id".to_string(), id_val.to_string());
    }
    
    Element::new(tag.to_string(), attrs)
}

fn bench_simple_selectors(c: &mut Criterion) {
    let mut group = c.benchmark_group("selector_matching/simple");
    
    let element = create_test_element("div", vec!["container", "main"], Some("app"));
    
    // Type selector
    group.bench_function("type_selector", |b| {
        let selector = Selector::Type("div".to_string());
        b.iter(|| {
            black_box(SelectorMatcher::matches(&selector, &element));
        });
    });
    
    // Class selector
    group.bench_function("class_selector", |b| {
        let selector = Selector::Class("container".to_string());
        b.iter(|| {
            black_box(SelectorMatcher::matches(&selector, &element));
        });
    });
    
    // ID selector
    group.bench_function("id_selector", |b| {
        let selector = Selector::Id("app".to_string());
        b.iter(|| {
            black_box(SelectorMatcher::matches(&selector, &element));
        });
    });
    
    group.finish();
}

fn bench_complex_selectors(c: &mut Criterion) {
    let mut group = c.benchmark_group("selector_matching/complex");
    
    let element = create_test_element("button", vec!["btn", "btn-primary"], None);
    
    // Compound selector: button.btn.btn-primary
    group.bench_function("compound_selector", |b| {
        let selector = Selector::Compound(vec![
            Selector::Type("button".to_string()),
            Selector::Class("btn".to_string()),
            Selector::Class("btn-primary".to_string()),
        ]);
        b.iter(|| {
            black_box(SelectorMatcher::matches(&selector, &element));
        });
    });
    
    group.finish();
}

fn bench_specificity_calculation(c: &mut Criterion) {
    let mut group = c.benchmark_group("selector_matching/specificity");
    
    // Simple selector
    group.bench_function("simple", |b| {
        let selector = Selector::Class("container".to_string());
        b.iter(|| {
            black_box(selector.specificity());
        });
    });
    
    // Complex selector
    group.bench_function("complex", |b| {
        let selector = Selector::Compound(vec![
            Selector::Type("div".to_string()),
            Selector::Id("main".to_string()),
            Selector::Class("container".to_string()),
            Selector::Class("active".to_string()),
        ]);
        b.iter(|| {
            black_box(selector.specificity());
        });
    });
    
    group.finish();
}

fn bench_cascade_resolution(c: &mut Criterion) {
    let mut group = c.benchmark_group("cascade/resolution");
    
    let element = create_test_element("div", vec!["box"], Some("main"));
    
    // Small stylesheet
    group.bench_function("small_stylesheet", |b| {
        let engine = CascadeEngine::new();
        let css = r#"
            div { color: black; }
            .box { width: 100px; height: 100px; }
            #main { background: blue; }
        "#;
        
        b.iter(|| {
            black_box(engine.compute_styles(&element, css));
        });
    });
    
    // Medium stylesheet
    group.bench_function("medium_stylesheet", |b| {
        let engine = CascadeEngine::new();
        let css = r#"
            * { margin: 0; padding: 0; }
            body { font-family: Arial; }
            div { display: block; }
            .box { width: 100px; height: 100px; border: 1px solid; }
            .container { max-width: 1200px; }
            #main { background: blue; color: white; }
            div.box { padding: 10px; }
            #main.box { margin: 20px; }
        "#;
        
        b.iter(|| {
            black_box(engine.compute_styles(&element, css));
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_simple_selectors,
    bench_complex_selectors,
    bench_specificity_calculation,
    bench_cascade_resolution
);
criterion_main!(benches);
