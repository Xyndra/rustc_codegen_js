#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WidevineCdmManifest` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
#[js::opaque]
#[derive(Copy)]
pub struct WidevineCdmManifest(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `description` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
#[js::import = "None"]
pub fn get_description(this: &WidevineCdmManifest) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `description` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
#[js::import = "None = $1"]
pub fn set_description(this: &WidevineCdmManifest, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
#[js::import = "None"]
pub fn get_name(this: &WidevineCdmManifest) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
#[js::import = "None = $1"]
pub fn set_name(this: &WidevineCdmManifest, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
#[js::import = "None"]
pub fn get_version(this: &WidevineCdmManifest) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `version` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
#[js::import = "None = $1"]
pub fn set_version(this: &WidevineCdmManifest, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `x-cdm-codecs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
#[js::import = "None"]
pub fn get_x_cdm_codecs(this: &WidevineCdmManifest) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `x-cdm-codecs` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
#[js::import = "None = $1"]
pub fn set_x_cdm_codecs(this: &WidevineCdmManifest, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `x-cdm-host-versions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
#[js::import = "None"]
pub fn get_x_cdm_host_versions(this: &WidevineCdmManifest) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `x-cdm-host-versions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
#[js::import = "None = $1"]
pub fn set_x_cdm_host_versions(this: &WidevineCdmManifest, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `x-cdm-interface-versions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
#[js::import = "None"]
pub fn get_x_cdm_interface_versions(this: &WidevineCdmManifest) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `x-cdm-interface-versions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
#[js::import = "None = $1"]
pub fn set_x_cdm_interface_versions(this: &WidevineCdmManifest, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `x-cdm-module-versions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
#[js::import = "None"]
pub fn get_x_cdm_module_versions(this: &WidevineCdmManifest) -> ::alloc::string::String { panic!("web-sys intrinsic") }
    #[doc = "Change the `x-cdm-module-versions` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
#[js::import = "None = $1"]
pub fn set_x_cdm_module_versions(this: &WidevineCdmManifest, val: &str) { panic!("web-sys intrinsic") }
impl WidevineCdmManifest {
    #[doc = "Construct a new `WidevineCdmManifest`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WidevineCdmManifest`*"]
#[js::import = "None"]
pub fn new(description: &str, name: &str, version: &str, x_cdm_codecs: &str, x_cdm_host_versions: &str, x_cdm_interface_versions: &str, x_cdm_module_versions: &str,) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_description(description);
        ret.set_name(name);
        ret.set_version(version);
        ret.set_x_cdm_codecs(x_cdm_codecs);
        ret.set_x_cdm_host_versions(x_cdm_host_versions);
        ret.set_x_cdm_interface_versions(x_cdm_interface_versions);
        ret.set_x_cdm_module_versions(x_cdm_module_versions);
        ret
    }
    #[deprecated = "Use `set_description()` instead."]
#[js::import = "None"]
pub fn description(&mut self, val: &str) -> &mut Self { self.set_description(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_name()` instead."]
#[js::import = "None"]
pub fn name(&mut self, val: &str) -> &mut Self { self.set_name(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_version()` instead."]
#[js::import = "None"]
pub fn version(&mut self, val: &str) -> &mut Self { self.set_version(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_x_cdm_codecs()` instead."]
#[js::import = "None"]
pub fn x_cdm_codecs(&mut self, val: &str) -> &mut Self { self.set_x_cdm_codecs(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_x_cdm_host_versions()` instead."]
#[js::import = "None"]
pub fn x_cdm_host_versions(&mut self, val: &str) -> &mut Self { self.set_x_cdm_host_versions(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_x_cdm_interface_versions()` instead."]
#[js::import = "None"]
pub fn x_cdm_interface_versions(&mut self, val: &str) -> &mut Self { self.set_x_cdm_interface_versions(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_x_cdm_module_versions()` instead."]
#[js::import = "None"]
pub fn x_cdm_module_versions(&mut self, val: &str) -> &mut Self { self.set_x_cdm_module_versions(val) { panic!("web-sys intrinsic") }
        self
    }
}
