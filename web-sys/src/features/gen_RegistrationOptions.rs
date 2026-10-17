#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RegistrationOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct RegistrationOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `scope` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`*"]
#[js::import = "None"]
pub fn get_scope(this: &RegistrationOptions) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `scope` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`*"]
#[js::import = "None = $1"]
pub fn set_scope(this: &RegistrationOptions, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`*"]
#[js::import = "None"]
pub fn get_type(this: &RegistrationOptions) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`*"]
#[js::import = "None = $1"]
pub fn set_type(this: &RegistrationOptions, val: &str) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ServiceWorkerUpdateViaCache")]
    #[doc = "Get the `updateViaCache` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`, `ServiceWorkerUpdateViaCache`*"]
#[js::import = "None"]
pub fn get_update_via_cache(this: &RegistrationOptions) -> Option<ServiceWorkerUpdateViaCache> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ServiceWorkerUpdateViaCache")]
    #[doc = "Change the `updateViaCache` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`, `ServiceWorkerUpdateViaCache`*"]
#[js::import = "None = $1"]
pub fn set_update_via_cache(this: &RegistrationOptions, val: ServiceWorkerUpdateViaCache) { panic!("web-sys intrinsic") }
impl RegistrationOptions {
    #[doc = "Construct a new `RegistrationOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RegistrationOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_scope()` instead."]
#[js::import = "None"]
pub fn scope(&mut self, val: &str) -> &mut Self { self.set_scope(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_type()` instead."]
#[js::import = "None"]
pub fn type_(&mut self, val: &str) -> &mut Self { self.set_type(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "ServiceWorkerUpdateViaCache")]
    #[deprecated = "Use `set_update_via_cache()` instead."]
#[js::import = "None"]
pub fn update_via_cache(&mut self, val: ServiceWorkerUpdateViaCache) -> &mut Self { self.set_update_via_cache(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for RegistrationOptions {
    fn default() -> Self {
        Self::new()
    }
}
