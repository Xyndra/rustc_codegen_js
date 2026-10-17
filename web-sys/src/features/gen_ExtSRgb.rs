#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ExtSRgb` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_sRGB)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtSRgb`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ExtSRgb(pub ::wasm_bindgen::JsValue);
impl ExtSRgb {
    #[doc = "The `EXT_sRGB.SRGB_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtSRgb`*"]
    pub const SRGB_EXT: u32 = 35904u64 as u32;
    #[doc = "The `EXT_sRGB.SRGB_ALPHA_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtSRgb`*"]
    pub const SRGB_ALPHA_EXT: u32 = 35906u64 as u32;
    #[doc = "The `EXT_sRGB.SRGB8_ALPHA8_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtSRgb`*"]
    pub const SRGB8_ALPHA8_EXT: u32 = 35907u64 as u32;
    #[doc = "The `EXT_sRGB.FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtSRgb`*"]
    pub const FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING_EXT: u32 = 33296u64 as u32;
}
