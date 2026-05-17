//! URL API implementation.

use boa_engine::{Context, JsResult, JsValue, NativeFunction};

/// URL API bindings.
pub struct UrlApi;

impl UrlApi {
    /// Install URL API into JavaScript context.
    pub fn install(ctx: &mut Context) -> JsResult<()> {
        // URL constructor
        let url_constructor = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let _url = args.get_or_undefined(0).to_string(ctx)?;
            let _base = args.get_or_undefined(1);
            
            // TODO: Parse URL and return URL object
            Ok(JsValue::undefined())
        });

        ctx.register_global_property("URL", url_constructor, Default::default())?;

        // URLSearchParams constructor
        let url_search_params = NativeFunction::from_fn_ptr(|_, _args, _ctx| {
            // TODO: Parse query string
            Ok(JsValue::undefined())
        });

        ctx.register_global_property("URLSearchParams", url_search_params, Default::default())?;

        Ok(())
    }
}
