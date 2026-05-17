//! Geolocation API implementation.

use boa_engine::{Context, JsResult, JsValue, NativeFunction};

/// Geolocation API bindings.
pub struct GeolocationApi;

impl GeolocationApi {
    /// Install Geolocation API into JavaScript context.
    pub fn install(ctx: &mut Context) -> JsResult<()> {
        // navigator.geolocation.getCurrentPosition
        let get_current_position = NativeFunction::from_fn_ptr(|_, _args, _ctx| {
            // TODO: Get current position and call success callback
            Ok(JsValue::undefined())
        });

        ctx.register_global_property("__slate_geolocation_getCurrentPosition", get_current_position, Default::default())?;

        // navigator.geolocation.watchPosition
        let watch_position = NativeFunction::from_fn_ptr(|_, _args, _ctx| {
            // TODO: Watch position and return watch ID
            Ok(JsValue::from(1))
        });

        ctx.register_global_property("__slate_geolocation_watchPosition", watch_position, Default::default())?;

        // navigator.geolocation.clearWatch
        let clear_watch = NativeFunction::from_fn_ptr(|_, _args, _ctx| {
            // TODO: Clear position watch
            Ok(JsValue::undefined())
        });

        ctx.register_global_property("__slate_geolocation_clearWatch", clear_watch, Default::default())?;

        Ok(())
    }
}
