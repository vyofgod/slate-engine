//! Web Storage API implementation (localStorage, sessionStorage).

use boa_engine::{Context, JsResult, JsValue, NativeFunction};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Storage API bindings.
pub struct StorageApi {
    local_storage: Arc<Mutex<HashMap<String, String>>>,
    session_storage: Arc<Mutex<HashMap<String, String>>>,
}

impl StorageApi {
    /// Create new Storage API.
    pub fn new() -> Self {
        Self {
            local_storage: Arc::new(Mutex::new(HashMap::new())),
            session_storage: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Install Storage API into JavaScript context.
    pub fn install(&self, ctx: &mut Context) -> JsResult<()> {
        self.install_storage(ctx, "localStorage", Arc::clone(&self.local_storage))?;
        self.install_storage(ctx, "sessionStorage", Arc::clone(&self.session_storage))?;
        Ok(())
    }

    fn install_storage(
        &self,
        ctx: &mut Context,
        name: &str,
        storage: Arc<Mutex<HashMap<String, String>>>,
    ) -> JsResult<()> {
        // setItem
        let storage_clone = Arc::clone(&storage);
        let set_item = NativeFunction::from_fn_ptr(move |_, args, ctx| {
            let key = args.get_or_undefined(0).to_string(ctx)?.to_std_string_escaped();
            let value = args.get_or_undefined(1).to_string(ctx)?.to_std_string_escaped();
            
            storage_clone.lock().unwrap().insert(key, value);
            Ok(JsValue::undefined())
        });

        ctx.register_global_property(
            format!("__slate_{}_setItem", name),
            set_item,
            Default::default(),
        )?;

        // getItem
        let storage_clone = Arc::clone(&storage);
        let get_item = NativeFunction::from_fn_ptr(move |_, args, ctx| {
            let key = args.get_or_undefined(0).to_string(ctx)?.to_std_string_escaped();
            
            let value = storage_clone
                .lock()
                .unwrap()
                .get(&key)
                .cloned();
            
            Ok(value.map(JsValue::from).unwrap_or(JsValue::null()))
        });

        ctx.register_global_property(
            format!("__slate_{}_getItem", name),
            get_item,
            Default::default(),
        )?;

        // removeItem
        let storage_clone = Arc::clone(&storage);
        let remove_item = NativeFunction::from_fn_ptr(move |_, args, ctx| {
            let key = args.get_or_undefined(0).to_string(ctx)?.to_std_string_escaped();
            
            storage_clone.lock().unwrap().remove(&key);
            Ok(JsValue::undefined())
        });

        ctx.register_global_property(
            format!("__slate_{}_removeItem", name),
            remove_item,
            Default::default(),
        )?;

        // clear
        let storage_clone = Arc::clone(&storage);
        let clear = NativeFunction::from_fn_ptr(move |_, _args, _ctx| {
            storage_clone.lock().unwrap().clear();
            Ok(JsValue::undefined())
        });

        ctx.register_global_property(
            format!("__slate_{}_clear", name),
            clear,
            Default::default(),
        )?;

        Ok(())
    }
}

impl Default for StorageApi {
    fn default() -> Self {
        Self::new()
    }
}
