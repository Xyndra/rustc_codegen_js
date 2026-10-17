#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UrlSearchParams` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UrlSearchParams`*"]
#[js::opaque]
#[derive(Copy)]
pub struct UrlSearchParams(pub ::wasm_bindgen::JsValue);
    #[doc = "The `new UrlSearchParams(..)` constructor, creating a new instance of `UrlSearchParams`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/URLSearchParams)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UrlSearchParams`*"]
impl UrlSearchParams {
    #[js::import = "new URLSearchParams()"]
pub fn new() -> Result<UrlSearchParams, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new UrlSearchParams(..)` constructor, creating a new instance of `UrlSearchParams`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/URLSearchParams)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UrlSearchParams`*"]
impl UrlSearchParams {
    #[js::import = "new URLSearchParams($0)"]
pub fn new_with_str_sequence_sequence(init: &::wasm_bindgen::JsValue,) -> Result<UrlSearchParams, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new UrlSearchParams(..)` constructor, creating a new instance of `UrlSearchParams`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/URLSearchParams)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UrlSearchParams`*"]
impl UrlSearchParams {
    #[js::import = "new URLSearchParams($0)"]
pub fn new_with_record_from_str_to_str(init: &::js_sys::Object,) -> Result<UrlSearchParams, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new UrlSearchParams(..)` constructor, creating a new instance of `UrlSearchParams`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/URLSearchParams)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UrlSearchParams`*"]
impl UrlSearchParams {
    #[js::import = "new URLSearchParams($0)"]
pub fn new_with_str(init: &str) -> Result<UrlSearchParams, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UrlSearchParams`*"]
impl UrlSearchParams {
    #[js::import = "$0.append($1, $2)"]
pub fn append(&self, name: &str, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `delete()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/delete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UrlSearchParams`*"]
impl UrlSearchParams {
    #[js::import = "$0.delete($1)"]
pub fn delete(&self, name: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UrlSearchParams`*"]
impl UrlSearchParams {
    #[js::import = "$0.entries()"]
pub fn entries(&self) -> ::js_sys::Iterator { panic!("web-sys intrinsic") }
}
    #[doc = "The `forEach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/forEach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UrlSearchParams`*"]
impl UrlSearchParams {
    #[js::import = "$0.forEach($1)"]
pub fn for_each(&self, callback: &::js_sys::Function) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UrlSearchParams`*"]
impl UrlSearchParams {
    #[js::import = "$0.get($1)"]
pub fn get(&self, name: &str) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/getAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UrlSearchParams`*"]
impl UrlSearchParams {
    #[js::import = "$0.getAll($1)"]
pub fn get_all(&self, name: &str) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "The `has()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/has)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UrlSearchParams`*"]
impl UrlSearchParams {
    #[js::import = "$0.has($1)"]
pub fn has(&self, name: &str) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UrlSearchParams`*"]
impl UrlSearchParams {
    #[js::import = "$0.keys()"]
pub fn keys(&self) -> ::js_sys::Iterator { panic!("web-sys intrinsic") }
}
    #[doc = "The `set()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/set)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UrlSearchParams`*"]
impl UrlSearchParams {
    #[js::import = "$0.set($1, $2)"]
pub fn set(&self, name: &str, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `sort()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/sort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UrlSearchParams`*"]
impl UrlSearchParams {
    #[js::import = "$0.sort()"]
pub fn sort(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `values()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/URLSearchParams/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UrlSearchParams`*"]
impl UrlSearchParams {
    #[js::import = "$0.values()"]
pub fn values(&self) -> ::js_sys::Iterator { panic!("web-sys intrinsic") }
}
