#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaKeyStatusMap` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyStatusMap`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaKeyStatusMap(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyStatusMap`*"]
impl MediaKeyStatusMap {
    #[js::import = "$0.size"]
pub fn size(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyStatusMap`*"]
impl MediaKeyStatusMap {
    #[js::import = "$0.entries()"]
pub fn entries(&self) -> ::js_sys::Iterator { panic!("web-sys intrinsic") }
}
    #[doc = "The `forEach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/forEach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyStatusMap`*"]
impl MediaKeyStatusMap {
    #[js::import = "$0.forEach($1)"]
pub fn for_each(&self, callback: &::js_sys::Function) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyStatusMap`*"]
impl MediaKeyStatusMap {
    #[js::import = "$0.get($1)"]
pub fn get_with_buffer_source(&self, key_id: &::js_sys::Object) -> Result<::wasm_bindgen::JsValue, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyStatusMap`*"]
impl MediaKeyStatusMap {
    #[js::import = "$0.get($1)"]
pub fn get_with_u8_array(&self, key_id: &mut [u8]) -> Result<::wasm_bindgen::JsValue, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyStatusMap`*"]
impl MediaKeyStatusMap {
    #[js::import = "$0.get($1)"]
pub fn get_with_js_u8_array(&self, key_id: &::js_sys::Uint8Array) -> Result<::wasm_bindgen::JsValue, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `has()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/has)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyStatusMap`*"]
impl MediaKeyStatusMap {
    #[js::import = "$0.has($1)"]
pub fn has_with_buffer_source(&self, key_id: &::js_sys::Object) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `has()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/has)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyStatusMap`*"]
impl MediaKeyStatusMap {
    #[js::import = "$0.has($1)"]
pub fn has_with_u8_array(&self, key_id: &mut [u8]) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `has()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/has)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyStatusMap`*"]
impl MediaKeyStatusMap {
    #[js::import = "$0.has($1)"]
pub fn has_with_js_u8_array(&self, key_id: &::js_sys::Uint8Array) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyStatusMap`*"]
impl MediaKeyStatusMap {
    #[js::import = "$0.keys()"]
pub fn keys(&self) -> ::js_sys::Iterator { panic!("web-sys intrinsic") }
}
    #[doc = "The `values()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaKeyStatusMap/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaKeyStatusMap`*"]
impl MediaKeyStatusMap {
    #[js::import = "$0.values()"]
pub fn values(&self) -> ::js_sys::Iterator { panic!("web-sys intrinsic") }
}
