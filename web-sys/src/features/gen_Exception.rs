#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Exception` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Exception)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Exception`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Exception(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Exception/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Exception`*"]
impl Exception {
    #[js::import = "$0.name"]
pub fn name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `message` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Exception/message)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Exception`*"]
impl Exception {
    #[js::import = "$0.message"]
pub fn message(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `result` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Exception/result)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Exception`*"]
impl Exception {
    #[js::import = "$0.result"]
pub fn result(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `filename` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Exception/filename)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Exception`*"]
impl Exception {
    #[js::import = "$0.filename"]
pub fn filename(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `lineNumber` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Exception/lineNumber)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Exception`*"]
impl Exception {
    #[js::import = "$0.lineNumber"]
pub fn line_number(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `columnNumber` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Exception/columnNumber)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Exception`*"]
impl Exception {
    #[js::import = "$0.columnNumber"]
pub fn column_number(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Exception/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Exception`*"]
impl Exception {
    #[js::import = "$0.data"]
pub fn data(&self) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `stack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Exception/stack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Exception`*"]
impl Exception {
    #[js::import = "$0.stack"]
pub fn stack(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
