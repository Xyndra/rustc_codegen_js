#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ProfileTimelineStackFrame` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ProfileTimelineStackFrame(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `asyncCause` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
#[js::import = "None"]
pub fn get_async_cause(this: &ProfileTimelineStackFrame) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `asyncCause` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
#[js::import = "None = $1"]
pub fn set_async_cause(this: &ProfileTimelineStackFrame, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `asyncParent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
#[js::import = "None"]
pub fn get_async_parent(this: &ProfileTimelineStackFrame) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
    #[doc = "Change the `asyncParent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
#[js::import = "None = $1"]
pub fn set_async_parent(this: &ProfileTimelineStackFrame, val: Option<&::js_sys::Object>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `column` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
#[js::import = "None"]
pub fn get_column(this: &ProfileTimelineStackFrame) -> Option<i32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `column` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
#[js::import = "None = $1"]
pub fn set_column(this: &ProfileTimelineStackFrame, val: i32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `functionDisplayName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
#[js::import = "None"]
pub fn get_function_display_name(this: &ProfileTimelineStackFrame,) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `functionDisplayName` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
#[js::import = "None = $1"]
pub fn set_function_display_name(this: &ProfileTimelineStackFrame, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `line` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
#[js::import = "None"]
pub fn get_line(this: &ProfileTimelineStackFrame) -> Option<i32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `line` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
#[js::import = "None = $1"]
pub fn set_line(this: &ProfileTimelineStackFrame, val: i32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `parent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
#[js::import = "None"]
pub fn get_parent(this: &ProfileTimelineStackFrame) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
    #[doc = "Change the `parent` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
#[js::import = "None = $1"]
pub fn set_parent(this: &ProfileTimelineStackFrame, val: Option<&::js_sys::Object>) { panic!("web-sys intrinsic") }
    #[doc = "Get the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
#[js::import = "None"]
pub fn get_source(this: &ProfileTimelineStackFrame) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `source` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
#[js::import = "None = $1"]
pub fn set_source(this: &ProfileTimelineStackFrame, val: &str) { panic!("web-sys intrinsic") }
impl ProfileTimelineStackFrame {
    #[doc = "Construct a new `ProfileTimelineStackFrame`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ProfileTimelineStackFrame`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_async_cause()` instead."]
#[js::import = "None"]
pub fn async_cause(&mut self, val: &str) -> &mut Self { self.set_async_cause(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_async_parent()` instead."]
#[js::import = "None"]
pub fn async_parent(&mut self, val: Option<&::js_sys::Object>) -> &mut Self { self.set_async_parent(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_column()` instead."]
#[js::import = "None"]
pub fn column(&mut self, val: i32) -> &mut Self { self.set_column(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_function_display_name()` instead."]
#[js::import = "None"]
pub fn function_display_name(&mut self, val: &str) -> &mut Self { self.set_function_display_name(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_line()` instead."]
#[js::import = "None"]
pub fn line(&mut self, val: i32) -> &mut Self { self.set_line(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_parent()` instead."]
#[js::import = "None"]
pub fn parent(&mut self, val: Option<&::js_sys::Object>) -> &mut Self { self.set_parent(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_source()` instead."]
#[js::import = "None"]
pub fn source(&mut self, val: &str) -> &mut Self { self.set_source(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ProfileTimelineStackFrame {
    fn default() -> Self {
        Self::new()
    }
}
