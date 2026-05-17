// SVG Icons
const icons = {
    home: '<svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"></path></svg>',
    code: '<svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="16 18 22 12 16 6"></polyline><polyline points="8 6 2 12 8 18"></polyline></svg>',
    layers: '<svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="12 2 2 7 12 12 22 7 12 2"></polygon></svg>',
    cpu: '<svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="4" y="4" width="16" height="16" rx="2"></rect></svg>',
    zap: '<svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"></polygon></svg>',
    box: '<svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8"></path></svg>',
    activity: '<svg class="icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><polyline points="22 12 18 12 15 21 9 3 6 12 2 12"></polyline></svg>'
};

let currentLang = 'en';

const translations = {
    en: {
        title: 'Slate Engine',
        subtitle: 'A minimalist browser engine core in Rust',
        description: 'Slate is a Web Compatibility Layer, not a browser: it decomposes every high-level Web API call into a closed set of Atomic Instructions (AIS) before execution.',
        features: 'Key Features',
        quickstart: 'Quick Start',
        architecture: 'Architecture',
        archSubtitle: 'Understanding Slate\'s design philosophy'
    },
    tr: {
        title: 'Slate Engine',
        subtitle: 'Rust ile yazılmış minimalist tarayıcı motoru çekirdeği',
        description: 'Slate bir tarayıcı değil, Web Uyumluluk Katmanıdır: her üst düzey Web API çağrısını yürütmeden önce kapalı bir Atomik Talimat (AIS) kümesine ayrıştırır.',
        features: 'Temel Özellikler',
        quickstart: 'Hızlı Başlangıç',
        architecture: 'Mimari',
        archSubtitle: 'Slate\'in tasarım felsefesini anlamak'
    }
};

const content = {
    'overview': function() {
        const t = translations[currentLang];
        return `
        <div class="header">
            <h1>${icons.home} ${t.title}</h1>
            <p class="subtitle">${t.subtitle}</p>
        </div>

        <p>${t.description}</p>

        <div class="stats">
            <div class="stat-card">
                <div class="stat-number">20+</div>
                <div class="stat-label">${currentLang === 'tr' ? 'Crate' : 'Crates'}</div>
            </div>
            <div class="stat-card">
                <div class="stat-number">50K+</div>
                <div class="stat-label">${currentLang === 'tr' ? 'Kod Satırı' : 'Lines of Code'}</div>
            </div>
            <div class="stat-card">
                <div class="stat-number">82</div>
                <div class="stat-label">${currentLang === 'tr' ? 'Başarılı Test' : 'Tests Passing'}</div>
            </div>
            <div class="stat-card">
                <div class="stat-number">5</div>
                <div class="stat-label">${currentLang === 'tr' ? 'Tamamlanan Faz' : 'Phases Complete'}</div>
            </div>
        </div>

        <div class="section">
            <h2>${icons.zap} ${t.features}</h2>
            
            <h3>${currentLang === 'tr' ? 'Çekirdek Mimari' : 'Core Architecture'}</h3>
            <ul class="feature-list">
                <li><strong>Atomic Instruction Set (AIS)</strong> - ${currentLang === 'tr' ? 'Her web API çağrısı atomik talimatlara ayrıştırılır' : 'Every web API call decomposed into atomic instructions'}</li>
                <li><strong>${currentLang === 'tr' ? 'Sıfır DOM Yüzeyi' : 'Zero DOM Surface'}</strong> - ${currentLang === 'tr' ? 'JavaScript doğrudan DOM\'u değiştiremez' : 'JavaScript cannot directly mutate the DOM'}</li>
                <li><strong>${currentLang === 'tr' ? 'Arena Tahsisi' : 'Arena Allocation'}</strong> - ${currentLang === 'tr' ? 'Sayfa başına O(1) bellek sıfırlama' : 'Per-page O(1) memory reset'}</li>
                <li><strong>${currentLang === 'tr' ? 'Deterministik Durum' : 'Deterministic State'}</strong> - ${currentLang === 'tr' ? 'Anlık görüntü ve zaman yolculuğu hata ayıklama' : 'Snapshots and time-travel debugging'}</li>
            </ul>

            <h3>${currentLang === 'tr' ? 'Web Standartları' : 'Web Standards'}</h3>
            <ul class="feature-list">
                <li><strong>HTML5</strong> - ${currentLang === 'tr' ? 'Hata kurtarmalı tam ayrıştırıcı' : 'Full parser with error recovery'}</li>
                <li><strong>CSS3</strong> - ${currentLang === 'tr' ? 'Seçiciler, basamaklama, hesaplanmış stiller' : 'Selectors, cascade, computed styles'}</li>
                <li><strong>Flexbox & Grid</strong> - ${currentLang === 'tr' ? 'Modern yerleşim motorları' : 'Modern layout engines'}</li>
                <li><strong>Canvas 2D</strong> - ${currentLang === 'tr' ? 'Tam 2D çizim API\'si' : 'Full 2D drawing API'}</li>
                <li><strong>WebGL 1.0 & 2.0</strong> - ${currentLang === 'tr' ? '3D grafik desteği' : '3D graphics support'}</li>
                <li><strong>Web Workers</strong> - ${currentLang === 'tr' ? 'Çok iş parçacıklı JavaScript' : 'Multi-threaded JavaScript'}</li>
                <li><strong>WebAssembly</strong> - ${currentLang === 'tr' ? 'WASM çalışma zamanı' : 'WASM runtime'}</li>
                <li><strong>Storage APIs</strong> - localStorage, sessionStorage, IndexedDB</li>
                <li><strong>WebSocket</strong> - ${currentLang === 'tr' ? 'Gerçek zamanlı iletişim' : 'Real-time communication'}</li>
            </ul>

            <h3>${currentLang === 'tr' ? 'Render Hattı' : 'Rendering Pipeline'}</h3>
            <ul class="feature-list">
                <li><strong>${currentLang === 'tr' ? 'Metin Oluşturma' : 'Text Rendering'}</strong> - HarfBuzz ${currentLang === 'tr' ? 've FreeType ile profesyonel tipografi' : 'and FreeType for professional typography'}</li>
                <li><strong>${currentLang === 'tr' ? 'Görüntü Desteği' : 'Image Support'}</strong> - PNG, JPEG, WebP, GIF, BMP, TIFF, ICO</li>
                <li><strong>GPU ${currentLang === 'tr' ? 'Hızlandırma' : 'Acceleration'}</strong> - wgpu ${currentLang === 'tr' ? 'ile modern grafik API\'si' : 'for modern graphics API'}</li>
                <li><strong>CPU Rasterizer</strong> - ${currentLang === 'tr' ? 'Headless render için yedek' : 'Fallback for headless rendering'}</li>
            </ul>
        </div>

        <div class="section">
            <h2>${icons.code} ${t.quickstart}</h2>
            
            <h3>${currentLang === 'tr' ? 'Gereksinimler' : 'Requirements'}</h3>
            <ul class="feature-list">
                <li>Rust 1.70+</li>
                <li>Cargo</li>
                <li>C/C++ compiler (${currentLang === 'tr' ? 'FreeType ve HarfBuzz için' : 'for FreeType and HarfBuzz'})</li>
                <li>Git</li>
            </ul>

            <h3>${currentLang === 'tr' ? 'Kurulum' : 'Installation'}</h3>
            <pre><code># ${currentLang === 'tr' ? 'Depoyu klonla' : 'Clone the repository'}
git clone https://github.com/yourusername/slate-engine.git
cd slate-engine

# ${currentLang === 'tr' ? 'Release modunda derle' : 'Build in release mode'}
cargo build --release

# ${currentLang === 'tr' ? 'Demo çalıştır' : 'Run demo'}
cargo run --release --bin slate-demo</code></pre>
            
            <h3>${currentLang === 'tr' ? 'Testleri Çalıştır' : 'Run Tests'}</h3>
            <pre><code># ${currentLang === 'tr' ? 'Tüm testler' : 'All tests'}
cargo test --workspace

# ${currentLang === 'tr' ? 'Belirli bir crate' : 'Specific crate'}
cargo test -p slate-layout

# ${currentLang === 'tr' ? 'Benchmark\'lar' : 'Benchmarks'}
cargo bench</code></pre>
        </div>

        <div class="section">
            <h2>${icons.box} ${currentLang === 'tr' ? 'Proje Yapısı' : 'Project Structure'}</h2>
            <pre><code>slate-engine/
├── crates/
│   ├── slate-kernel/      # ${currentLang === 'tr' ? 'Ana orkestratör' : 'Main orchestrator'}
│   ├── slate-ais/         # ${currentLang === 'tr' ? 'Atomik talimat seti' : 'Atomic instruction set'}
│   ├── slate-dispatcher/  # WebCall → AIS ${currentLang === 'tr' ? 'çevirici' : 'translator'}
│   ├── slate-state/       # ${currentLang === 'tr' ? 'Durum yönetimi' : 'State management'}
│   ├── slate-arena/       # ${currentLang === 'tr' ? 'Bellek tahsisi' : 'Memory allocation'}
│   ├── slate-layout/      # ${currentLang === 'tr' ? 'Yerleşim motorları' : 'Layout engines'}
│   ├── slate-render/      # GPU renderer
│   ├── slate-rasterizer/  # CPU rasterizer
│   ├── slate-text/        # ${currentLang === 'tr' ? 'Metin oluşturma' : 'Text rendering'}
│   ├── slate-html/        # HTML5 parser
│   ├── slate-css/         # CSS3 engine
│   ├── slate-dom/         # DOM ${currentLang === 'tr' ? 'implementasyonu' : 'implementation'}
│   ├── slate-events/      # ${currentLang === 'tr' ? 'Olay sistemi' : 'Event system'}
│   ├── slate-image/       # ${currentLang === 'tr' ? 'Görüntü yükleme' : 'Image loading'}
│   ├── slate-network/     # HTTP client
│   ├── slate-script/      # JavaScript runtime
│   ├── slate-webapi/      # Web API ${currentLang === 'tr' ? 'bağlamaları' : 'bindings'}
│   ├── slate-webgl/       # WebGL
│   ├── slate-workers/     # Web Workers
│   ├── slate-wasm/        # WebAssembly
│   ├── slate-storage/     # Storage APIs
│   └── slate-websocket/   # WebSocket
├── docs/                  # ${currentLang === 'tr' ? 'Dokümantasyon' : 'Documentation'}
└── examples/              # ${currentLang === 'tr' ? 'Örnek uygulamalar' : 'Example applications'}</code></pre>
        </div>

        <div class="section">
            <h2>${icons.activity} ${currentLang === 'tr' ? 'Neden Slate?' : 'Why Slate?'}</h2>
            <ul class="feature-list">
                <li><strong>${currentLang === 'tr' ? 'Güvenlik' : 'Security'}</strong> - ${currentLang === 'tr' ? 'Sıfır DOM yüzeyi XSS ve DOM tabanlı saldırıları önler' : 'Zero DOM surface prevents XSS and DOM-based attacks'}</li>
                <li><strong>${currentLang === 'tr' ? 'Test Edilebilirlik' : 'Testability'}</strong> - ${currentLang === 'tr' ? 'Deterministik yürütme ve talimat kaydı' : 'Deterministic execution and instruction recording'}</li>
                <li><strong>${currentLang === 'tr' ? 'Performans' : 'Performance'}</strong> - ${currentLang === 'tr' ? 'Arena tahsisi ve sıfır-kopyalama mimarisi' : 'Arena allocation and zero-copy architecture'}</li>
                <li><strong>${currentLang === 'tr' ? 'Hata Ayıklama' : 'Debugging'}</strong> - ${currentLang === 'tr' ? 'Zaman yolculuğu ve anlık görüntüler' : 'Time-travel and snapshots'}</li>
                <li><strong>${currentLang === 'tr' ? 'Modülerlik' : 'Modularity'}</strong> - ${currentLang === 'tr' ? 'Her bileşen bağımsız kullanılabilir' : 'Each component can be used independently'}</li>
            </ul>
        </div>
        `;
    },

    'architecture': function() {
        const t = translations[currentLang];
        return `
        <div class="header">
            <h1>${icons.layers} ${t.architecture}</h1>
            <p class="subtitle">${t.archSubtitle}</p>
        </div>

        <div class="section">
            <h2>${icons.cpu} ${currentLang === 'tr' ? 'Atomik Talimat Seti (AIS)' : 'Atomic Instruction Set (AIS)'}</h2>
            <p>${currentLang === 'tr' ? 'Slate\'in temel yeniliği Atomik Talimat Setidir.' : 'The core innovation of Slate is the Atomic Instruction Set.'}</p>
            <ul class="feature-list">
                <li>${currentLang === 'tr' ? 'Deterministik yürütme' : 'Deterministic execution'}</li>
                <li>${currentLang === 'tr' ? 'Test edilebilirlik' : 'Testability'}</li>
                <li>${currentLang === 'tr' ? 'Hata ayıklanabilirlik' : 'Debuggability'}</li>
                <li>${currentLang === 'tr' ? 'Optimizasyon' : 'Optimization'}</li>
                <li>${currentLang === 'tr' ? 'Güvenlik' : 'Security'}</li>
            </ul>
        </div>

        <div class="section">
            <h2>${icons.activity} ${currentLang === 'tr' ? 'Render Hattı' : 'Rendering Pipeline'}</h2>
            <ul class="feature-list">
                <li>${currentLang === 'tr' ? 'Ayrıştırma - HTML/CSS' : 'Parse - HTML/CSS'}</li>
                <li>${currentLang === 'tr' ? 'DOM - Belge ağacı' : 'DOM - Document tree'}</li>
                <li>${currentLang === 'tr' ? 'Stil - Hesaplama' : 'Style - Computation'}</li>
                <li>${currentLang === 'tr' ? 'Yerleşim - Konumlar' : 'Layout - Positions'}</li>
                <li>${currentLang === 'tr' ? 'Boyama - Çizim listesi' : 'Paint - Display list'}</li>
                <li>${currentLang === 'tr' ? 'Rasterleştirme - Pikseller' : 'Raster - Pixels'}</li>
            </ul>
        </div>

        <div class="section">
            <h2>${icons.box} ${currentLang === 'tr' ? 'Bellek Yönetimi' : 'Memory Management'}</h2>
            <ul class="feature-list">
                <li>${currentLang === 'tr' ? 'Arena tahsisi' : 'Arena allocation'}</li>
                <li>${currentLang === 'tr' ? 'Çöp toplama yok' : 'No garbage collection'}</li>
                <li>${currentLang === 'tr' ? 'Önbellek dostu' : 'Cache-friendly'}</li>
                <li>${currentLang === 'tr' ? 'Deterministik' : 'Deterministic'}</li>
            </ul>
        </div>
        `;
    },

    'getting-started': function() {
        return `
        <div class="header">
            <h1>${icons.zap} ${currentLang === 'tr' ? 'Başlangıç' : 'Getting Started'}</h1>
            <p class="subtitle">${currentLang === 'tr' ? 'İlk uygulamanızı oluşturun' : 'Build your first application'}</p>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Gereksinimler' : 'Prerequisites'}</h2>
            <ul class="feature-list">
                <li>Rust 1.70+</li>
                <li>Cargo</li>
                <li>Git</li>
                <li>C compiler</li>
            </ul>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Kurulum' : 'Installation'}</h2>
            <pre><code>git clone https://github.com/yourusername/slate-engine.git
cd slate-engine
cargo build --release</code></pre>
        </div>
        `;
    },

    'slate-kernel': function() {
        return `
        <div class="header">
            <h1>${icons.cpu} slate-kernel</h1>
            <p class="subtitle">${currentLang === 'tr' ? 'Ana orkestratör ve giriş noktası' : 'Main orchestrator and entry point'}</p>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Genel Bakış' : 'Overview'}</h2>
            <p>${currentLang === 'tr' ? 'Kernel crate\'i tüm diğer bileşenleri koordine eden ana orkestratördür. Tüm web sayfası yaşam döngüsünü yönetir: HTML/CSS ayrıştırma, DOM oluşturma, stil hesaplama, yerleşim, render ve olay işleme.' : 'The kernel crate is the main orchestrator that coordinates all other components. It manages the entire web page lifecycle: HTML/CSS parsing, DOM construction, style computation, layout, rendering, and event handling.'}</p>
            
            <p>${currentLang === 'tr' ? 'Kernel, Slate\'in "sıfır DOM yüzeyi" felsefesinin merkezi uygulamasıdır. Tüm üst düzey API çağrıları önce Atomik Talimatlara (AIS) dönüştürülür, ardından yürütülür.' : 'The kernel is the central implementation of Slate\'s "zero DOM surface" philosophy. All high-level API calls are first translated into Atomic Instructions (AIS) before execution.'}</p>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Temel Bileşenler' : 'Key Components'}</h2>
            <pre><code>pub struct Kernel {
    // Durum yönetimi - tüm DOM ve stil durumunu tutar
    state: StateStore,
    
    // Bellek yönetimi - sayfa başına arena tahsisi
    arena: Arena,
    
    // WebCall → AIS çevirici
    dispatcher: Dispatcher,
    
    // Render motoru (GPU veya CPU)
    renderer: Renderer,
    
    // Olay döngüsü ve zamanlayıcı
    event_loop: EventLoop,
    
    // JavaScript çalışma zamanı
    script_runtime: Option<ScriptRuntime>,
    
    // Ağ istekleri için HTTP istemcisi
    network_client: NetworkClient,
}</code></pre>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Yaşam Döngüsü' : 'Lifecycle'}</h2>
            <ul class="feature-list">
                <li><strong>${currentLang === 'tr' ? 'Başlatma' : 'Initialization'}</strong> - ${currentLang === 'tr' ? 'Arena, durum deposu ve renderer oluşturma' : 'Create arena, state store, and renderer'}</li>
                <li><strong>${currentLang === 'tr' ? 'Yükleme' : 'Loading'}</strong> - ${currentLang === 'tr' ? 'HTML/CSS ayrıştırma ve DOM oluşturma' : 'Parse HTML/CSS and construct DOM'}</li>
                <li><strong>${currentLang === 'tr' ? 'Stil Hesaplama' : 'Style Computation'}</strong> - ${currentLang === 'tr' ? 'CSS basamaklama ve hesaplanmış stiller' : 'CSS cascade and computed styles'}</li>
                <li><strong>${currentLang === 'tr' ? 'Yerleşim' : 'Layout'}</strong> - ${currentLang === 'tr' ? 'Kutu ağacı oluşturma ve konumlandırma' : 'Build box tree and positioning'}</li>
                <li><strong>${currentLang === 'tr' ? 'Boyama' : 'Paint'}</strong> - ${currentLang === 'tr' ? 'Görüntü listesi oluşturma' : 'Generate display list'}</li>
                <li><strong>${currentLang === 'tr' ? 'Rasterleştirme' : 'Rasterization'}</strong> - ${currentLang === 'tr' ? 'Piksellere dönüştürme' : 'Convert to pixels'}</li>
                <li><strong>${currentLang === 'tr' ? 'Olay İşleme' : 'Event Processing'}</strong> - ${currentLang === 'tr' ? 'Kullanıcı etkileşimleri ve zamanlayıcılar' : 'User interactions and timers'}</li>
            </ul>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Örnek Kullanım' : 'Example Usage'}</h2>
            <h3>${currentLang === 'tr' ? 'Temel Render' : 'Basic Rendering'}</h3>
            <pre><code>use slate_kernel::Kernel;

fn main() -> Result<()> {
    let mut kernel = Kernel::new();
    
    // HTML yükle
    kernel.load_html(r#"
        &lt;!DOCTYPE html&gt;
        &lt;html&gt;
            &lt;head&gt;
                &lt;title&gt;Slate Demo&lt;/title&gt;
            &lt;/head&gt;
            &lt;body&gt;
                &lt;div class="container"&gt;
                    &lt;h1&gt;Hello Slate!&lt;/h1&gt;
                    &lt;p&gt;A minimalist browser engine&lt;/p&gt;
                &lt;/div&gt;
            &lt;/body&gt;
        &lt;/html&gt;
    "#)?;
    
    // CSS yükle
    kernel.load_css(r#"
        .container {
            display: flex;
            flex-direction: column;
            align-items: center;
            padding: 20px;
        }
        
        h1 {
            color: #2563eb;
            font-size: 32px;
            margin-bottom: 10px;
        }
        
        p {
            color: #64748b;
            font-size: 16px;
        }
    "#)?;
    
    // Render ve kaydet
    let frame = kernel.render()?;
    frame.save("output.png")?;
    
    Ok(())
}</code></pre>

            <h3>${currentLang === 'tr' ? 'Olay İşleme' : 'Event Handling'}</h3>
            <pre><code>// Tıklama olayı dinle
kernel.add_event_listener("click", |event| {
    println!("Clicked at: {:?}", event.position);
});

// Olay döngüsünü çalıştır
kernel.run_event_loop()?;</code></pre>

            <h3>${currentLang === 'tr' ? 'JavaScript Entegrasyonu' : 'JavaScript Integration'}</h3>
            <pre><code>// JavaScript çalışma zamanını etkinleştir
kernel.enable_javascript()?;

// Script çalıştır
kernel.eval_script(r#"
    document.querySelector('h1').textContent = 'Updated!';
    console.log('Script executed');
"#)?;</code></pre>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'API Referansı' : 'API Reference'}</h2>
            
            <div class="api-method">
                <div class="method-signature">fn load_html(&mut self, html: &str) -> Result&lt;()&gt;</div>
                <div class="method-desc">${currentLang === 'tr' ? 'HTML içeriğini ayrıştırır ve DOM ağacını oluşturur.' : 'Parses HTML content and constructs the DOM tree.'}</div>
            </div>

            <div class="api-method">
                <div class="method-signature">fn load_css(&mut self, css: &str) -> Result&lt;()&gt;</div>
                <div class="method-desc">${currentLang === 'tr' ? 'CSS kurallarını ayrıştırır ve stil deposuna ekler.' : 'Parses CSS rules and adds them to the style store.'}</div>
            </div>

            <div class="api-method">
                <div class="method-signature">fn render(&mut self) -> Result&lt;Frame&gt;</div>
                <div class="method-desc">${currentLang === 'tr' ? 'Tam render hattını çalıştırır ve bir frame döndürür.' : 'Runs the full rendering pipeline and returns a frame.'}</div>
            </div>

            <div class="api-method">
                <div class="method-signature">fn eval_script(&mut self, script: &str) -> Result&lt;Value&gt;</div>
                <div class="method-desc">${currentLang === 'tr' ? 'JavaScript kodunu çalıştırır ve sonucu döndürür.' : 'Executes JavaScript code and returns the result.'}</div>
            </div>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Performans Özellikleri' : 'Performance Characteristics'}</h2>
            <ul class="feature-list">
                <li>${currentLang === 'tr' ? 'Sıfır-kopyalama mimarisi ile minimal bellek tahsisi' : 'Minimal memory allocation with zero-copy architecture'}</li>
                <li>${currentLang === 'tr' ? 'Arena tahsisi ile O(1) sayfa sıfırlama' : 'O(1) page reset with arena allocation'}</li>
                <li>${currentLang === 'tr' ? 'Paralel stil hesaplama ve yerleşim' : 'Parallel style computation and layout'}</li>
                <li>${currentLang === 'tr' ? 'GPU hızlandırmalı render (wgpu)' : 'GPU-accelerated rendering (wgpu)'}</li>
                <li>${currentLang === 'tr' ? 'Deterministik yürütme ve test edilebilirlik' : 'Deterministic execution and testability'}</li>
            </ul>
        </div>
        `;
    },

    'slate-ais': function() {
        return `
        <div class="header">
            <h1>${icons.box} slate-ais</h1>
            <p class="subtitle">${currentLang === 'tr' ? 'Atomik Talimat Seti' : 'Atomic Instruction Set'}</p>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Genel Bakış' : 'Overview'}</h2>
            <p>${currentLang === 'tr' ? 'AIS (Atomic Instruction Set), Slate\'in temel yeniliğidir. Her üst düzey Web API çağrısı, yürütmeden önce kapalı bir atomik talimat kümesine ayrıştırılır. Bu yaklaşım deterministik yürütme, test edilebilirlik ve güvenlik sağlar.' : 'AIS (Atomic Instruction Set) is Slate\'s core innovation. Every high-level Web API call is decomposed into a closed set of atomic instructions before execution. This approach provides deterministic execution, testability, and security.'}</p>
            
            <p>${currentLang === 'tr' ? 'Geleneksel tarayıcılar DOM\'u doğrudan değiştirirken, Slate önce talimatları üretir, ardından bunları toplu olarak yürütür. Bu, zaman yolculuğu hata ayıklama, anlık görüntü ve geri alma/yineleme gibi güçlü özellikleri mümkün kılar.' : 'While traditional browsers mutate the DOM directly, Slate first generates instructions, then executes them in batch. This enables powerful features like time-travel debugging, snapshots, and undo/redo.'}</p>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Talimat Kategorileri' : 'Instruction Categories'}</h2>
            
            <h3>${currentLang === 'tr' ? '1. Durum Talimatları' : '1. State Instructions'}</h3>
            <p>${currentLang === 'tr' ? 'DOM ağacını ve özelliklerini değiştiren talimatlar:' : 'Instructions that modify the DOM tree and properties:'}</p>
            <ul class="feature-list">
                <li><strong>CreateNode</strong> - ${currentLang === 'tr' ? 'Yeni bir DOM düğümü oluştur' : 'Create a new DOM node'}</li>
                <li><strong>RemoveNode</strong> - ${currentLang === 'tr' ? 'Bir düğümü kaldır' : 'Remove a node'}</li>
                <li><strong>SetAttribute</strong> - ${currentLang === 'tr' ? 'Düğüm özelliğini ayarla' : 'Set node attribute'}</li>
                <li><strong>SetTextContent</strong> - ${currentLang === 'tr' ? 'Metin içeriğini güncelle' : 'Update text content'}</li>
                <li><strong>AppendChild</strong> - ${currentLang === 'tr' ? 'Alt düğüm ekle' : 'Append child node'}</li>
            </ul>

            <h3>${currentLang === 'tr' ? '2. Yerleşim Talimatları' : '2. Layout Instructions'}</h3>
            <p>${currentLang === 'tr' ? 'Kutu ağacını ve konumlandırmayı yöneten talimatlar:' : 'Instructions that manage the box tree and positioning:'}</p>
            <ul class="feature-list">
                <li><strong>CreateLayoutBox</strong> - ${currentLang === 'tr' ? 'Yeni bir yerleşim kutusu oluştur' : 'Create a new layout box'}</li>
                <li><strong>SetBoxSize</strong> - ${currentLang === 'tr' ? 'Kutu boyutlarını ayarla' : 'Set box dimensions'}</li>
                <li><strong>SetBoxPosition</strong> - ${currentLang === 'tr' ? 'Kutu konumunu ayarla' : 'Set box position'}</li>
                <li><strong>ComputeFlexLayout</strong> - ${currentLang === 'tr' ? 'Flexbox yerleşimini hesapla' : 'Compute flexbox layout'}</li>
                <li><strong>ComputeGridLayout</strong> - ${currentLang === 'tr' ? 'Grid yerleşimini hesapla' : 'Compute grid layout'}</li>
            </ul>

            <h3>${currentLang === 'tr' ? '3. Render Talimatları' : '3. Render Instructions'}</h3>
            <p>${currentLang === 'tr' ? 'Görüntü listesini oluşturan talimatlar:' : 'Instructions that build the display list:'}</p>
            <ul class="feature-list">
                <li><strong>DrawRect</strong> - ${currentLang === 'tr' ? 'Dikdörtgen çiz' : 'Draw rectangle'}</li>
                <li><strong>DrawText</strong> - ${currentLang === 'tr' ? 'Metin render et' : 'Render text'}</li>
                <li><strong>DrawImage</strong> - ${currentLang === 'tr' ? 'Görüntü çiz' : 'Draw image'}</li>
                <li><strong>DrawShadow</strong> - ${currentLang === 'tr' ? 'Gölge efekti uygula' : 'Apply shadow effect'}</li>
                <li><strong>ApplyTransform</strong> - ${currentLang === 'tr' ? 'Dönüşüm matrisi uygula' : 'Apply transformation matrix'}</li>
            </ul>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Talimat Yapısı' : 'Instruction Structure'}</h2>
            <pre><code>pub enum AisInstruction {
    // Durum talimatları
    CreateNode {
        id: NodeId,
        node_type: NodeType,
        parent: Option&lt;NodeId&gt;,
    },
    
    SetAttribute {
        node: NodeId,
        name: String,
        value: String,
    },
    
    // Yerleşim talimatları
    CreateLayoutBox {
        id: BoxId,
        node: NodeId,
        style: ComputedStyle,
    },
    
    SetBoxSize {
        id: BoxId,
        width: SubPixel,
        height: SubPixel,
    },
    
    SetBoxPosition {
        id: BoxId,
        x: SubPixel,
        y: SubPixel,
    },
    
    // Render talimatları
    DrawRect {
        rect: Rect,
        color: Color,
        border_radius: BorderRadius,
    },
    
    DrawText {
        text: String,
        font: FontId,
        position: Point,
        color: Color,
    },
    
    DrawImage {
        image: ImageId,
        src_rect: Rect,
        dst_rect: Rect,
    },
}</code></pre>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Örnek: WebCall → AIS Dönüşümü' : 'Example: WebCall → AIS Translation'}</h2>
            <p>${currentLang === 'tr' ? 'Bir element oluşturma çağrısının nasıl AIS talimatlarına dönüştürüldüğü:' : 'How an element creation call is translated to AIS instructions:'}</p>
            
            <pre><code>// Üst düzey WebCall
WebCall::CreateElement {
    tag: "div",
    parent: body_node,
}

// ↓ Dispatcher tarafından dönüştürülür ↓

// Oluşturulan AIS talimatları
[
    AisInstruction::CreateNode {
        id: NodeId(42),
        node_type: NodeType::Element("div"),
        parent: Some(body_node),
    },
    
    AisInstruction::CreateLayoutBox {
        id: BoxId(42),
        node: NodeId(42),
        style: ComputedStyle::default(),
    },
]</code></pre>

            <h3>${currentLang === 'tr' ? 'Karmaşık Örnek: Canvas çizimi' : 'Complex Example: Canvas drawing'}</h3>
            <pre><code>// JavaScript: ctx.fillRect(10, 20, 100, 50)

// ↓ Dönüştürülür ↓

[
    AisInstruction::DrawRect {
        rect: Rect {
            x: SubPixel(10.0),
            y: SubPixel(20.0),
            width: SubPixel(100.0),
            height: SubPixel(50.0),
        },
        color: current_fill_style,
        border_radius: BorderRadius::zero(),
    },
]</code></pre>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Avantajlar' : 'Benefits'}</h2>
            <ul class="feature-list">
                <li><strong>${currentLang === 'tr' ? 'Deterministik Yürütme' : 'Deterministic Execution'}</strong> - ${currentLang === 'tr' ? 'Aynı talimatlar her zaman aynı sonucu üretir' : 'Same instructions always produce the same result'}</li>
                <li><strong>${currentLang === 'tr' ? 'Test Edilebilirlik' : 'Testability'}</strong> - ${currentLang === 'tr' ? 'Talimatları kolayca kaydet, tekrarla ve doğrula' : 'Easily record, replay, and verify instructions'}</li>
                <li><strong>${currentLang === 'tr' ? 'Hata Ayıklama' : 'Debugging'}</strong> - ${currentLang === 'tr' ? 'Zaman yolculuğu hata ayıklama ve anlık görüntüler' : 'Time-travel debugging and snapshots'}</li>
                <li><strong>${currentLang === 'tr' ? 'Optimizasyon' : 'Optimization'}</strong> - ${currentLang === 'tr' ? 'Talimatları toplu işle ve optimize et' : 'Batch and optimize instructions'}</li>
                <li><strong>${currentLang === 'tr' ? 'Güvenlik' : 'Security'}</strong> - ${currentLang === 'tr' ? 'Talimatları yürütmeden önce doğrula' : 'Validate instructions before execution'}</li>
                <li><strong>${currentLang === 'tr' ? 'Serileştirme' : 'Serialization'}</strong> - ${currentLang === 'tr' ? 'Talimatları kolayca kaydet ve yükle' : 'Easily save and load instructions'}</li>
            </ul>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Performans' : 'Performance'}</h2>
            <p>${currentLang === 'tr' ? 'AIS mimarisi ek yük getirse de, birçok optimizasyon fırsatı sunar:' : 'While the AIS architecture adds overhead, it enables many optimization opportunities:'}</p>
            <ul class="feature-list">
                <li>${currentLang === 'tr' ? 'Toplu yürütme - Talimatları grupla ve bir kerede işle' : 'Batch execution - Group and process instructions at once'}</li>
                <li>${currentLang === 'tr' ? 'Gereksiz işlemleri eleme - Çakışan talimatları birleştir' : 'Redundancy elimination - Merge overlapping instructions'}</li>
                <li>${currentLang === 'tr' ? 'Paralel işleme - Bağımsız talimatları paralel çalıştır' : 'Parallel processing - Execute independent instructions in parallel'}</li>
                <li>${currentLang === 'tr' ? 'Önbellekleme - Talimat sonuçlarını önbelleğe al' : 'Caching - Cache instruction results'}</li>
            </ul>
        </div>
        `;
    },

    'slate-layout': function() {
        return `
        <div class="header">
            <h1>${icons.layers} slate-layout</h1>
            <p class="subtitle">${currentLang === 'tr' ? 'Gelişmiş yerleşim motorları' : 'Advanced layout engines'}</p>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Genel Bakış' : 'Overview'}</h2>
            <p>${currentLang === 'tr' ? 'slate-layout, modern web yerleşim algoritmalarının tam uygulamasını sağlar. Her yerleşim modu kendi motoruna sahiptir ve CSS spesifikasyonlarına tam uyumludur.' : 'slate-layout provides full implementation of modern web layout algorithms. Each layout mode has its own engine and is fully compliant with CSS specifications.'}</p>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Yerleşim Motorları' : 'Layout Engines'}</h2>
            
            <h3>1. Flexbox ${currentLang === 'tr' ? 'Motoru' : 'Engine'}</h3>
            <p>${currentLang === 'tr' ? 'Tek boyutlu esnek yerleşim sistemi. CSS Flexbox spesifikasyonuna tam uyumlu.' : 'One-dimensional flexible layout system. Fully compliant with CSS Flexbox specification.'}</p>
            <ul class="feature-list">
                <li><strong>flex-direction</strong> - row, row-reverse, column, column-reverse</li>
                <li><strong>justify-content</strong> - flex-start, flex-end, center, space-between, space-around, space-evenly</li>
                <li><strong>align-items</strong> - flex-start, flex-end, center, baseline, stretch</li>
                <li><strong>align-content</strong> - ${currentLang === 'tr' ? 'Çok satırlı hizalama' : 'Multi-line alignment'}</li>
                <li><strong>flex-wrap</strong> - nowrap, wrap, wrap-reverse</li>
                <li><strong>gap</strong> - ${currentLang === 'tr' ? 'Öğeler arası boşluk' : 'Spacing between items'}</li>
            </ul>

            <h3>2. Grid ${currentLang === 'tr' ? 'Motoru' : 'Engine'}</h3>
            <p>${currentLang === 'tr' ? 'İki boyutlu ızgara yerleşim sistemi. CSS Grid spesifikasyonuna tam uyumlu.' : 'Two-dimensional grid layout system. Fully compliant with CSS Grid specification.'}</p>
            <ul class="feature-list">
                <li><strong>grid-template-columns/rows</strong> - ${currentLang === 'tr' ? 'Sütun ve satır tanımları' : 'Column and row definitions'}</li>
                <li><strong>grid-auto-flow</strong> - row, column, dense</li>
                <li><strong>grid-gap</strong> - ${currentLang === 'tr' ? 'Hücre boşlukları' : 'Cell spacing'}</li>
                <li><strong>grid-area</strong> - ${currentLang === 'tr' ? 'Öğe konumlandırma' : 'Item positioning'}</li>
                <li><strong>minmax(), repeat(), fr</strong> - ${currentLang === 'tr' ? 'Gelişmiş boyutlandırma' : 'Advanced sizing'}</li>
            </ul>

            <h3>3. Block ${currentLang === 'tr' ? 'Motoru' : 'Engine'}</h3>
            <p>${currentLang === 'tr' ? 'Geleneksel blok yerleşim sistemi. Normal akış ve float desteği.' : 'Traditional block layout system. Normal flow and float support.'}</p>
            <ul class="feature-list">
                <li><strong>margin</strong> - ${currentLang === 'tr' ? 'Dış boşluk ve çökme' : 'Outer spacing and collapsing'}</li>
                <li><strong>padding</strong> - ${currentLang === 'tr' ? 'İç boşluk' : 'Inner spacing'}</li>
                <li><strong>border</strong> - ${currentLang === 'tr' ? 'Kenarlık kutusu' : 'Border box'}</li>
                <li><strong>float</strong> - left, right, none</li>
                <li><strong>clear</strong> - left, right, both, none</li>
            </ul>

            <h3>4. Inline ${currentLang === 'tr' ? 'Motoru' : 'Engine'}</h3>
            <p>${currentLang === 'tr' ? 'Satır içi metin ve öğe yerleşimi. Metin şekillendirme ve satır kesme.' : 'Inline text and element layout. Text shaping and line breaking.'}</p>
            <ul class="feature-list">
                <li><strong>line-height</strong> - ${currentLang === 'tr' ? 'Satır yüksekliği' : 'Line height'}</li>
                <li><strong>vertical-align</strong> - baseline, top, middle, bottom</li>
                <li><strong>text-align</strong> - left, right, center, justify</li>
                <li><strong>white-space</strong> - normal, nowrap, pre, pre-wrap</li>
                <li><strong>word-break</strong> - ${currentLang === 'tr' ? 'Kelime kesme kuralları' : 'Word breaking rules'}</li>
            </ul>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Flexbox Örneği' : 'Flexbox Example'}</h2>
            <pre><code>use slate_layout::flexbox::*;

// Flexbox container oluştur
let container = FlexContainer {
    direction: FlexDirection::Row,
    justify_content: JustifyContent::SpaceBetween,
    align_items: AlignItems::Center,
    wrap: FlexWrap::Wrap,
    gap: SubPixel(10.0),
};

// Flex öğeleri tanımla
let items = vec![
    FlexItem {
        flex_grow: 1.0,
        flex_shrink: 1.0,
        flex_basis: FlexBasis::Auto,
        align_self: AlignSelf::Auto,
    },
    FlexItem {
        flex_grow: 2.0,
        flex_shrink: 0.0,
        flex_basis: FlexBasis::Pixels(SubPixel(200.0)),
        align_self: AlignSelf::Stretch,
    },
];

// Yerleşimi hesapla
let layout = compute_flex_layout(
    &container,
    &items,
    AvailableSpace {
        width: SubPixel(800.0),
        height: SubPixel(600.0),
    },
)?;

// Sonuçları al
for (i, item_layout) in layout.items.iter().enumerate() {
    println!("Item {}: {:?}", i, item_layout.rect);
}</code></pre>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Grid Örneği' : 'Grid Example'}</h2>
            <pre><code>use slate_layout::grid::*;

// Grid container tanımla
let grid = GridContainer {
    template_columns: vec![
        TrackSize::Fr(1.0),
        TrackSize::Pixels(SubPixel(200.0)),
        TrackSize::Fr(2.0),
    ],
    template_rows: vec![
        TrackSize::Auto,
        TrackSize::MinMax(
            SubPixel(100.0),
            SubPixel(300.0),
        ),
        TrackSize::Fr(1.0),
    ],
    gap: Gap {
        column: SubPixel(10.0),
        row: SubPixel(15.0),
    },
    auto_flow: GridAutoFlow::Row,
};

// Grid öğesi yerleştir
let item = GridItem {
    column_start: 1,
    column_end: 3,  // 2 sütun kapla
    row_start: 1,
    row_end: 2,
};

// Yerleşimi hesapla
let layout = compute_grid_layout(&grid, &items, container_size)?;</code></pre>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Performans Optimizasyonları' : 'Performance Optimizations'}</h2>
            <ul class="feature-list">
                <li><strong>${currentLang === 'tr' ? 'Artımlı Yerleşim' : 'Incremental Layout'}</strong> - ${currentLang === 'tr' ? 'Sadece değişen kısımları yeniden hesapla' : 'Recompute only changed portions'}</li>
                <li><strong>${currentLang === 'tr' ? 'Önbellekleme' : 'Caching'}</strong> - ${currentLang === 'tr' ? 'Yerleşim sonuçlarını önbelleğe al' : 'Cache layout results'}</li>
                <li><strong>${currentLang === 'tr' ? 'Paralel İşleme' : 'Parallel Processing'}</strong> - ${currentLang === 'tr' ? 'Bağımsız alt ağaçları paralel hesapla' : 'Compute independent subtrees in parallel'}</li>
                <li><strong>${currentLang === 'tr' ? 'Kirli İzleme' : 'Dirty Tracking'}</strong> - ${currentLang === 'tr' ? 'Hangi düğümlerin yeniden hesaplanması gerektiğini izle' : 'Track which nodes need recomputation'}</li>
            </ul>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Yerleşim Hattı' : 'Layout Pipeline'}</h2>
            <pre><code>// 1. Stil hesaplama
let computed_styles = compute_styles(&dom_tree, &stylesheets)?;

// 2. Kutu ağacı oluşturma
let box_tree = build_box_tree(&dom_tree, &computed_styles)?;

// 3. Yerleşim hesaplama
let layout_tree = compute_layout(
    &box_tree,
    viewport_size,
)?;

// 4. Sonuçları al
for node in layout_tree.iter() {
    println!("Node {}: {:?}", node.id, node.rect);
}</code></pre>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Desteklenen CSS Özellikleri' : 'Supported CSS Properties'}</h2>
            <table>
                <tr>
                    <th>${currentLang === 'tr' ? 'Özellik' : 'Property'}</th>
                    <th>${currentLang === 'tr' ? 'Durum' : 'Status'}</th>
                    <th>${currentLang === 'tr' ? 'Notlar' : 'Notes'}</th>
                </tr>
                <tr>
                    <td>display</td>
                    <td>✓ ${currentLang === 'tr' ? 'Tam' : 'Full'}</td>
                    <td>block, inline, flex, grid, none</td>
                </tr>
                <tr>
                    <td>position</td>
                    <td>✓ ${currentLang === 'tr' ? 'Tam' : 'Full'}</td>
                    <td>static, relative, absolute, fixed</td>
                </tr>
                <tr>
                    <td>width/height</td>
                    <td>✓ ${currentLang === 'tr' ? 'Tam' : 'Full'}</td>
                    <td>px, %, auto, min/max</td>
                </tr>
                <tr>
                    <td>margin/padding</td>
                    <td>✓ ${currentLang === 'tr' ? 'Tam' : 'Full'}</td>
                    <td>${currentLang === 'tr' ? 'Çökme dahil' : 'Including collapsing'}</td>
                </tr>
                <tr>
                    <td>flexbox</td>
                    <td>✓ ${currentLang === 'tr' ? 'Tam' : 'Full'}</td>
                    <td>${currentLang === 'tr' ? 'Tüm özellikler' : 'All properties'}</td>
                </tr>
                <tr>
                    <td>grid</td>
                    <td>✓ ${currentLang === 'tr' ? 'Tam' : 'Full'}</td>
                    <td>${currentLang === 'tr' ? 'Tüm özellikler' : 'All properties'}</td>
                </tr>
            </table>
        </div>
        `;
    },

    'slate-css': function() {
        return `
        <div class="header">
            <h1>${icons.code} slate-css</h1>
            <p class="subtitle">${currentLang === 'tr' ? 'CSS3 motoru' : 'CSS3 engine'}</p>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Özellikler' : 'Features'}</h2>
            <ul class="feature-list">
                <li>${currentLang === 'tr' ? 'Seçici eşleştirme' : 'Selector matching'}</li>
                <li>${currentLang === 'tr' ? 'Basamaklama çözümlemesi' : 'Cascade resolution'}</li>
                <li>${currentLang === 'tr' ? 'Özgüllük hesaplama' : 'Specificity calculation'}</li>
                <li>${currentLang === 'tr' ? 'Hesaplanmış stiller' : 'Computed styles'}</li>
                <li>${currentLang === 'tr' ? 'Gradyanlar ve animasyonlar' : 'Gradients and animations'}</li>
            </ul>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Desteklenen Seçiciler' : 'Supported Selectors'}</h2>
            <pre><code>/* Type selector */
div { }

/* Class selector */
.button { }

/* ID selector */
#header { }

/* Attribute selector */
[type="text"] { }

/* Pseudo-class */
:hover, :focus, :nth-child(2) { }

/* Combinators */
div > p { }  /* Child */
h1 + p { }   /* Adjacent sibling */
h1 ~ p { }   /* General sibling */</code></pre>
        </div>
        `;
    },

    'slate-html': function() {
        return `
        <div class="header">
            <h1>${icons.code} slate-html</h1>
            <p class="subtitle">${currentLang === 'tr' ? 'HTML5 ayrıştırıcı' : 'HTML5 parser'}</p>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Özellikler' : 'Features'}</h2>
            <ul class="feature-list">
                <li>${currentLang === 'tr' ? 'Tam HTML5 spesifikasyonu uyumluluğu' : 'Full HTML5 specification compliance'}</li>
                <li>${currentLang === 'tr' ? 'Hata kurtarma' : 'Error recovery'}</li>
                <li>${currentLang === 'tr' ? 'Quirks modu algılama' : 'Quirks mode detection'}</li>
                <li>${currentLang === 'tr' ? 'Akış desteği' : 'Streaming support'}</li>
            </ul>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Örnek' : 'Example'}</h2>
            <pre><code>let mut parser = HtmlParser::new();
let result = parser.parse(html)?;

println!("Nodes: {}", result.tree.node_count());
println!("Web calls: {}", result.web_calls.len());</code></pre>
        </div>
        `;
    },

    'slate-image': function() {
        return `
        <div class="header">
            <h1>${icons.box} slate-image</h1>
            <p class="subtitle">${currentLang === 'tr' ? 'Görüntü yükleme ve işleme' : 'Image loading and processing'}</p>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Desteklenen Formatlar' : 'Supported Formats'}</h2>
            <ul class="feature-list">
                <li>PNG - Portable Network Graphics</li>
                <li>JPEG - Joint Photographic Experts Group</li>
                <li>WebP - Modern web format</li>
                <li>GIF - Graphics Interchange Format</li>
                <li>BMP - Bitmap</li>
                <li>TIFF - Tagged Image File Format</li>
                <li>ICO - Icon format</li>
            </ul>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Örnek' : 'Example'}</h2>
            <pre><code>let loader = ImageLoader::new();
let image = loader.load("image.png").await?;

let thumbnail = image.resize(200, 200);
let grayscale = image.to_grayscale();</code></pre>
        </div>
        `;
    },

    'slate-dispatcher': function() {
        return `
        <div class="header">
            <h1>${icons.activity} slate-dispatcher</h1>
            <p class="subtitle">${currentLang === 'tr' ? 'WebCall → AIS çevirici' : 'WebCall → AIS translator'}</p>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Genel Bakış' : 'Overview'}</h2>
            <p>${currentLang === 'tr' ? 'Dispatcher, üst düzey web API çağrılarını atomik talimatlara çevirir.' : 'The dispatcher translates high-level web API calls into atomic instructions.'}</p>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Örnek' : 'Example'}</h2>
            <pre><code>let mut dispatcher = Dispatcher::new();

let call = WebCall::CreateElement {
    tag: "div".to_string(),
    parent: NodeId(1),
};

let instructions = dispatcher.dispatch(call);</code></pre>
        </div>
        `;
    },

    'slate-state': function() {
        return `
        <div class="header">
            <h1>${icons.database} slate-state</h1>
            <p class="subtitle">${currentLang === 'tr' ? 'Deterministik durum deposu' : 'Deterministic state store'}</p>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Özellikler' : 'Features'}</h2>
            <ul class="feature-list">
                <li>${currentLang === 'tr' ? 'SlotMap ile kararlı tutamaçlar' : 'Stable handles with SlotMap'}</li>
                <li>${currentLang === 'tr' ? 'Eşzamanlı erişim için DashMap' : 'DashMap for concurrent access'}</li>
                <li>${currentLang === 'tr' ? 'Anlık görüntü desteği' : 'Snapshot support'}</li>
                <li>${currentLang === 'tr' ? 'Kirli izleme' : 'Dirty tracking'}</li>
            </ul>
        </div>
        `;
    },

    'slate-arena': function() {
        return `
        <div class="header">
            <h1>${icons.box} slate-arena</h1>
            <p class="subtitle">${currentLang === 'tr' ? 'Sayfa başına arena tahsisi' : 'Per-page arena allocation'}</p>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Özellikler' : 'Features'}</h2>
            <ul class="feature-list">
                <li>${currentLang === 'tr' ? 'O(1) sıfırlama' : 'O(1) reset'}</li>
                <li>${currentLang === 'tr' ? 'Çöp toplama yok' : 'No garbage collection'}</li>
                <li>${currentLang === 'tr' ? 'Hızlı tahsis' : 'Fast allocation'}</li>
            </ul>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Örnek' : 'Example'}</h2>
            <pre><code>let arena = Arena::new();
let node = arena.alloc(Node::new("div"));
arena.reset(); // O(1)</code></pre>
        </div>
        `;
    },

    'slate-render': function() {
        return `
        <div class="header">
            <h1>${icons.box} slate-render</h1>
            <p class="subtitle">${currentLang === 'tr' ? 'GPU renderer (wgpu)' : 'GPU renderer (wgpu)'}</p>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Özellikler' : 'Features'}</h2>
            <ul class="feature-list">
                <li>wgpu ${currentLang === 'tr' ? 'tabanlı' : 'based'}</li>
                <li>${currentLang === 'tr' ? 'Instanced çizim' : 'Instanced drawing'}</li>
                <li>${currentLang === 'tr' ? 'Headless render' : 'Headless rendering'}</li>
            </ul>
        </div>
        `;
    },

    'slate-rasterizer': function() {
        return `
        <div class="header">
            <h1>${icons.box} slate-rasterizer</h1>
            <p class="subtitle">${currentLang === 'tr' ? 'CPU rasterizer' : 'CPU rasterizer'}</p>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Özellikler' : 'Features'}</h2>
            <ul class="feature-list">
                <li>${currentLang === 'tr' ? 'Görüntü listesi oluşturma' : 'Display list generation'}</li>
                <li>${currentLang === 'tr' ? 'Piksel rasterleştirme' : 'Pixel rasterization'}</li>
            </ul>
        </div>
        `;
    },

    'slate-text': function() {
        return `
        <div class="header">
            <h1>${icons.code} slate-text</h1>
            <p class="subtitle">${currentLang === 'tr' ? 'Metin şekillendirme ve render' : 'Text shaping and rendering'}</p>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Özellikler' : 'Features'}</h2>
            <ul class="feature-list">
                <li>HarfBuzz ${currentLang === 'tr' ? 'ile şekillendirme' : 'for shaping'}</li>
                <li>FreeType ${currentLang === 'tr' ? 'ile rasterleştirme' : 'for rasterization'}</li>
                <li>${currentLang === 'tr' ? 'Ligature desteği' : 'Ligature support'}</li>
                <li>${currentLang === 'tr' ? 'Kerning' : 'Kerning'}</li>
            </ul>
        </div>
        `;
    },

    'slate-dom': function() {
        return `
        <div class="header">
            <h1>${icons.layers} slate-dom</h1>
            <p class="subtitle">${currentLang === 'tr' ? 'DOM implementasyonu' : 'DOM implementation'}</p>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Özellikler' : 'Features'}</h2>
            <ul class="feature-list">
                <li>${currentLang === 'tr' ? 'Mutasyon izleme' : 'Mutation tracking'}</li>
                <li>${currentLang === 'tr' ? 'Sorgu seçiciler' : 'Query selectors'}</li>
                <li>${currentLang === 'tr' ? 'Ağaç gezinme' : 'Tree traversal'}</li>
            </ul>
        </div>
        `;
    },

    'slate-events': function() {
        return `
        <div class="header">
            <h1>${icons.activity} slate-events</h1>
            <p class="subtitle">${currentLang === 'tr' ? 'DOM olay sistemi' : 'DOM event system'}</p>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Özellikler' : 'Features'}</h2>
            <ul class="feature-list">
                <li>${currentLang === 'tr' ? 'Olay kabarcıklanması' : 'Event bubbling'}</li>
                <li>${currentLang === 'tr' ? 'Olay yakalama' : 'Event capturing'}</li>
                <li>${currentLang === 'tr' ? 'Olay delegasyonu' : 'Event delegation'}</li>
            </ul>
        </div>
        `;
    },

    'slate-network': function() {
        return `
        <div class="header">
            <h1>${icons.globe} slate-network</h1>
            <p class="subtitle">${currentLang === 'tr' ? 'Ağ istekleri' : 'Network requests'}</p>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Özellikler' : 'Features'}</h2>
            <ul class="feature-list">
                <li>tokio/reqwest ${currentLang === 'tr' ? 'tabanlı' : 'based'}</li>
                <li>${currentLang === 'tr' ? 'Akış desteği' : 'Streaming support'}</li>
                <li>${currentLang === 'tr' ? 'Sandbox' : 'Sandbox'}</li>
            </ul>
        </div>
        `;
    },

    'slate-script': function() {
        return `
        <div class="header">
            <h1>${icons.code} slate-script</h1>
            <p class="subtitle">${currentLang === 'tr' ? 'JavaScript çalışma zamanı' : 'JavaScript runtime'}</p>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Özellikler' : 'Features'}</h2>
            <ul class="feature-list">
                <li>Boa ${currentLang === 'tr' ? 'motoru' : 'engine'}</li>
                <li>${currentLang === 'tr' ? 'Host köprüsü' : 'Host bridge'}</li>
                <li>${currentLang === 'tr' ? 'Sıfır DOM yüzeyi' : 'Zero DOM surface'}</li>
            </ul>
        </div>
        `;
    },

    'slate-webapi': function() {
        return `
        <div class="header">
            <h1>${icons.globe} slate-webapi</h1>
            <p class="subtitle">${currentLang === 'tr' ? 'Web API bağlamaları' : 'Web API bindings'}</p>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Özellikler' : 'Features'}</h2>
            <ul class="feature-list">
                <li>DOM API</li>
                <li>Canvas 2D</li>
                <li>WebGL</li>
                <li>${currentLang === 'tr' ? 'Form işleme' : 'Form handling'}</li>
            </ul>
        </div>
        `;
    },

    'slate-webgl': function() {
        return `
        <div class="header">
            <h1>${icons.box} slate-webgl</h1>
            <p class="subtitle">WebGL 1.0 & 2.0</p>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Özellikler' : 'Features'}</h2>
            <ul class="feature-list">
                <li>WebGL 1.0 & 2.0</li>
                <li>wgpu ${currentLang === 'tr' ? 'arka uç' : 'backend'}</li>
                <li>${currentLang === 'tr' ? 'Shader derleme' : 'Shader compilation'}</li>
            </ul>
        </div>
        `;
    },

    'slate-workers': function() {
        return `
        <div class="header">
            <h1>${icons.cpu} slate-workers</h1>
            <p class="subtitle">Web Workers</p>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Özellikler' : 'Features'}</h2>
            <ul class="feature-list">
                <li>Dedicated Workers</li>
                <li>Shared Workers</li>
                <li>Service Workers</li>
            </ul>
        </div>
        `;
    },

    'slate-wasm': function() {
        return `
        <div class="header">
            <h1>${icons.zap} slate-wasm</h1>
            <p class="subtitle">WebAssembly</p>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Özellikler' : 'Features'}</h2>
            <ul class="feature-list">
                <li>wasmtime ${currentLang === 'tr' ? 'çalışma zamanı' : 'runtime'}</li>
                <li>WASI ${currentLang === 'tr' ? 'desteği' : 'support'}</li>
            </ul>
        </div>
        `;
    },

    'slate-storage': function() {
        return `
        <div class="header">
            <h1>${icons.database} slate-storage</h1>
            <p class="subtitle">${currentLang === 'tr' ? 'Depolama API\'leri' : 'Storage APIs'}</p>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Özellikler' : 'Features'}</h2>
            <ul class="feature-list">
                <li>localStorage</li>
                <li>sessionStorage</li>
                <li>IndexedDB</li>
            </ul>
        </div>
        `;
    },

    'slate-websocket': function() {
        return `
        <div class="header">
            <h1>${icons.activity} slate-websocket</h1>
            <p class="subtitle">WebSocket</p>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Özellikler' : 'Features'}</h2>
            <ul class="feature-list">
                <li>${currentLang === 'tr' ? 'Otomatik yeniden bağlanma' : 'Automatic reconnection'}</li>
                <li>Ping/pong heartbeat</li>
                <li>${currentLang === 'tr' ? 'İkili ve metin mesajları' : 'Binary and text messages'}</li>
            </ul>
        </div>
        `;
    },

    'slate-canvas': function() {
        return `
        <div class="header">
            <h1>${icons.box} Canvas 2D</h1>
            <p class="subtitle">Canvas 2D API</p>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Özellikler' : 'Features'}</h2>
            <ul class="feature-list">
                <li>${currentLang === 'tr' ? 'Tam Canvas 2D API' : 'Full Canvas 2D API'}</li>
                <li>${currentLang === 'tr' ? 'Çizim işlemleri' : 'Drawing operations'}</li>
                <li>${currentLang === 'tr' ? 'Dönüşümler' : 'Transformations'}</li>
            </ul>
        </div>
        `;
    },

    'slate-forms': function() {
        return `
        <div class="header">
            <h1>${icons.code} Forms</h1>
            <p class="subtitle">${currentLang === 'tr' ? 'Form işleme' : 'Form handling'}</p>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Özellikler' : 'Features'}</h2>
            <ul class="feature-list">
                <li>HTML5 ${currentLang === 'tr' ? 'doğrulama' : 'validation'}</li>
                <li>${currentLang === 'tr' ? 'Form gönderimi' : 'Form submission'}</li>
            </ul>
        </div>
        `;
    },

    'slate-svg': function() {
        return `
        <div class="header">
            <h1>${icons.box} SVG</h1>
            <p class="subtitle">SVG ${currentLang === 'tr' ? 'desteği' : 'support'}</p>
        </div>

        <div class="section">
            <h2>${currentLang === 'tr' ? 'Özellikler' : 'Features'}</h2>
            <ul class="feature-list">
                <li>${currentLang === 'tr' ? 'Temel şekiller' : 'Basic shapes'}</li>
                <li>${currentLang === 'tr' ? 'Yollar' : 'Paths'}</li>
                <li>${currentLang === 'tr' ? 'Dönüşümler' : 'Transforms'}</li>
            </ul>
        </div>
        `;
    },

    'roadmap': function() {
        return `
        <div class="header">
            <h1>${icons.activity} ${currentLang === 'tr' ? 'Yol Haritası' : 'Roadmap'}</h1>
            <p class="subtitle">${currentLang === 'tr' ? 'Slate Engine geliştirme planı' : 'Slate Engine development plan'}</p>
        </div>

        <div class="section">
            <h2>✅ ${currentLang === 'tr' ? 'Faz 1: Temel Altyapı (Tamamlandı)' : 'Phase 1: Core Infrastructure (Completed)'}</h2>
            <ul class="feature-list">
                <li>Atomik Talimat Seti (AIS) ${currentLang === 'tr' ? 'tasarımı' : 'design'}</li>
                <li>Arena ${currentLang === 'tr' ? 'tahsis sistemi' : 'allocation system'}</li>
                <li>${currentLang === 'tr' ? 'Durum deposu ve anlık görüntüler' : 'State store and snapshots'}</li>
                <li>Dispatcher ${currentLang === 'tr' ? 'mimarisi' : 'architecture'}</li>
                <li>${currentLang === 'tr' ? 'Temel render hattı' : 'Basic rendering pipeline'}</li>
            </ul>
        </div>

        <div class="section">
            <h2>✅ ${currentLang === 'tr' ? 'Faz 2: HTML & CSS (Tamamlandı)' : 'Phase 2: HTML & CSS (Completed)'}</h2>
            <ul class="feature-list">
                <li>HTML5 parser ${currentLang === 'tr' ? 'ile hata kurtarma' : 'with error recovery'}</li>
                <li>CSS3 ${currentLang === 'tr' ? 'ayrıştırıcı ve seçici motoru' : 'parser and selector engine'}</li>
                <li>${currentLang === 'tr' ? 'Basamaklama ve özgüllük hesaplama' : 'Cascade and specificity computation'}</li>
                <li>${currentLang === 'tr' ? 'Hesaplanmış stil sistemi' : 'Computed style system'}</li>
                <li>${currentLang === 'tr' ? 'Gradyan ve animasyon desteği' : 'Gradient and animation support'}</li>
            </ul>
        </div>

        <div class="section">
            <h2>✅ ${currentLang === 'tr' ? 'Faz 3: Yerleşim Motorları (Tamamlandı)' : 'Phase 3: Layout Engines (Completed)'}</h2>
            <ul class="feature-list">
                <li>Flexbox ${currentLang === 'tr' ? 'tam implementasyonu' : 'full implementation'}</li>
                <li>CSS Grid ${currentLang === 'tr' ? 'tam implementasyonu' : 'full implementation'}</li>
                <li>Block ${currentLang === 'tr' ? 've inline yerleşim' : 'and inline layout'}</li>
                <li>${currentLang === 'tr' ? 'Metin şekillendirme (HarfBuzz)' : 'Text shaping (HarfBuzz)'}</li>
                <li>${currentLang === 'tr' ? 'Font render (FreeType)' : 'Font rendering (FreeType)'}</li>
            </ul>
        </div>

        <div class="section">
            <h2>✅ ${currentLang === 'tr' ? 'Faz 4: Medya & Formlar (Tamamlandı)' : 'Phase 4: Media & Forms (Completed)'}</h2>
            <ul class="feature-list">
                <li>${currentLang === 'tr' ? 'Görüntü yükleme (PNG, JPEG, WebP, GIF, vb.)' : 'Image loading (PNG, JPEG, WebP, GIF, etc.)'}</li>
                <li>Canvas 2D API</li>
                <li>SVG ${currentLang === 'tr' ? 'temel desteği' : 'basic support'}</li>
                <li>HTML5 ${currentLang === 'tr' ? 'form elemanları' : 'form elements'}</li>
                <li>${currentLang === 'tr' ? 'Form doğrulama' : 'Form validation'}</li>
            </ul>
        </div>

        <div class="section">
            <h2>✅ ${currentLang === 'tr' ? 'Faz 5: Modern Web (Tamamlandı)' : 'Phase 5: Modern Web (Completed)'}</h2>
            <ul class="feature-list">
                <li>WebGL 1.0 & 2.0</li>
                <li>Web Workers (Dedicated, Shared, Service)</li>
                <li>WebAssembly ${currentLang === 'tr' ? 'çalışma zamanı' : 'runtime'}</li>
                <li>localStorage, sessionStorage, IndexedDB</li>
                <li>WebSocket ${currentLang === 'tr' ? 'desteği' : 'support'}</li>
            </ul>
        </div>

        <div class="section">
            <h2>🚧 ${currentLang === 'tr' ? 'Faz 6: JavaScript Entegrasyonu (Devam Ediyor)' : 'Phase 6: JavaScript Integration (In Progress)'}</h2>
            <ul class="feature-list">
                <li>✅ Boa ${currentLang === 'tr' ? 'motor entegrasyonu' : 'engine integration'}</li>
                <li>✅ ${currentLang === 'tr' ? 'Temel DOM API bağlamaları' : 'Basic DOM API bindings'}</li>
                <li>🚧 ${currentLang === 'tr' ? 'Tam DOM API yüzeyi' : 'Full DOM API surface'}</li>
                <li>🚧 ${currentLang === 'tr' ? 'Olay işleyicileri' : 'Event handlers'}</li>
                <li>⏳ Promise & async/await</li>
                <li>⏳ Fetch API</li>
                <li>⏳ ${currentLang === 'tr' ? 'Zamanlayıcılar (setTimeout, setInterval)' : 'Timers (setTimeout, setInterval)'}</li>
            </ul>
        </div>

        <div class="section">
            <h2>⏳ ${currentLang === 'tr' ? 'Faz 7: Performans & Optimizasyon (Planlandı)' : 'Phase 7: Performance & Optimization (Planned)'}</h2>
            <ul class="feature-list">
                <li>${currentLang === 'tr' ? 'Artımlı yerleşim' : 'Incremental layout'}</li>
                <li>${currentLang === 'tr' ? 'Paralel stil hesaplama' : 'Parallel style computation'}</li>
                <li>${currentLang === 'tr' ? 'GPU hızlandırmalı metin render' : 'GPU-accelerated text rendering'}</li>
                <li>${currentLang === 'tr' ? 'Talimat optimizasyonu' : 'Instruction optimization'}</li>
                <li>${currentLang === 'tr' ? 'Bellek havuzu' : 'Memory pooling'}</li>
                <li>${currentLang === 'tr' ? 'Önbellek iyileştirmeleri' : 'Cache improvements'}</li>
            </ul>
        </div>

        <div class="section">
            <h2>⏳ ${currentLang === 'tr' ? 'Faz 8: Gelişmiş Özellikler (Planlandı)' : 'Phase 8: Advanced Features (Planned)'}</h2>
            <ul class="feature-list">
                <li>CSS ${currentLang === 'tr' ? 'animasyonları ve geçişler' : 'animations and transitions'}</li>
                <li>CSS ${currentLang === 'tr' ? 'dönüşümleri (2D/3D)' : 'transforms (2D/3D)'}</li>
                <li>${currentLang === 'tr' ? 'Medya sorguları' : 'Media queries'}</li>
                <li>${currentLang === 'tr' ? 'Özel özellikler (CSS değişkenleri)' : 'Custom properties (CSS variables)'}</li>
                <li>Shadow DOM</li>
                <li>${currentLang === 'tr' ? 'Özel elementler' : 'Custom elements'}</li>
            </ul>
        </div>

        <div class="section">
            <h2>⏳ ${currentLang === 'tr' ? 'Faz 9: Geliştirici Araçları (Planlandı)' : 'Phase 9: Developer Tools (Planned)'}</h2>
            <ul class="feature-list">
                <li>${currentLang === 'tr' ? 'Zaman yolculuğu hata ayıklayıcı' : 'Time-travel debugger'}</li>
                <li>${currentLang === 'tr' ? 'Talimat görselleştirici' : 'Instruction visualizer'}</li>
                <li>${currentLang === 'tr' ? 'Performans profiler' : 'Performance profiler'}</li>
                <li>${currentLang === 'tr' ? 'Bellek analiz aracı' : 'Memory analyzer'}</li>
                <li>${currentLang === 'tr' ? 'Test kaydedici/oynatıcı' : 'Test recorder/player'}</li>
            </ul>
        </div>

        <div class="section">
            <h2>⏳ ${currentLang === 'tr' ? 'Faz 10: Ekosistem (Planlandı)' : 'Phase 10: Ecosystem (Planned)'}</h2>
            <ul class="feature-list">
                <li>${currentLang === 'tr' ? 'Komut satırı aracı' : 'Command-line tool'}</li>
                <li>${currentLang === 'tr' ? 'Headless test runner' : 'Headless test runner'}</li>
                <li>${currentLang === 'tr' ? 'Ekran görüntüsü API\'si' : 'Screenshot API'}</li>
                <li>PDF ${currentLang === 'tr' ? 'oluşturma' : 'generation'}</li>
                <li>${currentLang === 'tr' ? 'Eklenti sistemi' : 'Plugin system'}</li>
                <li>${currentLang === 'tr' ? 'Dil bağlamaları (Python, Node.js)' : 'Language bindings (Python, Node.js)'}</li>
            </ul>
        </div>

        <div class="section">
            <h2>${icons.zap} ${currentLang === 'tr' ? 'Katkıda Bulunun' : 'Contribute'}</h2>
            <p>${currentLang === 'tr' ? 'Slate açık kaynaklı bir projedir ve katkılarınızı bekliyoruz!' : 'Slate is an open-source project and we welcome your contributions!'}</p>
            <ul class="feature-list">
                <li>${currentLang === 'tr' ? 'GitHub\'da yıldız verin' : 'Star us on GitHub'}</li>
                <li>${currentLang === 'tr' ? 'Hata bildirin veya özellik isteyin' : 'Report bugs or request features'}</li>
                <li>${currentLang === 'tr' ? 'Kod katkısında bulunun' : 'Contribute code'}</li>
                <li>${currentLang === 'tr' ? 'Dokümantasyonu geliştirin' : 'Improve documentation'}</li>
                <li>${currentLang === 'tr' ? 'Projeyi paylaşın' : 'Share the project'}</li>
            </ul>
        </div>
        `;
    }
};

function showSection(sectionId) {
    const mainContentInner = document.getElementById('mainContentInner');
    const mainContent = document.getElementById('mainContent');
    
    if (content[sectionId]) {
        mainContentInner.innerHTML = content[sectionId]();
    } else {
        mainContentInner.innerHTML = '<div class="header"><h1>Section: ' + sectionId + '</h1></div>';
    }
    
    document.querySelectorAll('.nav-item').forEach(function(item) {
        item.classList.remove('active');
    });
    
    const items = document.querySelectorAll('.nav-item');
    for (let i = 0; i < items.length; i++) {
        const onclick = items[i].getAttribute('onclick');
        if (onclick && onclick.includes(sectionId)) {
            items[i].classList.add('active');
            break;
        }
    }
    
    mainContent.scrollTop = 0;
}

function toggleSection(titleElement) {
    const section = titleElement.parentElement;
    section.classList.toggle('collapsed');
}

function toggleSidebar() {
    const sidebar = document.getElementById('sidebar');
    const toggle = document.querySelector('.sidebar-toggle');
    
    const isCollapsed = sidebar.classList.contains('collapsed');
    
    if (isCollapsed) {
        // Açılıyor
        const lastWidth = sidebar.getAttribute('data-last-width') || '320';
        
        // Önce width'i ayarla, sonra collapsed'ı kaldır
        sidebar.style.width = lastWidth + 'px';
        
        // Bir frame bekle ki transition çalışsın
        requestAnimationFrame(() => {
            sidebar.classList.remove('collapsed');
            toggle.classList.remove('collapsed');
            toggle.style.left = lastWidth + 'px';
        });
    } else {
        // Kapanıyor
        // Mevcut genişliği kaydet
        const currentWidth = sidebar.offsetWidth;
        sidebar.setAttribute('data-last-width', currentWidth);
        
        // Collapsed ekle
        sidebar.classList.add('collapsed');
        toggle.classList.add('collapsed');
        toggle.style.left = '0px';
    }
}

function setLang(lang) {
    currentLang = lang;
    
    document.querySelectorAll('.lang-btn').forEach(function(btn) {
        btn.classList.remove('active');
        if (btn.textContent.toLowerCase() === lang) {
            btn.classList.add('active');
        }
    });
    
    const activeNav = document.querySelector('.nav-item.active');
    if (activeNav) {
        const onclick = activeNav.getAttribute('onclick');
        if (onclick) {
            const match = onclick.match(/'([^']+)'/);
            if (match) {
                showSection(match[1]);
            }
        }
    }
}

document.addEventListener('DOMContentLoaded', function() {
    const searchBox = document.getElementById('searchBox');
    
    if (searchBox) {
        searchBox.addEventListener('input', function(e) {
            const query = e.target.value.toLowerCase();
            
            document.querySelectorAll('.nav-item').forEach(function(item) {
                const text = item.textContent.toLowerCase();
                if (text.includes(query) || query === '') {
                    item.style.display = 'block';
                } else {
                    item.style.display = 'none';
                }
            });
        });
    }
    
    // Sidebar resize functionality
    const resizeHandle = document.getElementById('resizeHandle');
    const sidebar = document.getElementById('sidebar');
    const sidebarToggle = document.querySelector('.sidebar-toggle');
    let isResizing = false;
    let startX = 0;
    let startWidth = 0;
    
    if (resizeHandle) {
        resizeHandle.addEventListener('mousedown', function(e) {
            isResizing = true;
            startX = e.clientX;
            startWidth = sidebar.offsetWidth;
            resizeHandle.classList.add('resizing');
            document.body.style.cursor = 'ew-resize';
            document.body.style.userSelect = 'none';
            e.preventDefault();
        });
    }
    
    document.addEventListener('mousemove', function(e) {
        if (!isResizing) return;
        
        const deltaX = e.clientX - startX;
        const newWidth = startWidth + deltaX;
        
        // Min ve max genişlik kontrolü
        const minWidth = 200;
        const maxWidth = 600;
        
        if (newWidth >= minWidth && newWidth <= maxWidth) {
            sidebar.style.width = newWidth + 'px';
            // Daraltma butonunu da güncelle
            if (!sidebar.classList.contains('collapsed')) {
                sidebarToggle.style.left = newWidth + 'px';
            }
        }
    });
    
    document.addEventListener('mouseup', function() {
        if (isResizing) {
            isResizing = false;
            resizeHandle.classList.remove('resizing');
            document.body.style.cursor = '';
            document.body.style.userSelect = '';
            
            // Son genişliği kaydet
            sidebar.setAttribute('data-last-width', sidebar.offsetWidth);
        }
    });
    
    showSection('overview');
});
