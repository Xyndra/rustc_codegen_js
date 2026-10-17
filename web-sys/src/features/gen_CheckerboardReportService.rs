#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `CheckerboardReportService` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CheckerboardReportService)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReportService`*"]
#[js::opaque]
#[derive(Copy)]
pub struct CheckerboardReportService(pub ::wasm_bindgen::JsValue);
    #[doc = "The `new CheckerboardReportService(..)` constructor, creating a new instance of `CheckerboardReportService`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CheckerboardReportService/CheckerboardReportService)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReportService`*"]
impl CheckerboardReportService {
    #[js::import = "new CheckerboardReportService()"]
pub fn new() -> Result<CheckerboardReportService, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `flushActiveReports()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CheckerboardReportService/flushActiveReports)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReportService`*"]
impl CheckerboardReportService {
    #[js::import = "$0.flushActiveReports()"]
pub fn flush_active_reports(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `getReports()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CheckerboardReportService/getReports)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReportService`*"]
impl CheckerboardReportService {
    #[js::import = "$0.getReports()"]
pub fn get_reports(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "The `isRecordingEnabled()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CheckerboardReportService/isRecordingEnabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReportService`*"]
impl CheckerboardReportService {
    #[js::import = "$0.isRecordingEnabled()"]
pub fn is_recording_enabled(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `setRecordingEnabled()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/CheckerboardReportService/setRecordingEnabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CheckerboardReportService`*"]
impl CheckerboardReportService {
    #[js::import = "$0.setRecordingEnabled($1)"]
pub fn set_recording_enabled(&self, a_enabled: bool) { panic!("web-sys intrinsic") }
}
