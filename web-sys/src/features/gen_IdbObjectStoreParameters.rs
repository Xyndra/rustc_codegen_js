#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbObjectStoreParameters` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStoreParameters`*"]
#[js::opaque]
#[derive(Copy)]
pub struct IdbObjectStoreParameters(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `autoIncrement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStoreParameters`*"]
#[js::import = "None"]
pub fn get_auto_increment(this: &IdbObjectStoreParameters) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `autoIncrement` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStoreParameters`*"]
#[js::import = "None = $1"]
pub fn set_auto_increment(this: &IdbObjectStoreParameters, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `keyPath` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStoreParameters`*"]
#[js::import = "None"]
pub fn get_key_path(this: &IdbObjectStoreParameters) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
    #[doc = "Change the `keyPath` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStoreParameters`*"]
#[js::import = "None = $1"]
pub fn set_key_path(this: &IdbObjectStoreParameters, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
impl IdbObjectStoreParameters {
    #[doc = "Construct a new `IdbObjectStoreParameters`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbObjectStoreParameters`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_auto_increment()` instead."]
#[js::import = "None"]
pub fn auto_increment(&mut self, val: bool) -> &mut Self { self.set_auto_increment(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_key_path()` instead."]
#[js::import = "None"]
pub fn key_path(&mut self, val: Option<&::wasm_bindgen::JsValue>) -> &mut Self { self.set_key_path(val.unwrap_or(&::wasm_bindgen::JsValue::NULL)) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for IdbObjectStoreParameters {
    fn default() -> Self {
        Self::new()
    }
}
