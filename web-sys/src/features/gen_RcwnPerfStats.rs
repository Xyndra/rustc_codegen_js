#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RcwnPerfStats` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RcwnPerfStats(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `avgLong` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"]
#[js::import = "None"]
pub fn get_avg_long(this: &RcwnPerfStats) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `avgLong` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"]
#[js::import = "None = $1"]
pub fn set_avg_long(this: &RcwnPerfStats, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `avgShort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"]
#[js::import = "None"]
pub fn get_avg_short(this: &RcwnPerfStats) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `avgShort` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"]
#[js::import = "None = $1"]
pub fn set_avg_short(this: &RcwnPerfStats, val: u32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `stddevLong` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"]
#[js::import = "None"]
pub fn get_stddev_long(this: &RcwnPerfStats) -> Option<u32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `stddevLong` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"]
#[js::import = "None = $1"]
pub fn set_stddev_long(this: &RcwnPerfStats, val: u32) { panic!("web-sys intrinsic") }
impl RcwnPerfStats {
    #[doc = "Construct a new `RcwnPerfStats`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RcwnPerfStats`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_avg_long()` instead."]
#[js::import = "None"]
pub fn avg_long(&mut self, val: u32) -> &mut Self { self.set_avg_long(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_avg_short()` instead."]
#[js::import = "None"]
pub fn avg_short(&mut self, val: u32) -> &mut Self { self.set_avg_short(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_stddev_long()` instead."]
#[js::import = "None"]
pub fn stddev_long(&mut self, val: u32) -> &mut Self { self.set_stddev_long(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for RcwnPerfStats {
    fn default() -> Self {
        Self::new()
    }
}
