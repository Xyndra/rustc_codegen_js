#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `NativeOsFileWriteAtomicOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileWriteAtomicOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct NativeOsFileWriteAtomicOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `backupTo` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileWriteAtomicOptions`*"]
#[js::import = "None"]
pub fn get_backup_to(this: &NativeOsFileWriteAtomicOptions) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `backupTo` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileWriteAtomicOptions`*"]
#[js::import = "None = $1"]
pub fn set_backup_to(this: &NativeOsFileWriteAtomicOptions, val: Option<&str>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `bytes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileWriteAtomicOptions`*"]
#[js::import = "None"]
pub fn get_bytes(this: &NativeOsFileWriteAtomicOptions) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bytes` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileWriteAtomicOptions`*"]
#[js::import = "None = $1"]
pub fn set_bytes(this: &NativeOsFileWriteAtomicOptions, val: Option<f64>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `flush` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileWriteAtomicOptions`*"]
#[js::import = "None"]
pub fn get_flush(this: &NativeOsFileWriteAtomicOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `flush` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileWriteAtomicOptions`*"]
#[js::import = "None = $1"]
pub fn set_flush(this: &NativeOsFileWriteAtomicOptions, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `noOverwrite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileWriteAtomicOptions`*"]
#[js::import = "None"]
pub fn get_no_overwrite(this: &NativeOsFileWriteAtomicOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `noOverwrite` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileWriteAtomicOptions`*"]
#[js::import = "None = $1"]
pub fn set_no_overwrite(this: &NativeOsFileWriteAtomicOptions, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `tmpPath` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileWriteAtomicOptions`*"]
#[js::import = "None"]
pub fn get_tmp_path(this: &NativeOsFileWriteAtomicOptions) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `tmpPath` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileWriteAtomicOptions`*"]
#[js::import = "None = $1"]
pub fn set_tmp_path(this: &NativeOsFileWriteAtomicOptions, val: Option<&str>) { panic!("web-sys intrinsic") }
impl NativeOsFileWriteAtomicOptions {
    #[doc = "Construct a new `NativeOsFileWriteAtomicOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `NativeOsFileWriteAtomicOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_backup_to()` instead."]
#[js::import = "None"]
pub fn backup_to(&mut self, val: Option<&str>) -> &mut Self { self.set_backup_to(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_bytes()` instead."]
#[js::import = "None"]
pub fn bytes(&mut self, val: Option<f64>) -> &mut Self { self.set_bytes(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_flush()` instead."]
#[js::import = "None"]
pub fn flush(&mut self, val: bool) -> &mut Self { self.set_flush(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_no_overwrite()` instead."]
#[js::import = "None"]
pub fn no_overwrite(&mut self, val: bool) -> &mut Self { self.set_no_overwrite(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_tmp_path()` instead."]
#[js::import = "None"]
pub fn tmp_path(&mut self, val: Option<&str>) -> &mut Self { self.set_tmp_path(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for NativeOsFileWriteAtomicOptions {
    fn default() -> Self {
        Self::new()
    }
}
