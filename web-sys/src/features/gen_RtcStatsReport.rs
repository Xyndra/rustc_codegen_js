#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcStatsReport` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCStatsReport)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReport`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RtcStatsReport(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCStatsReport/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReport`*"]
impl RtcStatsReport {
    #[js::import = "$0.size"]
pub fn size(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCStatsReport/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReport`*"]
impl RtcStatsReport {
    #[js::import = "$0.entries()"]
pub fn entries(&self) -> ::js_sys::Iterator { panic!("web-sys intrinsic") }
}
    #[doc = "The `forEach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCStatsReport/forEach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReport`*"]
impl RtcStatsReport {
    #[js::import = "$0.forEach($1)"]
pub fn for_each(&self, callback: &::js_sys::Function) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `get()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCStatsReport/get)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReport`*"]
impl RtcStatsReport {
    #[js::import = "$0.get($1)"]
pub fn get(&self, key: &str) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
}
    #[doc = "The `has()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCStatsReport/has)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReport`*"]
impl RtcStatsReport {
    #[js::import = "$0.has($1)"]
pub fn has(&self, key: &str) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `keys()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCStatsReport/keys)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReport`*"]
impl RtcStatsReport {
    #[js::import = "$0.keys()"]
pub fn keys(&self) -> ::js_sys::Iterator { panic!("web-sys intrinsic") }
}
    #[doc = "The `values()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCStatsReport/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcStatsReport`*"]
impl RtcStatsReport {
    #[js::import = "$0.values()"]
pub fn values(&self) -> ::js_sys::Iterator { panic!("web-sys intrinsic") }
}
