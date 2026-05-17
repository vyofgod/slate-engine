#

 Slate Web API - Wine-like Compatibility Layer

## Core Philosophy

**Slate does NOT implement Web APIs directly.** Instead, it uses a **Wine-like translation layer** that converts high-level Web API calls into atomic primitives.

```
┌─────────────────────────────────────────────────────────────┐
│  Just like Wine translates Windows APIs to Linux syscalls   │
│  Slate translates Web APIs to Atomic Instructions (AIS)     │
└─────────────────────────────────────────────────────────────┘
```

## Architecture Comparison

### ❌ Traditional Browser (Chromium, Firefox)

```
JavaScript
    ↓
Web APIs (full implementation)
    ↓
Blink/Gecko (complex rendering engine)
    ↓
GPU
```

**Problems:**
- Massive codebase (millions of lines)
- Tight coupling between APIs
- Hard to optimize
- Interpretation overhead

### ✅ Slate (Wine-like approach)

```
JavaScript
    ↓
Web API Translator (Wine-like layer) ← NO IMPLEMENTATION, ONLY TRANSLATION
    ↓
WebCall (normalized)
    ↓
Dispatcher (stateless)
    ↓
Atomic Instructions (200-500 primitives)
    ↓
Kernel → GPU
```

**Benefits:**
- Minimal codebase
- Zero interpretation overhead
- Embarrassingly parallel
- Deterministic execution

## The Translation Layer

### What is Translation?

**Translation** means converting a high-level API call into a sequence of atomic operations, WITHOUT implementing the API itself.

#### Example: `document.createElement('div')`

**Traditional Browser (implements DOM):**
```rust
// Chromium/Blink approach
fn create_element(tag: &str) -> Element {
    let element = Element::new(tag);
    element.attach_to_document();
    element.setup_event_listeners();
    element.initialize_style();
    element.create_layout_object();
    // ... 100+ lines of setup code
    element
}
```

**Slate (translates to AIS):**
```rust
// Slate approach - pure translation
fn translate_create_element(node: NodeId, tag: &str) -> Vec<AtomicInstruction> {
    vec![
        AtomicInstruction::CreateNode { id: node },
        AtomicInstruction::SetNodeType { id: node, node_type: parse_tag(tag) },
    ]
}
```

**Key difference:**
- Traditional: **Implements** a full DOM element with all its complexity
- Slate: **Translates** to 2 atomic instructions

### Translation Examples

#### 1. DOM Manipulation

```javascript
// JavaScript
const div = document.createElement('div');
div.setAttribute('id', 'main');
div.style.backgroundColor = 'red';
parent.appendChild(div);
```

**Slate Translation:**
```rust
// Translated to AIS
[
    CreateNode { id: NodeId(1) },
    SetNodeType { id: NodeId(1), node_type: Div },
    SetAttribute { id: NodeId(1), name: "id", value: "main" },
    SetBackgroundColor { id: NodeId(1), color: Rgba8(255, 0, 0, 255) },
    AttachChild { parent: NodeId(0), child: NodeId(1) },
    InvalidateLayout { id: NodeId(0) },
]
```

#### 2. Canvas API

```javascript
// JavaScript
const ctx = canvas.getContext('2d');
ctx.fillStyle = 'blue';
ctx.fillRect(10, 10, 100, 50);
```

**Slate Translation:**
```rust
// Translated to AIS
[
    SetFillColor { ctx: CtxId(1), color: Rgba8(0, 0, 255, 255) },
    FillRect { ctx: CtxId(1), x: 10.0, y: 10.0, width: 100.0, height: 50.0 },
]
```

#### 3. Event Listeners

```javascript
// JavaScript
element.addEventListener('click', handler);
```

**Slate Translation:**
```rust
// Translated to AIS
[
    RegisterEventHandler { 
        node: NodeId(1), 
        event_type: Click, 
        handler_id: HandlerId(42) 
    },
]
```

## The Translator Module

### Core Component: `WebApiTranslator`

```rust
pub struct WebApiTranslator {
    pending_calls: Vec<OwnedWebCall>,
    node_metadata: HashMap<NodeId, NodeMetadata>,
}

impl WebApiTranslator {
    /// Translate a Web API call to AIS primitives.
    /// This is the "Wine syscall handler" of Slate.
    pub fn translate(&mut self, call: OwnedWebCall) -> Vec<AtomicInstruction> {
        match call {
            OwnedWebCall::CreateElement { node, tag } => {
                // NO IMPLEMENTATION - just translation
                vec![
                    AtomicInstruction::CreateNode { id: node },
                    AtomicInstruction::SetNodeType { id: node, node_type: parse_tag(tag) },
                ]
            }
            // ... more translations
        }
    }
}
```

### Translation Rules

1. **Stateless**: Translator has no hidden state, only metadata for context
2. **Deterministic**: Same input → same AIS sequence
3. **Single-pass**: O(n) translation, no backtracking
4. **Atomic**: Every Web API call reduces to 1-10 AIS primitives

## Comparison with Wine

### Wine Architecture

```
Windows Application
    ↓
Windows API call (CreateWindow, etc.)
    ↓
Wine Translation Layer ← Translates to Linux syscalls
    ↓
Linux Kernel
```

### Slate Architecture

```
Web Application (JavaScript)
    ↓
Web API call (createElement, etc.)
    ↓
Slate Translation Layer ← Translates to AIS
    ↓
Slate Kernel
```

### Key Similarities

| Aspect | Wine | Slate |
|--------|------|-------|
| **Goal** | Run Windows apps on Linux | Run Web apps on Slate |
| **Method** | Translate Windows APIs to Linux syscalls | Translate Web APIs to AIS |
| **No Implementation** | Doesn't implement Windows | Doesn't implement Web APIs |
| **Compatibility** | Binary compatible with Windows | API compatible with Web |
| **Performance** | Near-native (no emulation) | Near-native (no interpretation) |

## The Atomic Instruction Set (AIS)

### What are Atomic Instructions?

Atomic instructions are the **machine code** of Slate. They are:

- **Primitive**: Cannot be decomposed further
- **Orthogonal**: No overlap in functionality
- **Composable**: Complex operations = sequence of primitives
- **GPU-friendly**: Map directly to GPU operations

### AIS Categories

#### 1. Layout Primitives (Geometry)
```rust
SetPosition { node: NodeId, x: f32, y: f32 }
SetSize { node: NodeId, width: f32, height: f32 }
SetClip { node: NodeId, rect: Rect }
SetFlexBasis { node: NodeId, basis: f32 }
DetachFromFlow { node: NodeId }
```

#### 2. Render Primitives (GPU Commands)
```rust
FillRect { x: f32, y: f32, width: f32, height: f32, color: Rgba8 }
StrokeRect { x: f32, y: f32, width: f32, height: f32, color: Rgba8, width: f32 }
DrawText { x: f32, y: f32, text: String, font: FontId, color: Rgba8 }
FillPath { path: PathId, color: Rgba8 }
PushLayer { opacity: f32, blend_mode: BlendMode }
PopLayer
```

#### 3. State Primitives (Functional Deltas)
```rust
CreateNode { id: NodeId }
AttachChild { parent: NodeId, child: NodeId }
DetachChild { parent: NodeId, child: NodeId }
SetAttribute { id: NodeId, name: String, value: String }
InvalidateLayout { id: NodeId }
```

### Target: 200-500 Total Primitives

Slate aims for **200-500 total atomic instructions**. Any growth beyond this is considered a design failure.

**Why so few?**
- Forces orthogonality
- Enables exhaustive testing
- Simplifies optimization
- Reduces attack surface

## Translation Pipeline

### Full Flow

```
1. JavaScript Execution
   ↓
2. Web API Call (e.g., createElement)
   ↓
3. Boa Host Function (captures call)
   ↓
4. OwnedWebCall (owned data)
   ↓
5. WebApiTranslator.translate()
   ↓
6. Vec<AtomicInstruction> (AIS sequence)
   ↓
7. Kernel.execute()
   ↓
8. GPU Commands
```

### Example: Complete Flow

```javascript
// 1. JavaScript
const div = document.createElement('div');
div.style.width = '100px';
div.style.height = '50px';
div.style.backgroundColor = 'red';
```

```rust
// 2. Captured as OwnedWebCall
vec![
    OwnedWebCall::CreateElement { node: NodeId(1), tag: "div" },
    OwnedWebCall::SetInlineStyle { 
        node: NodeId(1), 
        css: "width:100px;height:50px;background-color:red" 
    },
]
```

```rust
// 3. Translated to AIS
vec![
    // From CreateElement
    AtomicInstruction::CreateNode { id: NodeId(1) },
    AtomicInstruction::SetNodeType { id: NodeId(1), node_type: Div },
    
    // From SetInlineStyle (parsed CSS)
    AtomicInstruction::SetWidth { id: NodeId(1), width: 100.0 },
    AtomicInstruction::SetHeight { id: NodeId(1), height: 50.0 },
    AtomicInstruction::SetBackgroundColor { 
        id: NodeId(1), 
        color: Rgba8 { r: 255, g: 0, b: 0, a: 255 } 
    },
    AtomicInstruction::InvalidateLayout { id: NodeId(1) },
]
```

```rust
// 4. Executed by Kernel
// → Updates state store
// → Triggers layout pass
// → Generates GPU commands
// → Renders to screen
```

## Benefits of Translation Approach

### 1. No Interpretation Overhead

**Traditional Browser:**
```
JavaScript → Parse → Interpret → Execute → Render
           ↑ Overhead at every step
```

**Slate:**
```
JavaScript → Translate once → Execute AIS → Render
           ↑ Single translation, then native speed
```

### 2. Embarrassingly Parallel

Since AIS are pure functions with no hidden state:
```rust
// Can execute in parallel
instructions.par_iter().for_each(|instr| {
    kernel.execute(instr);
});
```

### 3. Deterministic Replay

```rust
// Record session
let instructions = session.record();

// Replay exactly
for instr in instructions {
    kernel.execute(instr);
}
// → Identical result, every time
```

### 4. Minimal Attack Surface

- **Traditional browser**: Millions of lines, thousands of APIs
- **Slate**: 200-500 primitives, exhaustively tested

### 5. Easy Optimization

```rust
// Optimize at AIS level, benefits ALL Web APIs
fn optimize(instructions: Vec<AIS>) -> Vec<AIS> {
    instructions
        .deduplicate()      // Remove redundant ops
        .batch_similar()    // Batch GPU commands
        .reorder_safe()     // Reorder for cache
}
```

## Implementation Status

### ✅ Completed

- [x] Translation architecture
- [x] WebApiTranslator core
- [x] DOM API translation
- [x] Style API translation
- [x] Basic CSS parsing

### 🔄 In Progress

- [ ] Complete AIS primitive set
- [ ] Canvas API translation
- [ ] Event API translation
- [ ] Full CSS property coverage

### 📋 Planned

- [ ] WebGL translation
- [ ] WebRTC translation
- [ ] Service Worker translation
- [ ] WebAssembly integration

## Comparison: Lines of Code

### Traditional Browser (Chromium)

```
Blink (rendering engine):  ~2,000,000 lines
V8 (JavaScript):          ~1,500,000 lines
Total:                    ~3,500,000 lines
```

### Slate (Translation approach)

```
WebApiTranslator:         ~2,000 lines
Dispatcher:               ~1,000 lines
AIS definitions:          ~500 lines
Kernel:                   ~3,000 lines
Total:                    ~6,500 lines
```

**Result: 500x smaller codebase**

## Conclusion

Slate's Web API layer is **NOT an implementation** - it's a **translation layer**, just like Wine.

**Key Principles:**
1. **Never implement** - always translate
2. **Stateless translation** - no hidden state
3. **Atomic reduction** - every API → AIS sequence
4. **Deterministic** - same input → same AIS
5. **Minimal primitives** - target 200-500 total

This approach gives Slate:
- ✅ Minimal codebase
- ✅ Maximum performance
- ✅ Perfect determinism
- ✅ Easy optimization
- ✅ Small attack surface

**Slate is to Web APIs what Wine is to Windows APIs.**
