#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `AnimationPropertyDetails` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyDetails`*"]
#[js::opaque]
#[derive(Copy)]
pub struct AnimationPropertyDetails(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `property` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyDetails`*"]
#[js::import = "None"]
pub fn get_property(this: &AnimationPropertyDetails) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `property` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyDetails`*"]
#[js::import = "None = $1"]
pub fn set_property(this: &AnimationPropertyDetails, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `runningOnCompositor` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyDetails`*"]
#[js::import = "None"]
pub fn get_running_on_compositor(this: &AnimationPropertyDetails) -> bool { panic!("web-sys intrinsic") }
    #[doc = "Change the `runningOnCompositor` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyDetails`*"]
#[js::import = "None = $1"]
pub fn set_running_on_compositor(this: &AnimationPropertyDetails, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `values` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyDetails`*"]
#[js::import = "None"]
pub fn get_values(this: &AnimationPropertyDetails) -> ::js_sys::Array { panic!("web-sys intrinsic") }
    #[doc = "Change the `values` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyDetails`*"]
#[js::import = "None = $1"]
pub fn set_values(this: &AnimationPropertyDetails, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
    #[doc = "Get the `warning` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyDetails`*"]
#[js::import = "None"]
pub fn get_warning(this: &AnimationPropertyDetails) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `warning` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyDetails`*"]
#[js::import = "None = $1"]
pub fn set_warning(this: &AnimationPropertyDetails, val: &str) { panic!("web-sys intrinsic") }
impl AnimationPropertyDetails {
    #[doc = "Construct a new `AnimationPropertyDetails`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `AnimationPropertyDetails`*"]
#[js::import = "None"]
pub fn new(property: &str, running_on_compositor: bool, values: &::wasm_bindgen::JsValue,) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_property(property);
        ret.set_running_on_compositor(running_on_compositor);
        ret.set_values(values);
        ret
    }
    #[deprecated = "Use `set_property()` instead."]
#[js::import = "None"]
pub fn property(&mut self, val: &str) -> &mut Self { self.set_property(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_running_on_compositor()` instead."]
#[js::import = "None"]
pub fn running_on_compositor(&mut self, val: bool) -> &mut Self { self.set_running_on_compositor(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_values()` instead."]
#[js::import = "None"]
pub fn values(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_values(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_warning()` instead."]
#[js::import = "None"]
pub fn warning(&mut self, val: &str) -> &mut Self { self.set_warning(val) { panic!("web-sys intrinsic") }
        self
    }
}
