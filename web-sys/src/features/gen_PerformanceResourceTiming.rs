#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PerformanceResourceTiming` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PerformanceResourceTiming(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `initiatorType` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/initiatorType)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
impl PerformanceResourceTiming {
    #[js::import = "$0.initiatorType"]
pub fn initiator_type(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `nextHopProtocol` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/nextHopProtocol)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
impl PerformanceResourceTiming {
    #[js::import = "$0.nextHopProtocol"]
pub fn next_hop_protocol(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `workerStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/workerStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
impl PerformanceResourceTiming {
    #[js::import = "$0.workerStart"]
pub fn worker_start(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `redirectStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/redirectStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
impl PerformanceResourceTiming {
    #[js::import = "$0.redirectStart"]
pub fn redirect_start(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `redirectEnd` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/redirectEnd)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
impl PerformanceResourceTiming {
    #[js::import = "$0.redirectEnd"]
pub fn redirect_end(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `fetchStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/fetchStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
impl PerformanceResourceTiming {
    #[js::import = "$0.fetchStart"]
pub fn fetch_start(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `domainLookupStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/domainLookupStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
impl PerformanceResourceTiming {
    #[js::import = "$0.domainLookupStart"]
pub fn domain_lookup_start(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `domainLookupEnd` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/domainLookupEnd)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
impl PerformanceResourceTiming {
    #[js::import = "$0.domainLookupEnd"]
pub fn domain_lookup_end(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `connectStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/connectStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
impl PerformanceResourceTiming {
    #[js::import = "$0.connectStart"]
pub fn connect_start(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `connectEnd` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/connectEnd)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
impl PerformanceResourceTiming {
    #[js::import = "$0.connectEnd"]
pub fn connect_end(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `secureConnectionStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/secureConnectionStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
impl PerformanceResourceTiming {
    #[js::import = "$0.secureConnectionStart"]
pub fn secure_connection_start(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `requestStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/requestStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
impl PerformanceResourceTiming {
    #[js::import = "$0.requestStart"]
pub fn request_start(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `responseStart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/responseStart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
impl PerformanceResourceTiming {
    #[js::import = "$0.responseStart"]
pub fn response_start(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `responseEnd` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/responseEnd)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
impl PerformanceResourceTiming {
    #[js::import = "$0.responseEnd"]
pub fn response_end(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `transferSize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/transferSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
impl PerformanceResourceTiming {
    #[js::import = "$0.transferSize"]
pub fn transfer_size(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `encodedBodySize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/encodedBodySize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
impl PerformanceResourceTiming {
    #[js::import = "$0.encodedBodySize"]
pub fn encoded_body_size(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `decodedBodySize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/decodedBodySize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
impl PerformanceResourceTiming {
    #[js::import = "$0.decodedBodySize"]
pub fn decoded_body_size(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `serverTiming` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/serverTiming)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
impl PerformanceResourceTiming {
    #[js::import = "$0.serverTiming"]
pub fn server_timing(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "The `toJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceResourceTiming/toJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PerformanceResourceTiming`*"]
impl PerformanceResourceTiming {
    #[js::import = "$0.toJSON()"]
pub fn to_json(&self) -> ::js_sys::Object { panic!("web-sys intrinsic") }
}
