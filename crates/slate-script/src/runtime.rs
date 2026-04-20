//! The [`ScriptRuntime`] struct: one Boa context + the bridge.

use boa_engine::{Context, JsError, Source};
use slate_dispatcher::OwnedWebCall;

use crate::bridge;
use crate::host;

#[derive(Debug)]
pub enum ScriptError {
    Init(JsError),
    Eval(JsError),
}

impl std::fmt::Display for ScriptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScriptError::Init(e) => write!(f, "script init failed: {e}"),
            ScriptError::Eval(e) => write!(f, "script eval failed: {e}"),
        }
    }
}

impl std::error::Error for ScriptError {}

pub struct ScriptRuntime {
    ctx: Context,
}

impl ScriptRuntime {
    /// Build a fresh JS runtime with `slate.*` host functions installed.
    pub fn new() -> Result<Self, ScriptError> {
        let mut ctx = Context::default();
        host::install(&mut ctx).map_err(ScriptError::Init)?;
        // Reset bridge state so independent runtimes don't see each
        // other's ids or queued calls.
        bridge::reset();
        Ok(Self { ctx })
    }

    /// Evaluate a JS source fragment. Any `slate.*` calls it makes
    /// are queued; use [`ScriptRuntime::drain`] to collect them.
    pub fn eval(&mut self, src: &str) -> Result<(), ScriptError> {
        self.ctx
            .eval(Source::from_bytes(src.as_bytes()))
            .map(|_| ())
            .map_err(ScriptError::Eval)
    }

    /// Take ownership of the queued WebCalls.
    pub fn drain(&mut self) -> Vec<OwnedWebCall> {
        bridge::drain()
    }

    /// Clear the id counter and pending buffer — call on page nav.
    pub fn reset_ids(&mut self) { bridge::reset(); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_elements_from_js() {
        let mut rt = ScriptRuntime::new().unwrap();
        rt.eval(
            r#"
            const root  = slate.createElement('div');
            const child = slate.createElement('span');
            slate.appendChild(root, child, 0);
            slate.setStyle(root, 'width:100;height:50;background:red');
            "#,
        )
        .unwrap();

        let calls = rt.drain();
        assert_eq!(calls.len(), 4);
        assert!(matches!(calls[0], OwnedWebCall::CreateElement { .. }));
        assert!(matches!(calls[1], OwnedWebCall::CreateElement { .. }));
        assert!(matches!(calls[2], OwnedWebCall::AppendChild { .. }));
        assert!(matches!(calls[3], OwnedWebCall::SetInlineStyle { .. }));
    }
}
