//! Console API implementation.
//!
//! Implements the Console API for debugging and logging.

use boa_engine::{Context, JsResult, JsValue, NativeFunction};

/// Console API bindings.
pub struct ConsoleApi;

impl ConsoleApi {
    /// Install Console API into JavaScript context.
    pub fn install(ctx: &mut Context) -> JsResult<()> {
        // console.log
        let log = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let messages: Vec<String> = args
                .iter()
                .map(|v| v.to_string(ctx).map(|s| s.to_std_string_escaped()))
                .collect::<Result<_, _>>()?;
            
            println!("[LOG] {}", messages.join(" "));
            Ok(JsValue::undefined())
        });

        ctx.register_global_property("__slate_console_log", log, Default::default())?;

        // console.error
        let error = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let messages: Vec<String> = args
                .iter()
                .map(|v| v.to_string(ctx).map(|s| s.to_std_string_escaped()))
                .collect::<Result<_, _>>()?;
            
            eprintln!("[ERROR] {}", messages.join(" "));
            Ok(JsValue::undefined())
        });

        ctx.register_global_property("__slate_console_error", error, Default::default())?;

        // console.warn
        let warn = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let messages: Vec<String> = args
                .iter()
                .map(|v| v.to_string(ctx).map(|s| s.to_std_string_escaped()))
                .collect::<Result<_, _>>()?;
            
            println!("[WARN] {}", messages.join(" "));
            Ok(JsValue::undefined())
        });

        ctx.register_global_property("__slate_console_warn", warn, Default::default())?;

        // console.info
        let info = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let messages: Vec<String> = args
                .iter()
                .map(|v| v.to_string(ctx).map(|s| s.to_std_string_escaped()))
                .collect::<Result<_, _>>()?;
            
            println!("[INFO] {}", messages.join(" "));
            Ok(JsValue::undefined())
        });

        ctx.register_global_property("__slate_console_info", info, Default::default())?;

        // console.debug
        let debug = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let messages: Vec<String> = args
                .iter()
                .map(|v| v.to_string(ctx).map(|s| s.to_std_string_escaped()))
                .collect::<Result<_, _>>()?;
            
            println!("[DEBUG] {}", messages.join(" "));
            Ok(JsValue::undefined())
        });

        ctx.register_global_property("__slate_console_debug", debug, Default::default())?;

        // console.trace
        let trace = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let messages: Vec<String> = args
                .iter()
                .map(|v| v.to_string(ctx).map(|s| s.to_std_string_escaped()))
                .collect::<Result<_, _>>()?;
            
            println!("[TRACE] {}", messages.join(" "));
            // TODO: Add stack trace
            Ok(JsValue::undefined())
        });

        ctx.register_global_property("__slate_console_trace", trace, Default::default())?;

        // console.assert
        let assert = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let condition = args.get_or_undefined(0).to_boolean();
            
            if !condition {
                let messages: Vec<String> = args
                    .iter()
                    .skip(1)
                    .map(|v| v.to_string(ctx).map(|s| s.to_std_string_escaped()))
                    .collect::<Result<_, _>>()?;
                
                eprintln!("[ASSERT] Assertion failed: {}", messages.join(" "));
            }
            
            Ok(JsValue::undefined())
        });

        ctx.register_global_property("__slate_console_assert", assert, Default::default())?;

        // console.clear
        let clear = NativeFunction::from_fn_ptr(|_, _args, _ctx| {
            print!("\x1B[2J\x1B[1;1H"); // ANSI clear screen
            Ok(JsValue::undefined())
        });

        ctx.register_global_property("__slate_console_clear", clear, Default::default())?;

        // console.count
        use std::collections::HashMap;
        use std::sync::Mutex;
        
        let counters = std::sync::Arc::new(Mutex::new(HashMap::<String, u32>::new()));
        
        let count = NativeFunction::from_fn_ptr(move |_, args, ctx| {
            let label = args
                .get_or_undefined(0)
                .to_string(ctx)?
                .to_std_string_escaped();
            
            let mut counters = counters.lock().unwrap();
            let count = counters.entry(label.clone()).or_insert(0);
            *count += 1;
            
            println!("[COUNT] {}: {}", label, count);
            Ok(JsValue::undefined())
        });

        ctx.register_global_property("__slate_console_count", count, Default::default())?;

        // console.time / console.timeEnd
        use std::time::Instant;
        
        let timers = std::sync::Arc::new(Mutex::new(HashMap::<String, Instant>::new()));
        
        let timers_clone = timers.clone();
        let time = NativeFunction::from_fn_ptr(move |_, args, ctx| {
            let label = args
                .get_or_undefined(0)
                .to_string(ctx)?
                .to_std_string_escaped();
            
            timers_clone.lock().unwrap().insert(label, Instant::now());
            Ok(JsValue::undefined())
        });

        ctx.register_global_property("__slate_console_time", time, Default::default())?;

        let time_end = NativeFunction::from_fn_ptr(move |_, args, ctx| {
            let label = args
                .get_or_undefined(0)
                .to_string(ctx)?
                .to_std_string_escaped();
            
            if let Some(start) = timers.lock().unwrap().remove(&label) {
                let elapsed = start.elapsed();
                println!("[TIME] {}: {:?}", label, elapsed);
            }
            
            Ok(JsValue::undefined())
        });

        ctx.register_global_property("__slate_console_timeEnd", time_end, Default::default())?;

        Ok(())
    }
}
