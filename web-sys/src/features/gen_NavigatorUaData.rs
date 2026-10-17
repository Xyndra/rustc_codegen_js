#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NavigatorUaData` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorUAData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigatorUaData`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct NavigatorUaData(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `brands` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorUAData/brands)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigatorUaData`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl NavigatorUaData {
    #[js::import = "$0.brands"]
pub fn brands(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `mobile` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorUAData/mobile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigatorUaData`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl NavigatorUaData {
    #[js::import = "$0.mobile"]
pub fn mobile(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `platform` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorUAData/platform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigatorUaData`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl NavigatorUaData {
    #[js::import = "$0.platform"]
pub fn platform(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `getHighEntropyValues()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorUAData/getHighEntropyValues)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigatorUaData`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl NavigatorUaData {
    #[js::import = "$0.getHighEntropyValues($1)"]
pub fn get_high_entropy_values(&self, hints: &::wasm_bindgen::JsValue) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "UaLowEntropyJson")]
    #[doc = "The `toJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/NavigatorUAData/toJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NavigatorUaData`, `UaLowEntropyJson`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl NavigatorUaData {
    #[js::import = "$0.toJSON()"]
pub fn to_json(&self) -> UaLowEntropyJson { panic!("web-sys intrinsic") }
}
