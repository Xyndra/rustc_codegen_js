#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BaseComputedKeyframe` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
#[js::opaque]
#[derive(Copy)]
pub struct BaseComputedKeyframe(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "CompositeOperation")]
    #[doc = "Get the `composite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`, `CompositeOperation`*"]
#[js::import = "None"]
pub fn get_composite(this: &BaseComputedKeyframe) -> Option<CompositeOperation> { panic!("web-sys intrinsic") }
    #[cfg(feature = "CompositeOperation")]
    #[doc = "Change the `composite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`, `CompositeOperation`*"]
#[js::import = "None = $1"]
pub fn set_composite(this: &BaseComputedKeyframe, val: Option<CompositeOperation>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
#[js::import = "None"]
pub fn get_easing(this: &BaseComputedKeyframe) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
#[js::import = "None = $1"]
pub fn set_easing(this: &BaseComputedKeyframe, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
#[js::import = "None"]
pub fn get_offset(this: &BaseComputedKeyframe) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
#[js::import = "None = $1"]
pub fn set_offset(this: &BaseComputedKeyframe, val: Option<f64>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `simulateComputeValuesFailure` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
#[js::import = "None"]
pub fn get_simulate_compute_values_failure(this: &BaseComputedKeyframe) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `simulateComputeValuesFailure` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
#[js::import = "None = $1"]
pub fn set_simulate_compute_values_failure(this: &BaseComputedKeyframe, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `computedOffset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
#[js::import = "None"]
pub fn get_computed_offset(this: &BaseComputedKeyframe) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `computedOffset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
#[js::import = "None = $1"]
pub fn set_computed_offset(this: &BaseComputedKeyframe, val: f64) { panic!("web-sys intrinsic") }
impl BaseComputedKeyframe {
    #[doc = "Construct a new `BaseComputedKeyframe`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseComputedKeyframe`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(feature = "CompositeOperation")]
    #[deprecated = "Use `set_composite()` instead."]
#[js::import = "None"]
pub fn composite(&mut self, val: Option<CompositeOperation>) -> &mut Self { self.set_composite(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_easing()` instead."]
#[js::import = "None"]
pub fn easing(&mut self, val: &str) -> &mut Self { self.set_easing(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_offset()` instead."]
#[js::import = "None"]
pub fn offset(&mut self, val: Option<f64>) -> &mut Self { self.set_offset(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_simulate_compute_values_failure()` instead."]
#[js::import = "None"]
pub fn simulate_compute_values_failure(&mut self, val: bool) -> &mut Self { self.set_simulate_compute_values_failure(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_computed_offset()` instead."]
#[js::import = "None"]
pub fn computed_offset(&mut self, val: f64) -> &mut Self { self.set_computed_offset(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for BaseComputedKeyframe {
    fn default() -> Self {
        Self::new()
    }
}
