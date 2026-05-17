//! Web Crypto API implementation.

use boa_engine::{Context, JsResult, JsValue, NativeFunction};

/// Crypto API bindings.
pub struct CryptoApi;

impl CryptoApi {
    /// Install Crypto API into JavaScript context.
    pub fn install(ctx: &mut Context) -> JsResult<()> {
        // crypto.getRandomValues
        let get_random_values = NativeFunction::from_fn_ptr(|_, _args, _ctx| {
            // TODO: Fill typed array with random values
            Ok(JsValue::undefined())
        });

        ctx.register_global_property("__slate_crypto_getRandomValues", get_random_values, Default::default())?;

        // crypto.randomUUID
        let random_uuid = NativeFunction::from_fn_ptr(|_, _args, _ctx| {
            use std::fmt::Write;
            let mut uuid = String::with_capacity(36);
            let bytes: [u8; 16] = rand::random();
            
            write!(&mut uuid, "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
                bytes[0], bytes[1], bytes[2], bytes[3],
                bytes[4], bytes[5],
                bytes[6], bytes[7],
                bytes[8], bytes[9],
                bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]
            ).unwrap();
            
            Ok(JsValue::from(uuid))
        });

        ctx.register_global_property("__slate_crypto_randomUUID", random_uuid, Default::default())?;

        Ok(())
    }
}
