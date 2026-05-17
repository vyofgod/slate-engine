//! Web API bindings coordinator.
//!
//! This module coordinates all Web API bindings and installs them
//! into the JavaScript runtime.

use boa_engine::{Context, JsResult};
use slate_dispatcher::OwnedWebCall;

use crate::canvas::CanvasApi;
use crate::console::ConsoleApi;
use crate::crypto::CryptoApi;
use crate::dom::DomApi;
use crate::events::EventApi;
use crate::fetch::FetchApi;
use crate::geolocation::GeolocationApi;
use crate::notification::NotificationApi;
use crate::performance::PerformanceApi;
use crate::storage::StorageApi;
use crate::timers::TimerApi;
use crate::url::UrlApi;
use crate::websocket::WebSocketApi;

/// Web API bindings coordinator.
pub struct WebApiBindings {
    dom: DomApi,
    storage: StorageApi,
    performance: PerformanceApi,
}

impl WebApiBindings {
    /// Create new Web API bindings.
    pub fn new() -> Self {
        Self {
            dom: DomApi::new(),
            storage: StorageApi::new(),
            performance: PerformanceApi::new(),
        }
    }

    /// Install all Web APIs into JavaScript context.
    pub fn install(&self, ctx: &mut Context) -> JsResult<()> {
        // Core DOM APIs
        self.dom.install(ctx)?;

        // Console API
        ConsoleApi::install(ctx)?;

        // Event APIs
        EventApi::install(ctx)?;

        // Timer APIs
        TimerApi::install(ctx)?;

        // Storage APIs
        self.storage.install(ctx)?;

        // Fetch API
        FetchApi::install(ctx)?;

        // URL API
        UrlApi::install(ctx)?;

        // Canvas API
        CanvasApi::install(ctx)?;

        // Crypto API
        CryptoApi::install(ctx)?;

        // Performance API
        self.performance.install(ctx)?;

        // Geolocation API
        GeolocationApi::install(ctx)?;

        // Notification API
        NotificationApi::install(ctx)?;

        // WebSocket API
        WebSocketApi::install(ctx)?;

        // Install polyfills for standard Web APIs
        self.install_polyfills(ctx)?;

        Ok(())
    }

    /// Install JavaScript polyfills that wrap the low-level bindings.
    fn install_polyfills(&self, ctx: &mut Context) -> JsResult<()> {
        // Document object polyfill
        let document_polyfill = r#"
            const document = {
                createElement: function(tag) {
                    const node = __slate_createElement(tag);
                    return {
                        _nodeId: node,
                        appendChild: function(child) {
                            __slate_appendChild(this._nodeId, child._nodeId);
                            return child;
                        },
                        removeChild: function(child) {
                            __slate_removeChild(this._nodeId, child._nodeId);
                            return child;
                        },
                        insertBefore: function(newChild, refChild) {
                            __slate_insertBefore(this._nodeId, newChild._nodeId, refChild._nodeId);
                            return newChild;
                        },
                        setAttribute: function(name, value) {
                            __slate_setAttribute(this._nodeId, name, value);
                        },
                        getAttribute: function(name) {
                            return __slate_getAttribute(this._nodeId, name);
                        },
                        removeAttribute: function(name) {
                            __slate_removeAttribute(this._nodeId, name);
                        },
                        addEventListener: function(type, callback, options) {
                            __slate_addEventListener(this._nodeId, type, callback, options);
                        },
                        removeEventListener: function(type, callback) {
                            __slate_removeEventListener(this._nodeId, type, callback);
                        },
                        get style() {
                            const nodeId = this._nodeId;
                            return {
                                setProperty: function(prop, value) {
                                    __slate_setStyle(nodeId, prop + ':' + value);
                                }
                            };
                        },
                        get classList() {
                            const nodeId = this._nodeId;
                            return {
                                add: function(className) {
                                    __slate_addClass(nodeId, className);
                                }
                            };
                        }
                    };
                },
                createTextNode: function(text) {
                    const node = __slate_createTextNode(text);
                    return {
                        _nodeId: node
                    };
                },
                getElementById: function(id) {
                    return __slate_getElementById(id);
                },
                querySelector: function(selector) {
                    return __slate_querySelector(selector);
                },
                querySelectorAll: function(selector) {
                    return __slate_querySelectorAll(selector);
                }
            };
        "#;

        ctx.eval(boa_engine::Source::from_bytes(document_polyfill.as_bytes()))?;

        // Console object polyfill
        let console_polyfill = r#"
            const console = {
                log: __slate_console_log,
                error: __slate_console_error,
                warn: __slate_console_warn,
                info: __slate_console_info,
                debug: __slate_console_debug,
                trace: __slate_console_trace,
                assert: __slate_console_assert,
                clear: __slate_console_clear,
                count: __slate_console_count,
                time: __slate_console_time,
                timeEnd: __slate_console_timeEnd
            };
        "#;

        ctx.eval(boa_engine::Source::from_bytes(console_polyfill.as_bytes()))?;

        // Window object polyfill
        let window_polyfill = r#"
            const window = {
                document: document,
                console: console,
                setTimeout: setTimeout,
                clearTimeout: clearTimeout,
                setInterval: setInterval,
                clearInterval: clearInterval,
                requestAnimationFrame: requestAnimationFrame,
                cancelAnimationFrame: cancelAnimationFrame,
                fetch: fetch,
                localStorage: {
                    setItem: function(key, value) {
                        __slate_localStorage_setItem(key, value);
                    },
                    getItem: function(key) {
                        return __slate_localStorage_getItem(key);
                    },
                    removeItem: function(key) {
                        __slate_localStorage_removeItem(key);
                    },
                    clear: function() {
                        __slate_localStorage_clear();
                    }
                },
                sessionStorage: {
                    setItem: function(key, value) {
                        __slate_sessionStorage_setItem(key, value);
                    },
                    getItem: function(key) {
                        return __slate_sessionStorage_getItem(key);
                    },
                    removeItem: function(key) {
                        __slate_sessionStorage_removeItem(key);
                    },
                    clear: function() {
                        __slate_sessionStorage_clear();
                    }
                },
                performance: {
                    now: __slate_performance_now,
                    mark: __slate_performance_mark,
                    measure: __slate_performance_measure
                },
                crypto: {
                    getRandomValues: __slate_crypto_getRandomValues,
                    randomUUID: __slate_crypto_randomUUID
                }
            };
            
            // Make window global
            const self = window;
            const globalThis = window;
        "#;

        ctx.eval(boa_engine::Source::from_bytes(window_polyfill.as_bytes()))?;

        Ok(())
    }

    /// Take all pending WebCalls from DOM operations.
    pub fn take_web_calls(&self) -> Vec<OwnedWebCall> {
        self.dom.take_web_calls()
    }
}

impl Default for WebApiBindings {
    fn default() -> Self {
        Self::new()
    }
}
