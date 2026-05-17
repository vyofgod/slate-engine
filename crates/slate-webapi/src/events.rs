//! Event API implementation.

use boa_engine::{Context, JsResult, JsValue, NativeFunction};

/// Event API bindings.
pub struct EventApi;

impl EventApi {
    /// Install Event API into JavaScript context.
    pub fn install(ctx: &mut Context) -> JsResult<()> {
        // addEventListener
        let add_event_listener = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let _node = args.get_or_undefined(0).to_number(ctx)? as u32;
            let _event_type = args.get_or_undefined(1).to_string(ctx)?;
            let _callback = args.get_or_undefined(2);
            let _options = args.get_or_undefined(3);
            
            // TODO: Register event listener with slate-events
            Ok(JsValue::undefined())
        });

        ctx.register_global_property("__slate_addEventListener", add_event_listener, Default::default())?;

        // removeEventListener
        let remove_event_listener = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let _node = args.get_or_undefined(0).to_number(ctx)? as u32;
            let _event_type = args.get_or_undefined(1).to_string(ctx)?;
            let _callback = args.get_or_undefined(2);
            
            // TODO: Remove event listener
            Ok(JsValue::undefined())
        });

        ctx.register_global_property("__slate_removeEventListener", remove_event_listener, Default::default())?;

        // dispatchEvent
        let dispatch_event = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let _node = args.get_or_undefined(0).to_number(ctx)? as u32;
            let _event = args.get_or_undefined(1);
            
            // TODO: Dispatch event through slate-events
            Ok(JsValue::from(true))
        });

        ctx.register_global_property("__slate_dispatchEvent", dispatch_event, Default::default())?;

        Ok(())
    }
}
