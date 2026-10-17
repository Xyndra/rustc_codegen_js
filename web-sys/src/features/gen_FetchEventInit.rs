#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FetchEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct FetchEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &FetchEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &FetchEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &FetchEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &FetchEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &FetchEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &FetchEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `clientId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`*"]
#[js::import = "None"]
pub fn get_client_id(this: &FetchEventInit) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `clientId` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`*"]
#[js::import = "None = $1"]
pub fn set_client_id(this: &FetchEventInit, val: Option<&str>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `isReload` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`*"]
#[js::import = "None"]
pub fn get_is_reload(this: &FetchEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `isReload` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`*"]
#[js::import = "None = $1"]
pub fn set_is_reload(this: &FetchEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[cfg(feature = "Request")]
    #[doc = "Get the `request` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`, `Request`*"]
#[js::import = "None"]
pub fn get_request(this: &FetchEventInit) -> Request { panic!("web-sys intrinsic") }
    #[cfg(feature = "Request")]
    #[doc = "Change the `request` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`, `Request`*"]
#[js::import = "None = $1"]
pub fn set_request(this: &FetchEventInit, val: &Request) { panic!("web-sys intrinsic") }
impl FetchEventInit {
    #[cfg(feature = "Request")]
    #[doc = "Construct a new `FetchEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FetchEventInit`, `Request`*"]
#[js::import = "None"]
pub fn new(request: &Request) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_request(request);
        ret
    }
    #[deprecated = "Use `set_bubbles()` instead."]
#[js::import = "None"]
pub fn bubbles(&mut self, val: bool) -> &mut Self { self.set_bubbles(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_cancelable()` instead."]
#[js::import = "None"]
pub fn cancelable(&mut self, val: bool) -> &mut Self { self.set_cancelable(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_composed()` instead."]
#[js::import = "None"]
pub fn composed(&mut self, val: bool) -> &mut Self { self.set_composed(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_client_id()` instead."]
#[js::import = "None"]
pub fn client_id(&mut self, val: Option<&str>) -> &mut Self { self.set_client_id(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_is_reload()` instead."]
#[js::import = "None"]
pub fn is_reload(&mut self, val: bool) -> &mut Self { self.set_is_reload(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "Request")]
    #[deprecated = "Use `set_request()` instead."]
#[js::import = "None"]
pub fn request(&mut self, val: &Request) -> &mut Self { self.set_request(val) { panic!("web-sys intrinsic") }
        self
    }
}
