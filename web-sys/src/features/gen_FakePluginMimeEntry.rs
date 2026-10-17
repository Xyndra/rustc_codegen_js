#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FakePluginMimeEntry` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginMimeEntry`*"]
#[js::opaque]
#[derive(Copy)]
pub struct FakePluginMimeEntry(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `description` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginMimeEntry`*"]
#[js::import = "None"]
pub fn get_description(this: &FakePluginMimeEntry) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `description` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginMimeEntry`*"]
#[js::import = "None = $1"]
pub fn set_description(this: &FakePluginMimeEntry, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `extension` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginMimeEntry`*"]
#[js::import = "None"]
pub fn get_extension(this: &FakePluginMimeEntry) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `extension` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginMimeEntry`*"]
#[js::import = "None = $1"]
pub fn set_extension(this: &FakePluginMimeEntry, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginMimeEntry`*"]
#[js::import = "None"]
pub fn get_type(this: &FakePluginMimeEntry) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginMimeEntry`*"]
#[js::import = "None = $1"]
pub fn set_type(this: &FakePluginMimeEntry, val: &str) { panic!("web-sys intrinsic") }
impl FakePluginMimeEntry {
    #[doc = "Construct a new `FakePluginMimeEntry`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FakePluginMimeEntry`*"]
#[js::import = "None"]
pub fn new(type_: &str) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_type(type_);
        ret
    }
    #[deprecated = "Use `set_description()` instead."]
#[js::import = "None"]
pub fn description(&mut self, val: &str) -> &mut Self { self.set_description(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_extension()` instead."]
#[js::import = "None"]
pub fn extension(&mut self, val: &str) -> &mut Self { self.set_extension(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_type()` instead."]
#[js::import = "None"]
pub fn type_(&mut self, val: &str) -> &mut Self { self.set_type(val) { panic!("web-sys intrinsic") }
        self
    }
}
