#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomException` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DomException(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
impl DomException {
    #[js::import = "$0.name"]
pub fn name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `message` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/message)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
impl DomException {
    #[js::import = "$0.message"]
pub fn message(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `code` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/code)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
impl DomException {
    #[js::import = "$0.code"]
pub fn code(&self) -> u16 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `result` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/result)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
impl DomException {
    #[js::import = "$0.result"]
pub fn result(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `filename` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/filename)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
impl DomException {
    #[js::import = "$0.filename"]
pub fn filename(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `lineNumber` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/lineNumber)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
impl DomException {
    #[js::import = "$0.lineNumber"]
pub fn line_number(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `columnNumber` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/columnNumber)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
impl DomException {
    #[js::import = "$0.columnNumber"]
pub fn column_number(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
impl DomException {
    #[js::import = "$0.data"]
pub fn data(&self) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `stack` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/stack)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
impl DomException {
    #[js::import = "$0.stack"]
pub fn stack(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "The `new DomException(..)` constructor, creating a new instance of `DomException`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/DOMException)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
impl DomException {
    #[js::import = "new DOMException()"]
pub fn new() -> Result<DomException, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new DomException(..)` constructor, creating a new instance of `DomException`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/DOMException)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
impl DomException {
    #[js::import = "new DOMException($0)"]
pub fn new_with_message(message: &str) -> Result<DomException, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new DomException(..)` constructor, creating a new instance of `DomException`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMException/DOMException)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
impl DomException {
    #[js::import = "new DOMException($0, $1)"]
pub fn new_with_message_and_name(message: &str, name: &str) -> Result<DomException, JsValue> { panic!("web-sys intrinsic") }
}
impl DomException {
    #[doc = "The `DOMException.INDEX_SIZE_ERR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub const INDEX_SIZE_ERR: u16 = 1u64 as u16;
    #[doc = "The `DOMException.DOMSTRING_SIZE_ERR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub const DOMSTRING_SIZE_ERR: u16 = 2u64 as u16;
    #[doc = "The `DOMException.HIERARCHY_REQUEST_ERR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub const HIERARCHY_REQUEST_ERR: u16 = 3u64 as u16;
    #[doc = "The `DOMException.WRONG_DOCUMENT_ERR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub const WRONG_DOCUMENT_ERR: u16 = 4u64 as u16;
    #[doc = "The `DOMException.INVALID_CHARACTER_ERR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub const INVALID_CHARACTER_ERR: u16 = 5u64 as u16;
    #[doc = "The `DOMException.NO_DATA_ALLOWED_ERR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub const NO_DATA_ALLOWED_ERR: u16 = 6u64 as u16;
    #[doc = "The `DOMException.NO_MODIFICATION_ALLOWED_ERR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub const NO_MODIFICATION_ALLOWED_ERR: u16 = 7u64 as u16;
    #[doc = "The `DOMException.NOT_FOUND_ERR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub const NOT_FOUND_ERR: u16 = 8u64 as u16;
    #[doc = "The `DOMException.NOT_SUPPORTED_ERR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub const NOT_SUPPORTED_ERR: u16 = 9u64 as u16;
    #[doc = "The `DOMException.INUSE_ATTRIBUTE_ERR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub const INUSE_ATTRIBUTE_ERR: u16 = 10u64 as u16;
    #[doc = "The `DOMException.INVALID_STATE_ERR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub const INVALID_STATE_ERR: u16 = 11u64 as u16;
    #[doc = "The `DOMException.SYNTAX_ERR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub const SYNTAX_ERR: u16 = 12u64 as u16;
    #[doc = "The `DOMException.INVALID_MODIFICATION_ERR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub const INVALID_MODIFICATION_ERR: u16 = 13u64 as u16;
    #[doc = "The `DOMException.NAMESPACE_ERR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub const NAMESPACE_ERR: u16 = 14u64 as u16;
    #[doc = "The `DOMException.INVALID_ACCESS_ERR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub const INVALID_ACCESS_ERR: u16 = 15u64 as u16;
    #[doc = "The `DOMException.VALIDATION_ERR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub const VALIDATION_ERR: u16 = 16u64 as u16;
    #[doc = "The `DOMException.TYPE_MISMATCH_ERR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub const TYPE_MISMATCH_ERR: u16 = 17u64 as u16;
    #[doc = "The `DOMException.SECURITY_ERR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub const SECURITY_ERR: u16 = 18u64 as u16;
    #[doc = "The `DOMException.NETWORK_ERR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub const NETWORK_ERR: u16 = 19u64 as u16;
    #[doc = "The `DOMException.ABORT_ERR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub const ABORT_ERR: u16 = 20u64 as u16;
    #[doc = "The `DOMException.URL_MISMATCH_ERR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub const URL_MISMATCH_ERR: u16 = 21u64 as u16;
    #[doc = "The `DOMException.QUOTA_EXCEEDED_ERR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub const QUOTA_EXCEEDED_ERR: u16 = 22u64 as u16;
    #[doc = "The `DOMException.TIMEOUT_ERR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub const TIMEOUT_ERR: u16 = 23u64 as u16;
    #[doc = "The `DOMException.INVALID_NODE_TYPE_ERR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub const INVALID_NODE_TYPE_ERR: u16 = 24u64 as u16;
    #[doc = "The `DOMException.DATA_CLONE_ERR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomException`*"]
    pub const DATA_CLONE_ERR: u16 = 25u64 as u16;
}
