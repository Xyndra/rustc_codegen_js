#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RsaOtherPrimesInfo` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RsaOtherPrimesInfo(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `d` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*"]
#[js::import = "None"]
pub fn get_d(this: &RsaOtherPrimesInfo) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `d` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*"]
#[js::import = "None = $1"]
pub fn set_d(this: &RsaOtherPrimesInfo, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `r` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*"]
#[js::import = "None"]
pub fn get_r(this: &RsaOtherPrimesInfo) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `r` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*"]
#[js::import = "None = $1"]
pub fn set_r(this: &RsaOtherPrimesInfo, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `t` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*"]
#[js::import = "None"]
pub fn get_t(this: &RsaOtherPrimesInfo) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `t` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*"]
#[js::import = "None = $1"]
pub fn set_t(this: &RsaOtherPrimesInfo, val: &str) { panic!("web-sys intrinsic") }
impl RsaOtherPrimesInfo {
    #[doc = "Construct a new `RsaOtherPrimesInfo`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RsaOtherPrimesInfo`*"]
#[js::import = "None"]
pub fn new(d: &str, r: &str, t: &str) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_d(d);
        ret.set_r(r);
        ret.set_t(t);
        ret
    }
    #[deprecated = "Use `set_d()` instead."]
#[js::import = "None"]
pub fn d(&mut self, val: &str) -> &mut Self { self.set_d(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_r()` instead."]
#[js::import = "None"]
pub fn r(&mut self, val: &str) -> &mut Self { self.set_r(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_t()` instead."]
#[js::import = "None"]
pub fn t(&mut self, val: &str) -> &mut Self { self.set_t(val) { panic!("web-sys intrinsic") }
        self
    }
}
