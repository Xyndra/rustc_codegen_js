#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RcwnStatus` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RcwnStatus(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `cacheNotSlowCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
#[js::import = "None"]
pub fn get_cache_not_slow_count(this: &RcwnStatus) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cacheNotSlowCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
#[js::import = "None = $1"]
pub fn set_cache_not_slow_count(this: &RcwnStatus, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cacheSlowCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
#[js::import = "None"]
pub fn get_cache_slow_count(this: &RcwnStatus) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cacheSlowCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
#[js::import = "None = $1"]
pub fn set_cache_slow_count(this: &RcwnStatus, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `perfStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
#[js::import = "None"]
pub fn get_perf_stats(this: &RcwnStatus) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `perfStats` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
#[js::import = "None = $1"]
pub fn set_perf_stats(this: &RcwnStatus, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `rcwnCacheWonCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
#[js::import = "None"]
pub fn get_rcwn_cache_won_count(this: &RcwnStatus) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `rcwnCacheWonCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
#[js::import = "None = $1"]
pub fn set_rcwn_cache_won_count(this: &RcwnStatus, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `rcwnNetWonCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
#[js::import = "None"]
pub fn get_rcwn_net_won_count(this: &RcwnStatus) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `rcwnNetWonCount` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
#[js::import = "None = $1"]
pub fn set_rcwn_net_won_count(this: &RcwnStatus, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `totalNetworkRequests` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
#[js::import = "None"]
pub fn get_total_network_requests(this: &RcwnStatus) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `totalNetworkRequests` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
#[js::import = "None = $1"]
pub fn set_total_network_requests(this: &RcwnStatus, val: u32) { panic!("web-sys intrinsic") }
impl RcwnStatus {
    #[doc = "Construct a new `RcwnStatus`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnStatus`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_cache_not_slow_count()` instead."]
#[js::import = "None"]
pub fn cache_not_slow_count(&mut self, val: u32) -> &mut Self { self.set_cache_not_slow_count(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_cache_slow_count()` instead."]
#[js::import = "None"]
pub fn cache_slow_count(&mut self, val: u32) -> &mut Self { self.set_cache_slow_count(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_perf_stats()` instead."]
#[js::import = "None"]
pub fn perf_stats(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_perf_stats(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_rcwn_cache_won_count()` instead."]
#[js::import = "None"]
pub fn rcwn_cache_won_count(&mut self, val: u32) -> &mut Self { self.set_rcwn_cache_won_count(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_rcwn_net_won_count()` instead."]
#[js::import = "None"]
pub fn rcwn_net_won_count(&mut self, val: u32) -> &mut Self { self.set_rcwn_net_won_count(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_total_network_requests()` instead."]
#[js::import = "None"]
pub fn total_network_requests(&mut self, val: u32) -> &mut Self { self.set_total_network_requests(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for RcwnStatus {
    fn default() -> Self {
        Self::new()
    }
}
