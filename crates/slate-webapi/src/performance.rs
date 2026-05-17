//! Performance API implementation.

use boa_engine::{Context, JsResult, JsValue, NativeFunction};
use std::time::Instant;

/// Performance API bindings.
pub struct PerformanceApi {
    start_time: Instant,
}

impl PerformanceApi {
    /// Create new Performance API.
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
        }
    }

    /// Install Performance API into JavaScript context.
    pub fn install(&self, ctx: &mut Context) -> JsResult<()> {
        let start_time = self.start_time;

        // performance.now()
        let now = NativeFunction::from_fn_ptr(move |_, _args, _ctx| {
            let elapsed = start_time.elapsed();
            let ms = elapsed.as_secs_f64() * 1000.0;
            Ok(JsValue::from(ms))
        });

        ctx.register_global_property("__slate_performance_now", now, Default::default())?;

        // performance.mark()
        let mark = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let _name = args.get_or_undefined(0).to_string(ctx)?;
            // TODO: Store performance mark
            Ok(JsValue::undefined())
        });

        ctx.register_global_property("__slate_performance_mark", mark, Default::default())?;

        // performance.measure()
        let measure = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let _name = args.get_or_undefined(0).to_string(ctx)?;
            let _start_mark = args.get_or_undefined(1);
            let _end_mark = args.get_or_undefined(2);
            // TODO: Calculate and store performance measure
            Ok(JsValue::undefined())
        });

        ctx.register_global_property("__slate_performance_measure", measure, Default::default())?;

        Ok(())
    }
}

impl Default for PerformanceApi {
    fn default() -> Self {
        Self::new()
    }
}
