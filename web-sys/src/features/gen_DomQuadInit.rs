#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomQuadInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuadInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DomQuadInit(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "DomPointInit")]
    #[doc = "Get the `p1` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
#[js::import = "None"]
pub fn get_p1(this: &DomQuadInit) -> Option<DomPointInit> { panic!("web-sys intrinsic") }
    #[cfg(feature = "DomPointInit")]
    #[doc = "Change the `p1` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
#[js::import = "None = $1"]
pub fn set_p1(this: &DomQuadInit, val: &DomPointInit) { panic!("web-sys intrinsic") }
    #[cfg(feature = "DomPointInit")]
    #[doc = "Get the `p2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
#[js::import = "None"]
pub fn get_p2(this: &DomQuadInit) -> Option<DomPointInit> { panic!("web-sys intrinsic") }
    #[cfg(feature = "DomPointInit")]
    #[doc = "Change the `p2` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
#[js::import = "None = $1"]
pub fn set_p2(this: &DomQuadInit, val: &DomPointInit) { panic!("web-sys intrinsic") }
    #[cfg(feature = "DomPointInit")]
    #[doc = "Get the `p3` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
#[js::import = "None"]
pub fn get_p3(this: &DomQuadInit) -> Option<DomPointInit> { panic!("web-sys intrinsic") }
    #[cfg(feature = "DomPointInit")]
    #[doc = "Change the `p3` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
#[js::import = "None = $1"]
pub fn set_p3(this: &DomQuadInit, val: &DomPointInit) { panic!("web-sys intrinsic") }
    #[cfg(feature = "DomPointInit")]
    #[doc = "Get the `p4` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
#[js::import = "None"]
pub fn get_p4(this: &DomQuadInit) -> Option<DomPointInit> { panic!("web-sys intrinsic") }
    #[cfg(feature = "DomPointInit")]
    #[doc = "Change the `p4` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuadInit`*"]
#[js::import = "None = $1"]
pub fn set_p4(this: &DomQuadInit, val: &DomPointInit) { panic!("web-sys intrinsic") }
impl DomQuadInit {
    #[doc = "Construct a new `DomQuadInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuadInit`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(feature = "DomPointInit")]
    #[deprecated = "Use `set_p1()` instead."]
#[js::import = "None"]
pub fn p1(&mut self, val: &DomPointInit) -> &mut Self { self.set_p1(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "DomPointInit")]
    #[deprecated = "Use `set_p2()` instead."]
#[js::import = "None"]
pub fn p2(&mut self, val: &DomPointInit) -> &mut Self { self.set_p2(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "DomPointInit")]
    #[deprecated = "Use `set_p3()` instead."]
#[js::import = "None"]
pub fn p3(&mut self, val: &DomPointInit) -> &mut Self { self.set_p3(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "DomPointInit")]
    #[deprecated = "Use `set_p4()` instead."]
#[js::import = "None"]
pub fn p4(&mut self, val: &DomPointInit) -> &mut Self { self.set_p4(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for DomQuadInit {
    fn default() -> Self {
        Self::new()
    }
}
