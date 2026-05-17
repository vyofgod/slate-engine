//! Timer APIs (setTimeout, setInterval, requestAnimationFrame).

use boa_engine::{Context, JsResult, JsValue, NativeFunction};

/// Timer API bindings.
pub struct TimerApi;

impl TimerApi {
    /// Install Timer API into JavaScript context.
    pub fn install(ctx: &mut Context) -> JsResult<()> {
        // setTimeout
        let set_timeout = NativeFunction::from_fn_ptr(|_, args, _ctx| {
            let _callback = args.get_or_undefined(0);
            let _delay = args.get_or_undefined(1).to_number(_ctx).unwrap_or(0.0);
            
            // TODO: Schedule callback execution
            // Return timer ID
            Ok(JsValue::from(1))
        });

        ctx.register_global_property("setTimeout", set_timeout, Default::default())?;

        // clearTimeout
        let clear_timeout = NativeFunction::from_fn_ptr(|_, _args, _ctx| {
            // TODO: Cancel timer
            Ok(JsValue::undefined())
        });

        ctx.register_global_property("clearTimeout", clear_timeout, Default::default())?;

        // setInterval
        let set_interval = NativeFunction::from_fn_ptr(|_, args, _ctx| {
            let _callback = args.get_or_undefined(0);
            let _delay = args.get_or_undefined(1).to_number(_ctx).unwrap_or(0.0);
            
            // TODO: Schedule repeating callback
            Ok(JsValue::from(1))
        });

        ctx.register_global_property("setInterval", set_interval, Default::default())?;

        // clearInterval
        let clear_interval = NativeFunction::from_fn_ptr(|_, _args, _ctx| {
            // TODO: Cancel interval
            Ok(JsValue::undefined())
        });

        ctx.register_global_property("clearInterval", clear_interval, Default::default())?;

        // requestAnimationFrame
        let request_animation_frame = NativeFunction::from_fn_ptr(|_, _args, _ctx| {
            // TODO: Schedule callback for next frame
            Ok(JsValue::from(1))
        });

        ctx.register_global_property("requestAnimationFrame", request_animation_frame, Default::default())?;

        // cancelAnimationFrame
        let cancel_animation_frame = NativeFunction::from_fn_ptr(|_, _args, _ctx| {
            // TODO: Cancel animation frame
            Ok(JsValue::undefined())
        });

        ctx.register_global_property("cancelAnimationFrame", cancel_animation_frame, Default::default())?;

        Ok(())
    }
}
