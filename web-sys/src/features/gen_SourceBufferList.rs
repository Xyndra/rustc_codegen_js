#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SourceBufferList` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBufferList)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBufferList`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SourceBufferList(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBufferList/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBufferList`*"]
impl SourceBufferList {
    #[js::import = "$0.length"]
pub fn length(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onaddsourcebuffer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBufferList/onaddsourcebuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBufferList`*"]
impl SourceBufferList {
    #[js::import = "$0.onaddsourcebuffer"]
pub fn onaddsourcebuffer(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onaddsourcebuffer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBufferList/onaddsourcebuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBufferList`*"]
impl SourceBufferList {
    #[js::import = "$0.onaddsourcebuffer = $1"]
pub fn set_onaddsourcebuffer(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onremovesourcebuffer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBufferList/onremovesourcebuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBufferList`*"]
impl SourceBufferList {
    #[js::import = "$0.onremovesourcebuffer"]
pub fn onremovesourcebuffer(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onremovesourcebuffer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SourceBufferList/onremovesourcebuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBufferList`*"]
impl SourceBufferList {
    #[js::import = "$0.onremovesourcebuffer = $1"]
pub fn set_onremovesourcebuffer(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SourceBuffer")]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SourceBuffer`, `SourceBufferList`*"]
impl SourceBufferList {
    #[js::import = "$0.None($1)"]
pub fn get(&self, index: u32) -> Option<SourceBuffer> { panic!("web-sys intrinsic") }
}
