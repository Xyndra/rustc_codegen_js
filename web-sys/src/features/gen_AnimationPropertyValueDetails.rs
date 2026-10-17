#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AnimationPropertyValueDetails` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AnimationPropertyValueDetails(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "CompositeOperation")]
    #[doc = "Get the `composite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`, `CompositeOperation`*"]
#[js::import = "None"]
pub fn get_composite(this: &AnimationPropertyValueDetails) -> CompositeOperation { panic!("web-sys intrinsic") }
    #[cfg(feature = "CompositeOperation")]
    #[doc = "Change the `composite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`, `CompositeOperation`*"]
#[js::import = "None = $1"]
pub fn set_composite(this: &AnimationPropertyValueDetails, val: CompositeOperation) { panic!("web-sys intrinsic") }
    #[doc = "Get the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`*"]
#[js::import = "None"]
pub fn get_easing(this: &AnimationPropertyValueDetails) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `easing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`*"]
#[js::import = "None = $1"]
pub fn set_easing(this: &AnimationPropertyValueDetails, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`*"]
#[js::import = "None"]
pub fn get_offset(this: &AnimationPropertyValueDetails) -> f64 { panic!("web-sys intrinsic") }
    #[doc = "Change the `offset` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`*"]
#[js::import = "None = $1"]
pub fn set_offset(this: &AnimationPropertyValueDetails, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`*"]
#[js::import = "None"]
pub fn get_value(this: &AnimationPropertyValueDetails) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `value` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`*"]
#[js::import = "None = $1"]
pub fn set_value(this: &AnimationPropertyValueDetails, val: &str) { panic!("web-sys intrinsic") }
impl AnimationPropertyValueDetails {
    #[cfg(feature = "CompositeOperation")]
    #[doc = "Construct a new `AnimationPropertyValueDetails`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyValueDetails`, `CompositeOperation`*"]
#[js::import = "None"]
pub fn new(composite: CompositeOperation, offset: f64) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_composite(composite);
        ret.set_offset(offset);
        ret
    }
    #[cfg(feature = "CompositeOperation")]
    #[deprecated = "Use `set_composite()` instead."]
#[js::import = "None"]
pub fn composite(&mut self, val: CompositeOperation) -> &mut Self { self.set_composite(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_easing()` instead."]
#[js::import = "None"]
pub fn easing(&mut self, val: &str) -> &mut Self { self.set_easing(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_offset()` instead."]
#[js::import = "None"]
pub fn offset(&mut self, val: f64) -> &mut Self { self.set_offset(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_value()` instead."]
#[js::import = "None"]
pub fn value(&mut self, val: &str) -> &mut Self { self.set_value(val) { panic!("web-sys intrinsic") }
        self
    }
}
