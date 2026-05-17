//! Notifications API implementation.

use boa_engine::{Context, JsResult, JsValue, NativeFunction};

/// Notification API bindings.
pub struct NotificationApi;

impl NotificationApi {
    /// Install Notification API into JavaScript context.
    pub fn install(ctx: &mut Context) -> JsResult<()> {
        // Notification.requestPermission
        let request_permission = NativeFunction::from_fn_ptr(|_, _args, _ctx| {
            // TODO: Request notification permission
            // Return "granted", "denied", or "default"
            Ok(JsValue::from("default"))
        });

        ctx.register_global_property("__slate_notification_requestPermission", request_permission, Default::default())?;

        // new Notification()
        let notification_constructor = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let _title = args.get_or_undefined(0).to_string(ctx)?;
            let _options = args.get_or_undefined(1);
            
            // TODO: Show notification
            Ok(JsValue::undefined())
        });

        ctx.register_global_property("__slate_notification_constructor", notification_constructor, Default::default())?;

        Ok(())
    }
}
