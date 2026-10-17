#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DateTimeValue` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DateTimeValue(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `day` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
#[js::import = "None"]
pub fn get_day(this: &DateTimeValue) -> Option<i32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `day` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
#[js::import = "None = $1"]
pub fn set_day(this: &DateTimeValue, val: i32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `hour` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
#[js::import = "None"]
pub fn get_hour(this: &DateTimeValue) -> Option<i32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `hour` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
#[js::import = "None = $1"]
pub fn set_hour(this: &DateTimeValue, val: i32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `minute` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
#[js::import = "None"]
pub fn get_minute(this: &DateTimeValue) -> Option<i32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `minute` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
#[js::import = "None = $1"]
pub fn set_minute(this: &DateTimeValue, val: i32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `month` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
#[js::import = "None"]
pub fn get_month(this: &DateTimeValue) -> Option<i32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `month` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
#[js::import = "None = $1"]
pub fn set_month(this: &DateTimeValue, val: i32) { panic!("web-sys intrinsic") }
    #[doc = "Get the `year` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
#[js::import = "None"]
pub fn get_year(this: &DateTimeValue) -> Option<i32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `year` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
#[js::import = "None = $1"]
pub fn set_year(this: &DateTimeValue, val: i32) { panic!("web-sys intrinsic") }
impl DateTimeValue {
    #[doc = "Construct a new `DateTimeValue`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DateTimeValue`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_day()` instead."]
#[js::import = "None"]
pub fn day(&mut self, val: i32) -> &mut Self { self.set_day(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_hour()` instead."]
#[js::import = "None"]
pub fn hour(&mut self, val: i32) -> &mut Self { self.set_hour(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_minute()` instead."]
#[js::import = "None"]
pub fn minute(&mut self, val: i32) -> &mut Self { self.set_minute(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_month()` instead."]
#[js::import = "None"]
pub fn month(&mut self, val: i32) -> &mut Self { self.set_month(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_year()` instead."]
#[js::import = "None"]
pub fn year(&mut self, val: i32) -> &mut Self { self.set_year(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for DateTimeValue {
    fn default() -> Self {
        Self::new()
    }
}
