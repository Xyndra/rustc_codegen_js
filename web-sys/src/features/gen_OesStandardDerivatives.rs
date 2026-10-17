#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `OesStandardDerivatives` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/OES_standard_derivatives)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OesStandardDerivatives`*"]
#[js::opaque]
#[derive(Copy)]
pub struct OesStandardDerivatives(pub ::wasm_bindgen::JsValue);
impl OesStandardDerivatives {
    #[doc = "The `OES_standard_derivatives.FRAGMENT_SHADER_DERIVATIVE_HINT_OES` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OesStandardDerivatives`*"]
    pub const FRAGMENT_SHADER_DERIVATIVE_HINT_OES: u32 = 35723u64 as u32;
}
