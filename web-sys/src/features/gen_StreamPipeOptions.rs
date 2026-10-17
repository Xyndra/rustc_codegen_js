#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `StreamPipeOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct StreamPipeOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `preventAbort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"]
#[js::import = "None"]
pub fn get_prevent_abort(this: &StreamPipeOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `preventAbort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"]
#[js::import = "None = $1"]
pub fn set_prevent_abort(this: &StreamPipeOptions, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `preventCancel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"]
#[js::import = "None"]
pub fn get_prevent_cancel(this: &StreamPipeOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `preventCancel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"]
#[js::import = "None = $1"]
pub fn set_prevent_cancel(this: &StreamPipeOptions, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `preventClose` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"]
#[js::import = "None"]
pub fn get_prevent_close(this: &StreamPipeOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `preventClose` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"]
#[js::import = "None = $1"]
pub fn set_prevent_close(this: &StreamPipeOptions, val: bool) { panic!("web-sys intrinsic") }
    #[cfg(feature = "AbortSignal")]
    #[doc = "Get the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `StreamPipeOptions`*"]
#[js::import = "None"]
pub fn get_signal(this: &StreamPipeOptions) -> Option<AbortSignal> { panic!("web-sys intrinsic") }
    #[cfg(feature = "AbortSignal")]
    #[doc = "Change the `signal` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AbortSignal`, `StreamPipeOptions`*"]
#[js::import = "None = $1"]
pub fn set_signal(this: &StreamPipeOptions, val: &AbortSignal) { panic!("web-sys intrinsic") }
impl StreamPipeOptions {
    #[doc = "Construct a new `StreamPipeOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `StreamPipeOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_prevent_abort()` instead."]
#[js::import = "None"]
pub fn prevent_abort(&mut self, val: bool) -> &mut Self { self.set_prevent_abort(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_prevent_cancel()` instead."]
#[js::import = "None"]
pub fn prevent_cancel(&mut self, val: bool) -> &mut Self { self.set_prevent_cancel(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_prevent_close()` instead."]
#[js::import = "None"]
pub fn prevent_close(&mut self, val: bool) -> &mut Self { self.set_prevent_close(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "AbortSignal")]
    #[deprecated = "Use `set_signal()` instead."]
#[js::import = "None"]
pub fn signal(&mut self, val: &AbortSignal) -> &mut Self { self.set_signal(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for StreamPipeOptions {
    fn default() -> Self {
        Self::new()
    }
}
