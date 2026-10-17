#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PerformanceNavigation` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceNavigation`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PerformanceNavigation(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigation/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceNavigation`*"]
impl PerformanceNavigation {
    #[js::import = "$0.type"]
pub fn type_(&self) -> u16 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `redirectCount` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigation/redirectCount)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceNavigation`*"]
impl PerformanceNavigation {
    #[js::import = "$0.redirectCount"]
pub fn redirect_count(&self) -> u16 { panic!("web-sys intrinsic") }
}
    #[doc = "The `toJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigation/toJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceNavigation`*"]
impl PerformanceNavigation {
    #[js::import = "$0.toJSON()"]
pub fn to_json(&self) -> ::js_sys::Object { panic!("web-sys intrinsic") }
}
impl PerformanceNavigation {
    #[doc = "The `PerformanceNavigation.TYPE_NAVIGATE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceNavigation`*"]
    pub const TYPE_NAVIGATE: u16 = 0i64 as u16;
    #[doc = "The `PerformanceNavigation.TYPE_RELOAD` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceNavigation`*"]
    pub const TYPE_RELOAD: u16 = 1u64 as u16;
    #[doc = "The `PerformanceNavigation.TYPE_BACK_FORWARD` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceNavigation`*"]
    pub const TYPE_BACK_FORWARD: u16 = 2u64 as u16;
    #[doc = "The `PerformanceNavigation.TYPE_RESERVED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceNavigation`*"]
    pub const TYPE_RESERVED: u16 = 255u64 as u16;
}
