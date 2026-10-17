#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PositionError` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PositionError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionError`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PositionError(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `code` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PositionError/code)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionError`*"]
impl PositionError {
    #[js::import = "$0.code"]
pub fn code(&self) -> u16 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `message` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PositionError/message)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionError`*"]
impl PositionError {
    #[js::import = "$0.message"]
pub fn message(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
impl PositionError {
    #[doc = "The `PositionError.PERMISSION_DENIED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionError`*"]
    pub const PERMISSION_DENIED: u16 = 1u64 as u16;
    #[doc = "The `PositionError.POSITION_UNAVAILABLE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionError`*"]
    pub const POSITION_UNAVAILABLE: u16 = 2u64 as u16;
    #[doc = "The `PositionError.TIMEOUT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PositionError`*"]
    pub const TIMEOUT: u16 = 3u64 as u16;
}
