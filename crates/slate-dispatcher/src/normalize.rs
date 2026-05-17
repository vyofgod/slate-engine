//! Stage 1 of the bridge: *normalize*.
//!
//! Dirty input arrives here. Strings are parsed into typed atoms.
//! Unknown tags collapse to `Tag::Generic`. Unknown style properties
//! are dropped (they cannot survive to AIS — by construction).

use crate::ir::{NormalizedCall, ResolvedStyle, Tag, WebCall};
use crate::{style, DispatchError};

#[inline]
pub fn normalize(call: WebCall<'_>) -> Result<NormalizedCall, DispatchError> {
    Ok(match call {
        WebCall::CreateElement { node, tag } => NormalizedCall::CreateElement {
            node,
            tag: Tag::from_str(tag),
        },
        WebCall::CreateTextNode { node, .. } => NormalizedCall::CreateElement {
            node,
            tag: Tag::Text,
        },
        WebCall::AppendChild { parent, child, index } => NormalizedCall::AppendChild {
            parent,
            child,
            index,
        },
        WebCall::RemoveChild { parent, child } => {
            // For now, treat as AppendChild with special index
            // In full implementation, this would be a separate NormalizedCall variant
            NormalizedCall::AppendChild {
                parent,
                child,
                index: u32::MAX, // Special marker for removal
            }
        }
        WebCall::InsertBefore { parent, new_child, .. } => {
            // Simplified: treat as append for now
            NormalizedCall::AppendChild {
                parent,
                child: new_child,
                index: 0,
            }
        }
        WebCall::SetAttribute { node, name, value } => {
            // Phase 1: only the `style` attribute feeds the render
            // pipeline. Other attrs will later become AttrBind state
            // primitives; for now we just drop them on the floor.
            if name == "style" {
                NormalizedCall::Style {
                    node,
                    style: style::parse(value)?,
                }
            } else {
                NormalizedCall::Style {
                    node,
                    style: ResolvedStyle::default(),
                }
            }
        }
        WebCall::RemoveAttribute { node, .. } => {
            // For now, just return empty style
            NormalizedCall::Style {
                node,
                style: ResolvedStyle::default(),
            }
        }
        WebCall::AddClass { node, .. } => {
            // For now, treat as no-op style
            NormalizedCall::Style {
                node,
                style: ResolvedStyle::default(),
            }
        }
        WebCall::SetInlineStyle { node, css } => NormalizedCall::Style {
            node,
            style: style::parse(css)?,
        },
        WebCall::AnchorRect { node, rect } => NormalizedCall::AnchorRect { node, rect },
    })
}
