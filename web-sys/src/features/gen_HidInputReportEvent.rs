#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HidInputReportEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HIDInputReportEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidInputReportEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct HidInputReportEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HidDevice")]
    #[doc = "Getter for the `device` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HIDInputReportEvent/device)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidDevice`, `HidInputReportEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl HidInputReportEvent {
    #[js::import = "$0.device"]
pub fn device(&self) -> HidDevice { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `reportId` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HIDInputReportEvent/reportId)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidInputReportEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl HidInputReportEvent {
    #[js::import = "$0.reportId"]
pub fn report_id(&self) -> u8 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `data` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HIDInputReportEvent/data)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidInputReportEvent`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl HidInputReportEvent {
    #[js::import = "$0.data"]
pub fn data(&self) -> ::js_sys::DataView { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HidInputReportEventInit")]
    #[doc = "The `new HidInputReportEvent(..)` constructor, creating a new instance of `HidInputReportEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HIDInputReportEvent/HIDInputReportEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HidInputReportEvent`, `HidInputReportEventInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl HidInputReportEvent {
    #[js::import = "new HIDInputReportEvent($0, $1)"]
pub fn new(type_: &str, event_init_dict: &HidInputReportEventInit,) -> Result<HidInputReportEvent, JsValue> { panic!("web-sys intrinsic") }
}
