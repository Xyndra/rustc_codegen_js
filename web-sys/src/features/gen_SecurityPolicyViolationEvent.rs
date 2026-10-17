#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `SecurityPolicyViolationEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct SecurityPolicyViolationEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `documentURI` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/documentURI)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
impl SecurityPolicyViolationEvent {
    #[js::import = "$0.documentURI"]
pub fn document_uri(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `referrer` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/referrer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
impl SecurityPolicyViolationEvent {
    #[js::import = "$0.referrer"]
pub fn referrer(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `blockedURI` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/blockedURI)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
impl SecurityPolicyViolationEvent {
    #[js::import = "$0.blockedURI"]
pub fn blocked_uri(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `violatedDirective` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/violatedDirective)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
impl SecurityPolicyViolationEvent {
    #[js::import = "$0.violatedDirective"]
pub fn violated_directive(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `effectiveDirective` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/effectiveDirective)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
impl SecurityPolicyViolationEvent {
    #[js::import = "$0.effectiveDirective"]
pub fn effective_directive(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `originalPolicy` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/originalPolicy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
impl SecurityPolicyViolationEvent {
    #[js::import = "$0.originalPolicy"]
pub fn original_policy(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `sourceFile` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/sourceFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
impl SecurityPolicyViolationEvent {
    #[js::import = "$0.sourceFile"]
pub fn source_file(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `sample` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/sample)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
impl SecurityPolicyViolationEvent {
    #[js::import = "$0.sample"]
pub fn sample(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SecurityPolicyViolationEventDisposition")]
    #[doc = "Getter for the `disposition` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/disposition)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`, `SecurityPolicyViolationEventDisposition`*"]
impl SecurityPolicyViolationEvent {
    #[js::import = "$0.disposition"]
pub fn disposition(&self) -> SecurityPolicyViolationEventDisposition { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `statusCode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/statusCode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
impl SecurityPolicyViolationEvent {
    #[js::import = "$0.statusCode"]
pub fn status_code(&self) -> u16 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `lineNumber` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/lineNumber)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
impl SecurityPolicyViolationEvent {
    #[js::import = "$0.lineNumber"]
pub fn line_number(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `columnNumber` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/columnNumber)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
impl SecurityPolicyViolationEvent {
    #[js::import = "$0.columnNumber"]
pub fn column_number(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `new SecurityPolicyViolationEvent(..)` constructor, creating a new instance of `SecurityPolicyViolationEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/SecurityPolicyViolationEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`*"]
impl SecurityPolicyViolationEvent {
    #[js::import = "new SecurityPolicyViolationEvent($0)"]
pub fn new(type_: &str) -> Result<SecurityPolicyViolationEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SecurityPolicyViolationEventInit")]
    #[doc = "The `new SecurityPolicyViolationEvent(..)` constructor, creating a new instance of `SecurityPolicyViolationEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/SecurityPolicyViolationEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SecurityPolicyViolationEvent`, `SecurityPolicyViolationEventInit`*"]
impl SecurityPolicyViolationEvent {
    #[js::import = "new SecurityPolicyViolationEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &SecurityPolicyViolationEventInit,) -> Result<SecurityPolicyViolationEvent, JsValue> { panic!("web-sys intrinsic") }
}
