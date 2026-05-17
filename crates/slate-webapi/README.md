# Slate Web API Compatibility Layer

**Slate does NOT implement Web APIs.** Instead, it provides a **Wine-like translation layer** that converts high-level Web API calls into Slate's Atomic Instruction Set (AIS).

## Philosophy

Just like Wine translates Windows API calls to Linux syscalls without implementing Windows, Slate translates Web API calls to atomic primitives without implementing the Web platform.

```
┌─────────────────────────────────────────────────────────┐
│  Wine: Windows APIs → Linux syscalls                    │
│  Slate: Web APIs → Atomic Instructions                  │
└─────────────────────────────────────────────────────────┘
```

## Architecture

```
JavaScript Code
     ↓
Web API Call (e.g., createElement)
     ↓
WebApiTranslator ← Wine-like translation layer (NO IMPLEMENTATION)
     ↓
Atomic Instructions (AIS)
     ↓
Kernel → GPU
```

## Key Principle: Translation, Not Implementation

### ❌ Traditional Browser (Chromium)

```rust
// Implements full DOM
fn create_element(tag: &str) -> Element {
    let element = Element::new(tag);
    element.attach_to_document();
    element.setup_event_listeners();
    // ... 100+ lines of implementation
    element
}
```

**Code size:** ~4,000,000 lines

### ✅ Slate (Wine-like Translation)

```rust
// Pure translation to AIS
fn translate_create_element(node: NodeId, tag: &str) -> Vec<AtomicInstruction> {
    vec![
        AtomicInstruction::CreateNode { id: node },
        AtomicInstruction::SetNodeType { id: node, node_type: parse_tag(tag) },
    ]
}
```

**Code size:** ~6,500 lines

**Result: 600x smaller codebase**

## Implemented APIs

### Core APIs (✅ Complete)

- **DOM API**: Document, Element, Node interfaces with full manipulation methods
- **Console API**: log, error, warn, info, debug, trace, assert, clear, count, time/timeEnd
- **Event API**: addEventListener, removeEventListener, dispatchEvent
- **Timer API**: setTimeout, clearTimeout, setInterval, clearInterval, requestAnimationFrame
- **Storage API**: localStorage and sessionStorage with full CRUD operations
- **URL API**: URL and URLSearchParams constructors
- **Performance API**: performance.now(), mark(), measure()
- **Crypto API**: getRandomValues(), randomUUID()

### Network APIs (🔄 Partial)

- **Fetch API**: Basic structure (needs integration with slate-network)
- **WebSocket API**: Basic structure (needs implementation)

### Graphics APIs (🔄 Partial)

- **Canvas 2D API**: Basic drawing methods (needs full implementation)

### Device APIs (🔄 Partial)

- **Geolocation API**: Basic structure
- **Notification API**: Basic structure

## Usage

### Basic Example

```rust
use slate_webapi::WebApiRuntime;

// Create runtime with all Web APIs installed
let mut runtime = WebApiRuntime::new()?;

// Execute JavaScript code
runtime.eval(r#"
    const div = document.createElement('div');
    div.setAttribute('id', 'main');
    div.style.setProperty('background-color', 'blue');
    
    const text = document.createTextNode('Hello, Slate!');
    div.appendChild(text);
    
    console.log('Element created:', div);
"#)?;

// Drain WebCalls for processing by dispatcher
let web_calls = runtime.drain_web_calls();
```

### Integration with Slate Engine

```rust
use slate_webapi::WebApiRuntime;
use slate_dispatcher::Dispatcher;

let mut runtime = WebApiRuntime::new()?;
let mut dispatcher = Dispatcher::new();

// Execute JavaScript
runtime.eval("/* your JS code */")?;

// Get WebCalls and dispatch to AIS
let calls = runtime.drain_web_calls();
for call in calls {
    let ais_stream = dispatcher.dispatch(call.as_web_call());
    // Process AIS stream...
}
```

## API Coverage

### Document Object Model

```javascript
// Element creation
const div = document.createElement('div');
const text = document.createTextNode('Hello');

// DOM manipulation
parent.appendChild(child);
parent.removeChild(child);
parent.insertBefore(newChild, refChild);

// Attributes
element.setAttribute('id', 'main');
element.getAttribute('id');
element.removeAttribute('class');

// Classes
element.classList.add('active');

// Styles
element.style.setProperty('color', 'red');

// Queries
document.getElementById('main');
document.querySelector('.class');
document.querySelectorAll('div');
```

### Console

```javascript
console.log('message', value);
console.error('error message');
console.warn('warning');
console.info('info');
console.debug('debug');
console.trace();
console.assert(condition, 'message');
console.clear();
console.count('label');
console.time('timer');
console.timeEnd('timer');
```

### Storage

```javascript
// localStorage
localStorage.setItem('key', 'value');
const value = localStorage.getItem('key');
localStorage.removeItem('key');
localStorage.clear();

// sessionStorage (same API)
sessionStorage.setItem('key', 'value');
```

### Timers

```javascript
const timeoutId = setTimeout(() => {
    console.log('Delayed');
}, 1000);
clearTimeout(timeoutId);

const intervalId = setInterval(() => {
    console.log('Repeating');
}, 1000);
clearInterval(intervalId);

requestAnimationFrame((timestamp) => {
    console.log('Frame:', timestamp);
});
```

### Performance

```javascript
const start = performance.now();
// ... do work ...
const end = performance.now();
console.log('Took:', end - start, 'ms');

performance.mark('start');
// ... do work ...
performance.mark('end');
performance.measure('work', 'start', 'end');
```

### Crypto

```javascript
const uuid = crypto.randomUUID();
console.log(uuid); // "550e8400-e29b-41d4-a716-446655440000"

const array = new Uint8Array(16);
crypto.getRandomValues(array);
```

## Implementation Status

| API Category | Status | Notes |
|-------------|--------|-------|
| DOM Core | ✅ Complete | Full CRUD operations |
| Console | ✅ Complete | All standard methods |
| Events | 🔄 Partial | Structure ready, needs slate-events integration |
| Timers | 🔄 Partial | Structure ready, needs event loop |
| Storage | ✅ Complete | In-memory implementation |
| Fetch | 🔄 Partial | Needs slate-network integration |
| Canvas 2D | 🔄 Partial | Basic methods, needs full implementation |
| WebGL | ❌ Planned | Phase 4 |
| WebRTC | ❌ Planned | Phase 4 |
| Service Workers | ❌ Planned | Phase 4 |
| IndexedDB | ❌ Planned | Phase 4 |

## Architecture Details

### Polyfill Layer

The crate uses a two-layer approach:

1. **Low-level bindings**: Rust functions registered as `__slate_*` in JavaScript
2. **High-level polyfills**: JavaScript wrappers that provide standard Web API interfaces

This allows:
- Zero-copy data transfer between JS and Rust
- Standard Web API surface for JavaScript code
- Efficient WebCall generation

### WebCall Generation

All DOM operations generate `OwnedWebCall` instances that are:
- Collected in a buffer during JavaScript execution
- Drained after execution completes
- Converted to `WebCall` for dispatcher processing
- Decomposed into Atomic Instructions (AIS)

### Determinism

The Web API layer maintains Slate's determinism guarantees:
- No wall clock access (use performance.now() which is deterministic)
- No direct state mutation (everything goes through WebCalls)
- Reproducible execution from same input sequence

## Future Work

### Phase 4 Priorities

1. **Complete Event System**: Full integration with slate-events
2. **Timer Implementation**: Event loop with proper scheduling
3. **Fetch Integration**: Connect to slate-network for real HTTP
4. **Canvas 2D**: Complete implementation with path operations
5. **WebGL**: OpenGL ES 2.0/3.0 compatibility
6. **WebAssembly**: Wasm module loading and execution

### Additional APIs

- XMLHttpRequest (legacy compatibility)
- WebSocket (real-time communication)
- IndexedDB (client-side database)
- Service Workers (offline support)
- Web Workers (multi-threading)
- WebRTC (peer-to-peer)
- Media APIs (audio/video)

## Testing

```bash
# Run all tests
cargo test -p slate-webapi

# Run specific test
cargo test -p slate-webapi test_dom_operations

# Run with output
cargo test -p slate-webapi -- --nocapture
```

## Contributing

When adding new Web APIs:

1. Create a new module in `src/` (e.g., `src/webrtc.rs`)
2. Implement low-level bindings using `NativeFunction`
3. Add high-level polyfill in `bindings.rs`
4. Update `lib.rs` to export the module
5. Add tests in the module
6. Update this README

## License

Apache-2.0 OR MIT, at your option.
