#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HitRegionOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HitRegionOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HitRegionOptions(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "Element")]
    #[doc = "Get the `control` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `HitRegionOptions`*"]
#[js::import = "None"]
pub fn get_control(this: &HitRegionOptions) -> Option<Element> { panic!("web-sys intrinsic") }
    #[cfg(feature = "Element")]
    #[doc = "Change the `control` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `HitRegionOptions`*"]
#[js::import = "None = $1"]
pub fn set_control(this: &HitRegionOptions, val: Option<&Element>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HitRegionOptions`*"]
#[js::import = "None"]
pub fn get_id(this: &HitRegionOptions) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `id` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HitRegionOptions`*"]
#[js::import = "None = $1"]
pub fn set_id(this: &HitRegionOptions, val: &str) { panic!("web-sys intrinsic") }
    #[cfg(feature = "Path2d")]
    #[doc = "Get the `path` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HitRegionOptions`, `Path2d`*"]
#[js::import = "None"]
pub fn get_path(this: &HitRegionOptions) -> Option<Path2d> { panic!("web-sys intrinsic") }
    #[cfg(feature = "Path2d")]
    #[doc = "Change the `path` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HitRegionOptions`, `Path2d`*"]
#[js::import = "None = $1"]
pub fn set_path(this: &HitRegionOptions, val: Option<&Path2d>) { panic!("web-sys intrinsic") }
impl HitRegionOptions {
    #[doc = "Construct a new `HitRegionOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HitRegionOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(feature = "Element")]
    #[deprecated = "Use `set_control()` instead."]
#[js::import = "None"]
pub fn control(&mut self, val: Option<&Element>) -> &mut Self { self.set_control(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_id()` instead."]
#[js::import = "None"]
pub fn id(&mut self, val: &str) -> &mut Self { self.set_id(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "Path2d")]
    #[deprecated = "Use `set_path()` instead."]
#[js::import = "None"]
pub fn path(&mut self, val: Option<&Path2d>) -> &mut Self { self.set_path(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for HitRegionOptions {
    fn default() -> Self {
        Self::new()
    }
}
