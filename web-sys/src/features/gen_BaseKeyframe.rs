#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BaseKeyframe` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseKeyframe`*"]
#[js::opaque]
#[derive(Copy)]
pub struct BaseKeyframe(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "CompositeOperation")]
    #[doc = "Get the `composite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseKeyframe`, `CompositeOperation`*"]
#[js::import = "None"]
pub fn get_composite(this: &BaseKeyframe) -> Option<CompositeOperation> { panic!("web-sys intrinsic") }
    #[cfg(feature = "CompositeOperation")]
    #[doc = "Change the `composite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseKeyframe`, `CompositeOperation`*"]
#[js::import = "None = $1"]
pub fn set_composite(this: &BaseKeyframe, val: Option<CompositeOperation>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseKeyframe`*"]
#[js::import = "None"]
pub fn get_easing(this: &BaseKeyframe) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseKeyframe`*"]
#[js::import = "None = $1"]
pub fn set_easing(this: &BaseKeyframe, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseKeyframe`*"]
#[js::import = "None"]
pub fn get_offset(this: &BaseKeyframe) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseKeyframe`*"]
#[js::import = "None = $1"]
pub fn set_offset(this: &BaseKeyframe, val: Option<f64>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `simulateComputeValuesFailure` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseKeyframe`*"]
#[js::import = "None"]
pub fn get_simulate_compute_values_failure(this: &BaseKeyframe) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `simulateComputeValuesFailure` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseKeyframe`*"]
#[js::import = "None = $1"]
pub fn set_simulate_compute_values_failure(this: &BaseKeyframe, val: bool) { panic!("web-sys intrinsic") }
impl BaseKeyframe {
    #[doc = "Construct a new `BaseKeyframe`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BaseKeyframe`*"]
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
}
impl Default for BaseKeyframe {
    fn default() -> Self {
        Self::new()
    }
}
