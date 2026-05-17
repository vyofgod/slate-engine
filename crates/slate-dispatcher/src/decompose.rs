//! Stage 2 of the bridge: *decompose*.
//!
//! A [`NormalizedCall`] is expanded into the sequence of AIS primitives
//! that reproduces its observable behavior. Expansion is O(1) per call
//! (each call emits a bounded number of primitives — this is the whole
//! point of "atomic reduction").

use slate_ais::{
    AtomicInstruction, LayoutPrimitive, RenderPrimitive, StatePrimitive, Stream,
};

use crate::ir::{NormalizedCall, ResolvedStyle};

#[inline]
pub fn decompose(call: &NormalizedCall) -> Stream {
    let mut out: Stream = smallvec::SmallVec::new();
    decompose_into(call, &mut out);
    out
}

pub fn decompose_into(call: &NormalizedCall, out: &mut Stream) {
    match call {
        NormalizedCall::CreateElement { node, tag: _tag } => {
            // The tag is an index into the node's class table; in
            // Phase 1 we treat every tag uniformly at the AIS layer.
            // Tag-specific semantics live in the Dispatcher, not here.
            out.push(AtomicInstruction::State(StatePrimitive::NodeCreate {
                node: *node,
            }));
        }

        NormalizedCall::AppendChild { parent, child, index } => {
            out.push(AtomicInstruction::State(StatePrimitive::NodeAttach {
                node: *child,
                parent: *parent,
                index: *index,
            }));
        }

        NormalizedCall::Style { node, style } => {
            emit_style(*node, style, out);
        }

        NormalizedCall::AnchorRect { node, rect } => {
            out.push(AtomicInstruction::Layout(LayoutPrimitive::SetPosition {
                node: *node,
                point: rect.origin,
            }));
            out.push(AtomicInstruction::Layout(LayoutPrimitive::SetSize {
                node: *node,
                size: rect.size,
            }));
        }
    }
}

fn emit_style(
    node: slate_ais::NodeId,
    style: &ResolvedStyle,
    out: &mut Stream,
) {
    if let Some(size) = style.size {
        out.push(AtomicInstruction::Layout(LayoutPrimitive::SetSize {
            node,
            size,
        }));
    }

    if let Some(clip) = style.clip {
        out.push(AtomicInstruction::Layout(LayoutPrimitive::SetClip {
            node,
            rect: clip,
        }));
    }

    if let Some(bg) = style.background {
        // A background paints the node's box. The box geometry comes
        // from the layout pass; the render primitive references a
        // rect that will be resolved by the compositor via `node`.
        // In Phase 1 we stamp the size we have, defaulting to 0×0 if
        // none was set — the kernel treats a zero-area fill as a
        // no-op, so this is cheap and deterministic.
        let rect = slate_ais::Rect {
            origin: slate_ais::Point::ORIGIN,
            size:   style.size.unwrap_or(slate_ais::Size::ZERO),
        };
        out.push(AtomicInstruction::Render(RenderPrimitive::FillRect {
            rect,
            color: bg,
        }));
    }

    if let Some((color, width)) = style.border {
        let rect = slate_ais::Rect {
            origin: slate_ais::Point::ORIGIN,
            size:   style.size.unwrap_or(slate_ais::Size::ZERO),
        };
        out.push(AtomicInstruction::Render(RenderPrimitive::StrokeRect {
            rect,
            color,
            width,
        }));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::Tag;
    use slate_ais::NodeId;

    #[test]
    fn create_element_emits_single_state_prim() {
        let n = NodeId(7);
        let s = decompose(&NormalizedCall::CreateElement {
            node: n,
            tag: Tag::Div,
        });
        assert_eq!(s.len(), 1);
        assert!(matches!(
            s[0],
            AtomicInstruction::State(StatePrimitive::NodeCreate { .. })
        ));
    }

    #[test]
    fn style_with_size_and_bg_emits_layout_plus_render() {
        let style = ResolvedStyle {
            size:       Some(slate_ais::Size::new(200.0, 100.0)),
            background: Some(slate_ais::Rgba8::rgb(255, 0, 0)),
            ..Default::default()
        };
        let s = decompose(&NormalizedCall::Style { node: NodeId(1), style });
        assert_eq!(s.len(), 2);
        assert!(matches!(
            s[0],
            AtomicInstruction::Layout(LayoutPrimitive::SetSize { .. })
        ));
        assert!(matches!(
            s[1],
            AtomicInstruction::Render(RenderPrimitive::FillRect { .. })
        ));
    }
}
