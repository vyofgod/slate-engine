//! The Dispatcher's internal representation.
//!
//! [`WebCall`] is the *dirty input* boundary — it can contain anything
//! a JS runtime hands us. [`NormalizedCall`] is what the rest of the
//! pipeline sees: strings have been trimmed, numbers have been parsed,
//! unknown properties have been rejected.

use slate_ais::{NodeId, Rect, Rgba8, Size};

/// A `WebCall` variant with owned strings. Used where a borrow
/// cannot cross a boundary — JS host functions (returning to Boa's
/// GC heap), network chunk handlers (bytes freed after the call),
/// or a worker thread consuming a channel.
///
/// Use [`OwnedWebCall::as_web_call`] to zero-copy-borrow into a
/// [`WebCall`] when feeding the Dispatcher.
#[derive(Debug, Clone, PartialEq)]
pub enum OwnedWebCall {
    CreateElement { node: NodeId, tag: String },
    CreateTextNode { node: NodeId, text: String },
    AppendChild   { parent: NodeId, child: NodeId, index: u32 },
    RemoveChild   { parent: NodeId, child: NodeId },
    InsertBefore  { parent: NodeId, new_child: NodeId, ref_child: NodeId },
    SetAttribute  { node: NodeId, name: String, value: String },
    RemoveAttribute { node: NodeId, name: String },
    AddClass      { node: NodeId, class: String },
    SetInlineStyle{ node: NodeId, css: String },
    AnchorRect    { node: NodeId, rect: Rect },
}

impl OwnedWebCall {
    /// Borrow as a [`WebCall`]. Zero-copy.
    #[inline]
    pub fn as_web_call(&self) -> WebCall<'_> {
        match self {
            OwnedWebCall::CreateElement { node, tag } => WebCall::CreateElement {
                node: *node,
                tag:  tag.as_str(),
            },
            OwnedWebCall::CreateTextNode { node, text } => WebCall::CreateTextNode {
                node: *node,
                text: text.as_str(),
            },
            OwnedWebCall::AppendChild { parent, child, index } => WebCall::AppendChild {
                parent: *parent,
                child:  *child,
                index:  *index,
            },
            OwnedWebCall::RemoveChild { parent, child } => WebCall::RemoveChild {
                parent: *parent,
                child:  *child,
            },
            OwnedWebCall::InsertBefore { parent, new_child, ref_child } => WebCall::InsertBefore {
                parent: *parent,
                new_child: *new_child,
                ref_child: *ref_child,
            },
            OwnedWebCall::SetAttribute { node, name, value } => WebCall::SetAttribute {
                node:  *node,
                name:  name.as_str(),
                value: value.as_str(),
            },
            OwnedWebCall::RemoveAttribute { node, name } => WebCall::RemoveAttribute {
                node: *node,
                name: name.as_str(),
            },
            OwnedWebCall::AddClass { node, class } => WebCall::AddClass {
                node: *node,
                class: class.as_str(),
            },
            OwnedWebCall::SetInlineStyle { node, css } => WebCall::SetInlineStyle {
                node: *node,
                css:  css.as_str(),
            },
            OwnedWebCall::AnchorRect { node, rect } => WebCall::AnchorRect {
                node: *node,
                rect: *rect,
            },
        }
    }
}

/// A high-level Web API call at the bridge boundary. Borrows strings
/// from the JS runtime; the Dispatcher never owns them.
#[derive(Debug, Clone)]
pub enum WebCall<'a> {
    /// `document.createElement(tag)` — we track the node id explicitly
    /// so decomposition stays deterministic.
    CreateElement { node: NodeId, tag: &'a str },

    /// `document.createTextNode(text)` — create a text node.
    CreateTextNode { node: NodeId, text: &'a str },

    /// `parent.appendChild(child)` at a specific index.
    AppendChild { parent: NodeId, child: NodeId, index: u32 },

    /// `parent.removeChild(child)` — remove a child node.
    RemoveChild { parent: NodeId, child: NodeId },

    /// `parent.insertBefore(newChild, refChild)` — insert before reference.
    InsertBefore { parent: NodeId, new_child: NodeId, ref_child: NodeId },

    /// `element.setAttribute(name, value)` or `style.foo = bar`.
    SetAttribute { node: NodeId, name: &'a str, value: &'a str },

    /// `element.removeAttribute(name)` — remove an attribute.
    RemoveAttribute { node: NodeId, name: &'a str },

    /// `element.classList.add(class)` — add a CSS class.
    AddClass { node: NodeId, class: &'a str },

    /// A complete inline `style="..."` declaration.
    SetInlineStyle { node: NodeId, css: &'a str },

    /// Anchor `node` at an absolute viewport rect. Emitted by the
    /// layout-front-end for positioned elements.
    AnchorRect { node: NodeId, rect: Rect },
}

/// Post-normalization form. Free of parsing work; ready for
/// decomposition.
#[derive(Debug, Clone)]
pub enum NormalizedCall {
    CreateElement { node: NodeId, tag: Tag },
    AppendChild   { parent: NodeId, child: NodeId, index: u32 },
    Style         { node: NodeId, style: ResolvedStyle },
    AnchorRect    { node: NodeId, rect: Rect },
}

/// Tags that Slate recognizes natively. Anything else is a generic
/// container — no special-case HTML behaviors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tag {
    Div,
    Span,
    Text,
    Img,
    Generic,
}

impl Tag {
    pub fn from_str(s: &str) -> Self {
        match s {
            "div"  => Tag::Div,
            "span" => Tag::Span,
            "text" | "#text" => Tag::Text,
            "img"  => Tag::Img,
            _      => Tag::Generic,
        }
    }
}

/// A resolved style, post-parse. Every value is a fully-typed atom;
/// the decomposer does no string work.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ResolvedStyle {
    pub size:       Option<Size>,
    pub background: Option<Rgba8>,
    pub border:     Option<(Rgba8, f32)>,
    pub clip:       Option<Rect>,
}
