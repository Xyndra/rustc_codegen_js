#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `InputEventInit` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
#[js::opaque]
#[derive(Copy)]
pub struct InputEventInit(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
#[js::import = "None"]
pub fn get_bubbles(this: &InputEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `bubbles` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
#[js::import = "None = $1"]
pub fn set_bubbles(this: &InputEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
#[js::import = "None"]
pub fn get_cancelable(this: &InputEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancelable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
#[js::import = "None = $1"]
pub fn set_cancelable(this: &InputEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
#[js::import = "None"]
pub fn get_composed(this: &InputEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `composed` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
#[js::import = "None = $1"]
pub fn set_composed(this: &InputEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `detail` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
#[js::import = "None"]
pub fn get_detail(this: &InputEventInit) -> Option<i32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `detail` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
#[js::import = "None = $1"]
pub fn set_detail(this: &InputEventInit, val: i32) { panic!("web-sys intrinsic") }
    #[cfg(feature = "Window")]
    #[doc = "Get the `view` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`, `Window`*"]
#[js::import = "None"]
pub fn get_view(this: &InputEventInit) -> Option<Window> { panic!("web-sys intrinsic") }
    #[cfg(feature = "Window")]
    #[doc = "Change the `view` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`, `Window`*"]
#[js::import = "None = $1"]
pub fn set_view(this: &InputEventInit, val: Option<&Window>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
#[js::import = "None"]
pub fn get_data(this: &InputEventInit) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `data` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
#[js::import = "None = $1"]
pub fn set_data(this: &InputEventInit, val: Option<&str>) { panic!("web-sys intrinsic") }
    #[cfg(feature = "DataTransfer")]
    #[doc = "Get the `dataTransfer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`, `InputEventInit`*"]
#[js::import = "None"]
pub fn get_data_transfer(this: &InputEventInit) -> Option<DataTransfer> { panic!("web-sys intrinsic") }
    #[cfg(feature = "DataTransfer")]
    #[doc = "Change the `dataTransfer` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DataTransfer`, `InputEventInit`*"]
#[js::import = "None = $1"]
pub fn set_data_transfer(this: &InputEventInit, val: Option<&DataTransfer>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `inputType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
#[js::import = "None"]
pub fn get_input_type(this: &InputEventInit) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `inputType` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
#[js::import = "None = $1"]
pub fn set_input_type(this: &InputEventInit, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `isComposing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
#[js::import = "None"]
pub fn get_is_composing(this: &InputEventInit) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `isComposing` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
#[js::import = "None = $1"]
pub fn set_is_composing(this: &InputEventInit, val: bool) { panic!("web-sys intrinsic") }
    #[doc = "Get the `targetRanges` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
#[js::import = "None"]
pub fn get_target_ranges(this: &InputEventInit) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
    #[doc = "Change the `targetRanges` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
#[js::import = "None = $1"]
pub fn set_target_ranges(this: &InputEventInit, val: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
impl InputEventInit {
    #[doc = "Construct a new `InputEventInit`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `InputEventInit`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
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
    #[deprecated = "Use `set_detail()` instead."]
#[js::import = "None"]
pub fn detail(&mut self, val: i32) -> &mut Self { self.set_detail(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "Window")]
    #[deprecated = "Use `set_view()` instead."]
#[js::import = "None"]
pub fn view(&mut self, val: Option<&Window>) -> &mut Self { self.set_view(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_data()` instead."]
#[js::import = "None"]
pub fn data(&mut self, val: Option<&str>) -> &mut Self { self.set_data(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "DataTransfer")]
    #[deprecated = "Use `set_data_transfer()` instead."]
#[js::import = "None"]
pub fn data_transfer(&mut self, val: Option<&DataTransfer>) -> &mut Self { self.set_data_transfer(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_input_type()` instead."]
#[js::import = "None"]
pub fn input_type(&mut self, val: &str) -> &mut Self { self.set_input_type(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_is_composing()` instead."]
#[js::import = "None"]
pub fn is_composing(&mut self, val: bool) -> &mut Self { self.set_is_composing(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_target_ranges()` instead."]
#[js::import = "None"]
pub fn target_ranges(&mut self, val: &::wasm_bindgen::JsValue) -> &mut Self { self.set_target_ranges(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for InputEventInit {
    fn default() -> Self {
        Self::new()
    }
}
