#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `TreeCellInfo` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeCellInfo`*"]
#[js::opaque]
#[derive(Copy)]
pub struct TreeCellInfo(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `childElt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeCellInfo`*"]
#[js::import = "None"]
pub fn get_child_elt(this: &TreeCellInfo) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
    #[doc = "Change the `childElt` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeCellInfo`*"]
#[js::import = "None = $1"]
pub fn set_child_elt(this: &TreeCellInfo, val: &str) { panic!("web-sys intrinsic") }
    #[doc = "Get the `row` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeCellInfo`*"]
#[js::import = "None"]
pub fn get_row(this: &TreeCellInfo) -> Option<i32> { panic!("web-sys intrinsic") }
    #[doc = "Change the `row` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeCellInfo`*"]
#[js::import = "None = $1"]
pub fn set_row(this: &TreeCellInfo, val: i32) { panic!("web-sys intrinsic") }
impl TreeCellInfo {
    #[doc = "Construct a new `TreeCellInfo`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `TreeCellInfo`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_child_elt()` instead."]
#[js::import = "None"]
pub fn child_elt(&mut self, val: &str) -> &mut Self { self.set_child_elt(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_row()` instead."]
#[js::import = "None"]
pub fn row(&mut self, val: i32) -> &mut Self { self.set_row(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for TreeCellInfo {
    fn default() -> Self {
        Self::new()
    }
}
