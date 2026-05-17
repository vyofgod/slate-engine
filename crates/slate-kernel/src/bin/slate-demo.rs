//! The Phase 1 demo.
//!
//! Reads a hardcoded HTML snippet, runs it through the Dispatcher,
//! applies the resulting AIS to the state store, and prints the
//! instruction stream to stdout. This is the visible proof that
//! "translate and dominate" works end-to-end, even in the skeleton.

use slate_ais::{AtomicInstruction, NodeId};
use slate_dispatcher::WebCall;
use slate_kernel::{parse, Kernel};

const SAMPLE: &str = r#"
<div style="width:200;height:100;background:red">
    <span style="width:80;height:40;background:blue"></span>
</div>
"#;

fn main() {
    println!("╔══════════════════════════════════════════════╗");
    println!("║  Slate Engine — Phase 1 demo                 ║");
    println!("║  dirty web → normalize → decompose → AIS     ║");
    println!("╚══════════════════════════════════════════════╝");
    println!("\nsource HTML:{SAMPLE}");
    println!("-- AIS stream --");

    let mut kernel = Kernel::new();
    let mut next_id: u32 = 1;
    let mut stack: Vec<(NodeId, u32)> = Vec::new();

    for ev in parse::events(SAMPLE) {
        match ev {
            parse::Event::Open(tag, attrs) => {
                let id = NodeId(next_id);
                next_id += 1;

                // Record our position under the current parent, then
                // hand the event to the kernel.
                let (parent, index) = match stack.last_mut() {
                    Some(last) => {
                        let idx = last.1;
                        last.1 += 1;
                        (Some(last.0), idx)
                    }
                    None => (None, 0),
                };

                let stream = kernel
                    .submit(WebCall::CreateElement { node: id, tag })
                    .expect("dispatch create_element");
                print_stream(&stream);

                if let Some(parent) = parent {
                    let stream = kernel
                        .submit(WebCall::AppendChild {
                            parent,
                            child: id,
                            index,
                        })
                        .expect("dispatch append_child");
                    print_stream(&stream);
                }

                for (name, value) in attrs {
                    let stream = kernel
                        .submit(WebCall::SetAttribute { node: id, name, value })
                        .expect("dispatch set_attribute");
                    print_stream(&stream);
                }

                stack.push((id, 0));
            }
            parse::Event::Close(_) => {
                stack.pop();
            }
            parse::Event::Text(_) => {
                // Phase 1: text primitives (glyph runs) are not yet
                // modeled in AIS. Skip cleanly.
            }
        }
    }

    let s = kernel.snapshot();
    println!("\n-- snapshot --");
    println!(
        "version = {}    nodes = {}    root = {:?}",
        s.version, s.node_count, s.root
    );
}

fn print_stream(stream: &[AtomicInstruction]) {
    for instr in stream {
        match instr {
            AtomicInstruction::Layout(p) => println!("  [LAY] {p:?}"),
            AtomicInstruction::Render(p) => println!("  [REN] {p:?}"),
            AtomicInstruction::State(p)  => println!("  [STA] {p:?}"),
        }
    }
}
