#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MediaError` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaError`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MediaError(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `code` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaError/code)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaError`*"]
impl MediaError {
    #[js::import = "$0.code"]
pub fn code(&self) -> u16 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `message` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MediaError/message)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaError`*"]
impl MediaError {
    #[js::import = "$0.message"]
pub fn message(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
impl MediaError {
    #[doc = "The `MediaError.MEDIA_ERR_ABORTED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaError`*"]
    pub const MEDIA_ERR_ABORTED: u16 = 1u64 as u16;
    #[doc = "The `MediaError.MEDIA_ERR_NETWORK` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaError`*"]
    pub const MEDIA_ERR_NETWORK: u16 = 2u64 as u16;
    #[doc = "The `MediaError.MEDIA_ERR_DECODE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaError`*"]
    pub const MEDIA_ERR_DECODE: u16 = 3u64 as u16;
    #[doc = "The `MediaError.MEDIA_ERR_SRC_NOT_SUPPORTED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaError`*"]
    pub const MEDIA_ERR_SRC_NOT_SUPPORTED: u16 = 4u64 as u16;
}
