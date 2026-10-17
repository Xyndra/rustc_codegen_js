#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Headers` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Headers`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Headers(pub ::wasm_bindgen::JsValue);
    #[doc = "The `new Headers(..)` constructor, creating a new instance of `Headers`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers/Headers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Headers`*"]
impl Headers {
    #[js::import = "new Headers()"]
pub fn new() -> Result<Headers, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new Headers(..)` constructor, creating a new instance of `Headers`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers/Headers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Headers`*"]
impl Headers {
    #[js::import = "new Headers($0)"]
pub fn new_with_headers(init: &Headers) -> Result<Headers, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new Headers(..)` constructor, creating a new instance of `Headers`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers/Headers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Headers`*"]
impl Headers {
    #[js::import = "new Headers($0)"]
pub fn new_with_str_sequence_sequence(init: &::wasm_bindgen::JsValue,) -> Result<Headers, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new Headers(..)` constructor, creating a new instance of `Headers`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers/Headers)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Headers`*"]
impl Headers {
    #[js::import = "new Headers($0)"]
pub fn new_with_record_from_str_to_str(init: &::js_sys::Object) -> Result<Headers, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Headers`*"]
impl Headers {
    #[js::import = "$0.append($1, $2)"]
pub fn append(&self, name: &str, value: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `delete()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers/delete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Headers`*"]
impl Headers {
    #[js::import = "$0.delete($1)"]
pub fn delete(&self, name: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Headers`*"]
impl Headers {
    #[js::import = "$0.entries()"]
pub fn entries(&self) -> ::js_sys::Iterator { panic!("web-sys intrinsic") }
}
    #[doc = "The `forEach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers/forEach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Headers`*"]
impl Headers {
    #[js::import = "$0.forEach($1)"]
pub fn for_each(&self, callback: &::js_sys::Function) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Headers`*"]
impl Headers {
    #[js::import = "$0.get($1)"]
pub fn get(&self, name: &str) -> Result<Option<::alloc::string::String>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `has()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers/has)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Headers`*"]
impl Headers {
    #[js::import = "$0.has($1)"]
pub fn has(&self, name: &str) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Headers`*"]
impl Headers {
    #[js::import = "$0.keys()"]
pub fn keys(&self) -> ::js_sys::Iterator { panic!("web-sys intrinsic") }
}
    #[doc = "The `set()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers/set)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Headers`*"]
impl Headers {
    #[js::import = "$0.set($1, $2)"]
pub fn set(&self, name: &str, value: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `values()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Headers/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Headers`*"]
impl Headers {
    #[js::import = "$0.values()"]
pub fn values(&self) -> ::js_sys::Iterator { panic!("web-sys intrinsic") }
}
