#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ExtDisjointTimerQuery` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ExtDisjointTimerQuery(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "WebGlQuery")]
    #[doc = "The `beginQueryEXT()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query/beginQueryEXT)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`, `WebGlQuery`*"]
impl ExtDisjointTimerQuery {
    #[js::import = "$0.beginQueryEXT($1, $2)"]
pub fn begin_query_ext(&self, target: u32, query: &WebGlQuery) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlQuery")]
    #[doc = "The `createQueryEXT()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query/createQueryEXT)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`, `WebGlQuery`*"]
impl ExtDisjointTimerQuery {
    #[js::import = "$0.createQueryEXT()"]
pub fn create_query_ext(&self) -> Option<WebGlQuery> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlQuery")]
    #[doc = "The `deleteQueryEXT()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query/deleteQueryEXT)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`, `WebGlQuery`*"]
impl ExtDisjointTimerQuery {
    #[js::import = "$0.deleteQueryEXT($1)"]
pub fn delete_query_ext(&self, query: Option<&WebGlQuery>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `endQueryEXT()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query/endQueryEXT)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`*"]
impl ExtDisjointTimerQuery {
    #[js::import = "$0.endQueryEXT($1)"]
pub fn end_query_ext(&self, target: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `getQueryEXT()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query/getQueryEXT)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`*"]
impl ExtDisjointTimerQuery {
    #[js::import = "$0.getQueryEXT($1, $2)"]
pub fn get_query_ext(&self, target: u32, pname: u32) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlQuery")]
    #[doc = "The `getQueryObjectEXT()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query/getQueryObjectEXT)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`, `WebGlQuery`*"]
impl ExtDisjointTimerQuery {
    #[js::import = "$0.getQueryObjectEXT($1, $2)"]
pub fn get_query_object_ext(&self, query: &WebGlQuery, pname: u32) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlQuery")]
    #[doc = "The `isQueryEXT()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query/isQueryEXT)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`, `WebGlQuery`*"]
impl ExtDisjointTimerQuery {
    #[js::import = "$0.isQueryEXT($1)"]
pub fn is_query_ext(&self, query: Option<&WebGlQuery>) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlQuery")]
    #[doc = "The `queryCounterEXT()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/EXT_disjoint_timer_query/queryCounterEXT)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`, `WebGlQuery`*"]
impl ExtDisjointTimerQuery {
    #[js::import = "$0.queryCounterEXT($1, $2)"]
pub fn query_counter_ext(&self, query: &WebGlQuery, target: u32) { panic!("web-sys intrinsic") }
}
impl ExtDisjointTimerQuery {
    #[doc = "The `EXT_disjoint_timer_query.QUERY_COUNTER_BITS_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`*"]
    pub const QUERY_COUNTER_BITS_EXT: u32 = 34916u64 as u32;
    #[doc = "The `EXT_disjoint_timer_query.CURRENT_QUERY_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`*"]
    pub const CURRENT_QUERY_EXT: u32 = 34917u64 as u32;
    #[doc = "The `EXT_disjoint_timer_query.QUERY_RESULT_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`*"]
    pub const QUERY_RESULT_EXT: u32 = 34918u64 as u32;
    #[doc = "The `EXT_disjoint_timer_query.QUERY_RESULT_AVAILABLE_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`*"]
    pub const QUERY_RESULT_AVAILABLE_EXT: u32 = 34919u64 as u32;
    #[doc = "The `EXT_disjoint_timer_query.TIME_ELAPSED_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`*"]
    pub const TIME_ELAPSED_EXT: u32 = 35007u64 as u32;
    #[doc = "The `EXT_disjoint_timer_query.TIMESTAMP_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`*"]
    pub const TIMESTAMP_EXT: u32 = 36392u64 as u32;
    #[doc = "The `EXT_disjoint_timer_query.GPU_DISJOINT_EXT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ExtDisjointTimerQuery`*"]
    pub const GPU_DISJOINT_EXT: u32 = 36795u64 as u32;
}
