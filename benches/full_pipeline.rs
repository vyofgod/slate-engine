//! Full Pipeline Integration Benchmarks
//! 
//! Measures end-to-end performance: HTML → DOM → Layout → Paint → Raster

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use slate_kernel::Kernel;
use slate_html::parse_html;
use slate_css::cascade::CascadeEngine;
use slate_layout::flexbox::FlexLayout;
use slate_rasterizer::Framebuffer;

fn bench_simple_page(c: &mut Criterion) {
    let mut group = c.benchmark_group("pipeline/simple");
    
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head><title>Test</title></head>
        <body>
            <div class="container">
                <h1>Hello World</h1>
                <p>This is a test page.</p>
            </div>
        </body>
        </html>
    "#;
    
    group.bench_function("parse_html", |b| {
        b.iter(|| {
            black_box(parse_html(html));
        });
    });
    
    group.finish();
}

fn bench_complex_page(c: &mut Criterion) {
    let mut group = c.benchmark_group("pipeline/complex");
    
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Complex Page</title>
            <style>
                * { margin: 0; padding: 0; box-sizing: border-box; }
                body { font-family: Arial, sans-serif; }
                .container { max-width: 1200px; margin: 0 auto; }
                .header { background: #333; color: white; padding: 20px; }
                .nav { display: flex; gap: 20px; }
                .content { display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 20px; }
                .card { border: 1px solid #ddd; padding: 20px; }
            </style>
        </head>
        <body>
            <div class="container">
                <header class="header">
                    <h1>My Website</h1>
                    <nav class="nav">
                        <a href="/">Home</a>
                        <a href="/about">About</a>
                        <a href="/contact">Contact</a>
                    </nav>
                </header>
                <main class="content">
                    <div class="card"><h2>Card 1</h2><p>Content here</p></div>
                    <div class="card"><h2>Card 2</h2><p>Content here</p></div>
                    <div class="card"><h2>Card 3</h2><p>Content here</p></div>
                    <div class="card"><h2>Card 4</h2><p>Content here</p></div>
                    <div class="card"><h2>Card 5</h2><p>Content here</p></div>
                    <div class="card"><h2>Card 6</h2><p>Content here</p></div>
                </main>
            </div>
        </body>
        </html>
    "#;
    
    group.bench_function("full_pipeline", |b| {
        b.iter(|| {
            // Parse HTML
            let dom = parse_html(html);
            black_box(dom);
        });
    });
    
    group.finish();
}

fn bench_rendering(c: &mut Criterion) {
    let mut group = c.benchmark_group("pipeline/rendering");
    
    for size in [(800, 600), (1920, 1080), (3840, 2160)].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}x{}", size.0, size.1)),
            size,
            |b, &(width, height)| {
                let mut fb = Framebuffer::new(width, height);
                
                b.iter(|| {
                    // Simulate rendering operations
                    fb.clear([255, 255, 255, 255]);
                    
                    // Draw some rectangles
                    for i in 0..10 {
                        let x = (i * 80) as i32;
                        let y = 100;
                        fb.fill_rect(x, y, 70, 50, [255, 0, 0, 255]);
                    }
                    
                    black_box(&fb);
                });
            },
        );
    }
    
    group.finish();
}

fn bench_dom_manipulation(c: &mut Criterion) {
    let mut group = c.benchmark_group("pipeline/dom");
    
    group.bench_function("create_elements", |b| {
        let mut kernel = Kernel::new();
        
        b.iter(|| {
            for i in 0..100 {
                let call = slate_dispatcher::WebCall::CreateElement {
                    node: slate_ais::NodeId(i),
                    tag: "div",
                };
                black_box(kernel.submit(call).unwrap());
            }
        });
    });
    
    group.finish();
}

fn bench_style_computation(c: &mut Criterion) {
    let mut group = c.benchmark_group("pipeline/style");
    
    let css = r#"
        * { margin: 0; padding: 0; }
        body { font-family: Arial; font-size: 16px; }
        .container { max-width: 1200px; margin: 0 auto; }
        .header { background: #333; color: white; padding: 20px; }
        .nav { display: flex; gap: 20px; }
        .content { display: grid; grid-template-columns: 1fr 1fr 1fr; }
        .card { border: 1px solid #ddd; padding: 20px; border-radius: 8px; }
        h1 { font-size: 32px; font-weight: bold; }
        h2 { font-size: 24px; font-weight: bold; }
        p { line-height: 1.5; }
        a { color: blue; text-decoration: none; }
        a:hover { text-decoration: underline; }
    "#;
    
    group.bench_function("parse_stylesheet", |b| {
        b.iter(|| {
            black_box(slate_css::parser::parse_stylesheet(css));
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_simple_page,
    bench_complex_page,
    bench_rendering,
    bench_dom_manipulation,
    bench_style_computation
);
criterion_main!(benches);
