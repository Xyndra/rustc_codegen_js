#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomQuadJson` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuadJson`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DomQuadJson(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "DomPoint")]
    #[doc = "Get the `p1` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuadJson`*"]
#[js::import = "None"]
pub fn get_p1(this: &DomQuadJson) -> Option<DomPoint> { panic!("web-sys intrinsic") }
    #[cfg(feature = "DomPoint")]
    #[doc = "Change the `p1` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuadJson`*"]
#[js::import = "None = $1"]
pub fn set_p1(this: &DomQuadJson, val: &DomPoint) { panic!("web-sys intrinsic") }
    #[cfg(feature = "DomPoint")]
    #[doc = "Get the `p2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuadJson`*"]
#[js::import = "None"]
pub fn get_p2(this: &DomQuadJson) -> Option<DomPoint> { panic!("web-sys intrinsic") }
    #[cfg(feature = "DomPoint")]
    #[doc = "Change the `p2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuadJson`*"]
#[js::import = "None = $1"]
pub fn set_p2(this: &DomQuadJson, val: &DomPoint) { panic!("web-sys intrinsic") }
    #[cfg(feature = "DomPoint")]
    #[doc = "Get the `p3` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuadJson`*"]
#[js::import = "None"]
pub fn get_p3(this: &DomQuadJson) -> Option<DomPoint> { panic!("web-sys intrinsic") }
    #[cfg(feature = "DomPoint")]
    #[doc = "Change the `p3` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuadJson`*"]
#[js::import = "None = $1"]
pub fn set_p3(this: &DomQuadJson, val: &DomPoint) { panic!("web-sys intrinsic") }
    #[cfg(feature = "DomPoint")]
    #[doc = "Get the `p4` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuadJson`*"]
#[js::import = "None"]
pub fn get_p4(this: &DomQuadJson) -> Option<DomPoint> { panic!("web-sys intrinsic") }
    #[cfg(feature = "DomPoint")]
    #[doc = "Change the `p4` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuadJson`*"]
#[js::import = "None = $1"]
pub fn set_p4(this: &DomQuadJson, val: &DomPoint) { panic!("web-sys intrinsic") }
impl DomQuadJson {
    #[doc = "Construct a new `DomQuadJson`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuadJson`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(feature = "DomPoint")]
    #[deprecated = "Use `set_p1()` instead."]
#[js::import = "None"]
pub fn p1(&mut self, val: &DomPoint) -> &mut Self { self.set_p1(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "DomPoint")]
    #[deprecated = "Use `set_p2()` instead."]
#[js::import = "None"]
pub fn p2(&mut self, val: &DomPoint) -> &mut Self { self.set_p2(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "DomPoint")]
    #[deprecated = "Use `set_p3()` instead."]
#[js::import = "None"]
pub fn p3(&mut self, val: &DomPoint) -> &mut Self { self.set_p3(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "DomPoint")]
    #[deprecated = "Use `set_p4()` instead."]
#[js::import = "None"]
pub fn p4(&mut self, val: &DomPoint) -> &mut Self { self.set_p4(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for DomQuadJson {
    fn default() -> Self {
        Self::new()
    }
}
