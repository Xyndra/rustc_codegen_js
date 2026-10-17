#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `OesTextureHalfFloat` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OES_texture_half_float)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OesTextureHalfFloat`*"]
#[js::opaque]
#[derive(Copy)]
pub struct OesTextureHalfFloat(pub ::wasm_bindgen::JsValue);
impl OesTextureHalfFloat {
    #[doc = "The `OES_texture_half_float.HALF_FLOAT_OES` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OesTextureHalfFloat`*"]
    pub const HALF_FLOAT_OES: u32 = 36193u64 as u32;
}
