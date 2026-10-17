#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FormData` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FormData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FormData`*"]
#[js::opaque]
#[derive(Copy)]
pub struct FormData(pub ::wasm_bindgen::JsValue);
    #[doc = "The `new FormData(..)` constructor, creating a new instance of `FormData`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FormData/FormData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FormData`*"]
impl FormData {
    #[js::import = "new FormData()"]
pub fn new() -> Result<FormData, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlFormElement")]
    #[doc = "The `new FormData(..)` constructor, creating a new instance of `FormData`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FormData/FormData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FormData`, `HtmlFormElement`*"]
impl FormData {
    #[js::import = "new FormData($0)"]
pub fn new_with_form(form: &HtmlFormElement) -> Result<FormData, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Blob")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FormData/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `FormData`*"]
impl FormData {
    #[js::import = "$0.append($1, $2)"]
pub fn append_with_blob(&self, name: &str, value: &Blob) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Blob")]
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FormData/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `FormData`*"]
impl FormData {
    #[js::import = "$0.append($1, $2, $3)"]
pub fn append_with_blob_and_filename(&self, name: &str, value: &Blob, filename: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `append()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FormData/append)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FormData`*"]
impl FormData {
    #[js::import = "$0.append($1, $2)"]
pub fn append_with_str(&self, name: &str, value: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `delete()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FormData/delete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FormData`*"]
impl FormData {
    #[js::import = "$0.delete($1)"]
pub fn delete(&self, name: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FormData/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FormData`*"]
impl FormData {
    #[js::import = "$0.entries()"]
pub fn entries(&self) -> ::js_sys::Iterator { panic!("web-sys intrinsic") }
}
    #[doc = "The `forEach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FormData/forEach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FormData`*"]
impl FormData {
    #[js::import = "$0.forEach($1)"]
pub fn for_each(&self, callback: &::js_sys::Function) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FormData/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FormData`*"]
impl FormData {
    #[js::import = "$0.get($1)"]
pub fn get(&self, name: &str) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
}
    #[doc = "The `getAll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FormData/getAll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FormData`*"]
impl FormData {
    #[js::import = "$0.getAll($1)"]
pub fn get_all(&self, name: &str) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "The `has()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FormData/has)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FormData`*"]
impl FormData {
    #[js::import = "$0.has($1)"]
pub fn has(&self, name: &str) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FormData/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FormData`*"]
impl FormData {
    #[js::import = "$0.keys()"]
pub fn keys(&self) -> ::js_sys::Iterator { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Blob")]
    #[doc = "The `set()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FormData/set)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `FormData`*"]
impl FormData {
    #[js::import = "$0.set($1, $2)"]
pub fn set_with_blob(&self, name: &str, value: &Blob) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Blob")]
    #[doc = "The `set()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FormData/set)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `FormData`*"]
impl FormData {
    #[js::import = "$0.set($1, $2, $3)"]
pub fn set_with_blob_and_filename(&self, name: &str, value: &Blob, filename: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `set()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FormData/set)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FormData`*"]
impl FormData {
    #[js::import = "$0.set($1, $2)"]
pub fn set_with_str(&self, name: &str, value: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `values()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FormData/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FormData`*"]
impl FormData {
    #[js::import = "$0.values()"]
pub fn values(&self) -> ::js_sys::Iterator { panic!("web-sys intrinsic") }
}
