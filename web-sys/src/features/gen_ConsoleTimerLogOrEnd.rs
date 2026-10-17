#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ConsoleTimerLogOrEnd` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerLogOrEnd`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ConsoleTimerLogOrEnd(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerLogOrEnd`*"]
#[js::import = "None"]
pub fn get_duration(this: &ConsoleTimerLogOrEnd) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `duration` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerLogOrEnd`*"]
#[js::import = "None = $1"]
pub fn set_duration(this: &ConsoleTimerLogOrEnd, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerLogOrEnd`*"]
#[js::import = "None"]
pub fn get_name(this: &ConsoleTimerLogOrEnd) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `name` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerLogOrEnd`*"]
#[js::import = "None = $1"]
pub fn set_name(this: &ConsoleTimerLogOrEnd, val: &str) { panic!("web-sys intrinsic") }
impl ConsoleTimerLogOrEnd {
    #[doc = "Construct a new `ConsoleTimerLogOrEnd`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ConsoleTimerLogOrEnd`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_duration()` instead."]
#[js::import = "None"]
pub fn duration(&mut self, val: f64) -> &mut Self { self.set_duration(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_name()` instead."]
#[js::import = "None"]
pub fn name(&mut self, val: &str) -> &mut Self { self.set_name(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for ConsoleTimerLogOrEnd {
    fn default() -> Self {
        Self::new()
    }
}
