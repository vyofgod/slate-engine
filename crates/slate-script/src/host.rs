//! Host functions exposed to JS.
//!
//! The surface is intentionally tiny: `slate.createElement(tag)`,
//! `slate.appendChild(parent, child)`, `slate.setStyle(node, css)`.
//! Each returns a node id (integer). No prototype chains. No
//! observable `document` object.
//!
//! This is the "API Elimination" principle enforced at the JS
//! boundary. The richer `document.*` surface, if it is ever added,
//! will be a pure JS shim on top of these three primitives — written
//! in JS and bundled with the runtime, not implemented in Rust.

use boa_engine::{Context, JsArgs, JsResult, JsValue, NativeFunction};

use slate_dispatcher::OwnedWebCall;

use crate::bridge::{alloc_id, push};

fn js_to_str(v: &JsValue, ctx: &mut Context) -> String {
    v.to_string(ctx)
        .map(|s| s.to_std_string_escaped())
        .unwrap_or_default()
}

fn js_to_u32(v: &JsValue, ctx: &mut Context) -> u32 {
    v.to_u32(ctx).unwrap_or(0)
}

pub(crate) fn create_element(
    _: &JsValue,
    args: &[JsValue],
    ctx: &mut Context,
) -> JsResult<JsValue> {
    let tag = js_to_str(args.get_or_undefined(0), ctx);
    let id = alloc_id();
    push(OwnedWebCall::CreateElement { node: id, tag });
    Ok(JsValue::new(id.0))
}

pub(crate) fn append_child(
    _: &JsValue,
    args: &[JsValue],
    ctx: &mut Context,
) -> JsResult<JsValue> {
    let parent = js_to_u32(args.get_or_undefined(0), ctx);
    let child  = js_to_u32(args.get_or_undefined(1), ctx);
    let index  = js_to_u32(args.get_or_undefined(2), ctx);
    push(OwnedWebCall::AppendChild {
        parent: slate_ais::NodeId(parent),
        child:  slate_ais::NodeId(child),
        index,
    });
    Ok(JsValue::undefined())
}

pub(crate) fn set_style(
    _: &JsValue,
    args: &[JsValue],
    ctx: &mut Context,
) -> JsResult<JsValue> {
    let node = js_to_u32(args.get_or_undefined(0), ctx);
    let css  = js_to_str(args.get_or_undefined(1), ctx);
    push(OwnedWebCall::SetInlineStyle {
        node: slate_ais::NodeId(node),
        css,
    });
    Ok(JsValue::undefined())
}

/// Register the three host functions under a `slate` global.
pub(crate) fn install(ctx: &mut Context) -> JsResult<()> {
    // slate.createElement(tag) -> id
    ctx.register_global_callable(
        boa_engine::js_string!("__slate_create_element"),
        1,
        NativeFunction::from_fn_ptr(create_element),
    )?;
    ctx.register_global_callable(
        boa_engine::js_string!("__slate_append_child"),
        3,
        NativeFunction::from_fn_ptr(append_child),
    )?;
    ctx.register_global_callable(
        boa_engine::js_string!("__slate_set_style"),
        2,
        NativeFunction::from_fn_ptr(set_style),
    )?;

    // Tiny JS shim that mimics the `slate.*` surface. Nothing
    // magical — just a regular JS object. Written in JS on purpose:
    // if a developer reads the runtime, the mapping from JS to
    // WebCall is in plain sight.
    ctx.eval(boa_engine::Source::from_bytes(
        br#"
        globalThis.slate = {
            createElement(tag) { return __slate_create_element(String(tag)); },
            appendChild(parent, child, index)  {
                __slate_append_child(parent|0, child|0, (index|0));
            },
            setStyle(node, css) { __slate_set_style(node|0, String(css)); },
        };
        "#,
    ))?;

    Ok(())
}
