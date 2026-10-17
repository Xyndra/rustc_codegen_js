#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WorkletOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkletOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct WorkletOptions(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "RequestCredentials")]
    #[doc = "Get the `credentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestCredentials`, `WorkletOptions`*"]
#[js::import = "None"]
pub fn get_credentials(this: &WorkletOptions) -> Option<RequestCredentials> { panic!("web-sys intrinsic") }
    #[cfg(feature = "RequestCredentials")]
    #[doc = "Change the `credentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestCredentials`, `WorkletOptions`*"]
#[js::import = "None = $1"]
pub fn set_credentials(this: &WorkletOptions, val: RequestCredentials) { panic!("web-sys intrinsic") }
impl WorkletOptions {
    #[doc = "Construct a new `WorkletOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkletOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(feature = "RequestCredentials")]
    #[deprecated = "Use `set_credentials()` instead."]
#[js::import = "None"]
pub fn credentials(&mut self, val: RequestCredentials) -> &mut Self { self.set_credentials(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for WorkletOptions {
    fn default() -> Self {
        Self::new()
    }
}
