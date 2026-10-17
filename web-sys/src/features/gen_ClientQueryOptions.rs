#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ClientQueryOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientQueryOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ClientQueryOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `includeUncontrolled` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientQueryOptions`*"]
#[js::import = "None"]
pub fn get_include_uncontrolled(this: &ClientQueryOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `includeUncontrolled` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientQueryOptions`*"]
#[js::import = "None = $1"]
pub fn set_include_uncontrolled(this: &ClientQueryOptions, val: bool) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ClientType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientQueryOptions`, `ClientType`*"]
#[js::import = "None"]
pub fn get_type(this: &ClientQueryOptions) -> Option<ClientType> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ClientType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientQueryOptions`, `ClientType`*"]
#[js::import = "None = $1"]
pub fn set_type(this: &ClientQueryOptions, val: ClientType) { panic!("web-sys intrinsic") }
impl ClientQueryOptions {
    #[doc = "Construct a new `ClientQueryOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ClientQueryOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_include_uncontrolled()` instead."]
#[js::import = "None"]
pub fn include_uncontrolled(&mut self, val: bool) -> &mut Self { self.set_include_uncontrolled(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "ClientType")]
    #[deprecated = "Use `set_type()` instead."]
#[js::import = "None"]
pub fn type_(&mut self, val: ClientType) -> &mut Self { self.set_type(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ClientQueryOptions {
    fn default() -> Self {
        Self::new()
    }
}
