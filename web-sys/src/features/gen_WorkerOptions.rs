#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WorkerOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct WorkerOptions(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "RequestCredentials")]
    #[doc = "Get the `credentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestCredentials`, `WorkerOptions`*"]
#[js::import = "None"]
pub fn get_credentials(this: &WorkerOptions) -> Option<RequestCredentials> { panic!("web-sys intrinsic") }
    #[cfg(feature = "RequestCredentials")]
    #[doc = "Change the `credentials` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestCredentials`, `WorkerOptions`*"]
#[js::import = "None = $1"]
pub fn set_credentials(this: &WorkerOptions, val: RequestCredentials) { panic!("web-sys intrinsic") }
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerOptions`*"]
#[js::import = "None"]
pub fn get_name(this: &WorkerOptions) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerOptions`*"]
#[js::import = "None = $1"]
pub fn set_name(this: &WorkerOptions, val: &str) { panic!("web-sys intrinsic") }
    #[cfg(feature = "WorkerType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerOptions`, `WorkerType`*"]
#[js::import = "None"]
pub fn get_type(this: &WorkerOptions) -> Option<WorkerType> { panic!("web-sys intrinsic") }
    #[cfg(feature = "WorkerType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerOptions`, `WorkerType`*"]
#[js::import = "None = $1"]
pub fn set_type(this: &WorkerOptions, val: WorkerType) { panic!("web-sys intrinsic") }
impl WorkerOptions {
    #[doc = "Construct a new `WorkerOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[cfg(feature = "RequestCredentials")]
    #[deprecated = "Use `set_credentials()` instead."]
#[js::import = "None"]
pub fn credentials(&mut self, val: RequestCredentials) -> &mut Self { self.set_credentials(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_name()` instead."]
#[js::import = "None"]
pub fn name(&mut self, val: &str) -> &mut Self { self.set_name(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "WorkerType")]
    #[deprecated = "Use `set_type()` instead."]
#[js::import = "None"]
pub fn type_(&mut self, val: WorkerType) -> &mut Self { self.set_type(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for WorkerOptions {
    fn default() -> Self {
        Self::new()
    }
}
