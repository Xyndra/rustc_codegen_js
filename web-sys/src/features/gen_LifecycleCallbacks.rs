#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `LifecycleCallbacks` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LifecycleCallbacks`*"]
#[js::opaque]
#[derive(Copy)]
pub struct LifecycleCallbacks(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `adoptedCallback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LifecycleCallbacks`*"]
#[js::import = "None"]
pub fn get_adopted_callback(this: &LifecycleCallbacks) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
    #[doc = "Change the `adoptedCallback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LifecycleCallbacks`*"]
#[js::import = "None = $1"]
pub fn set_adopted_callback(this: &LifecycleCallbacks, val: &::js_sys::Function) { panic!("web-sys intrinsic") }
    #[doc = "Get the `attributeChangedCallback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LifecycleCallbacks`*"]
#[js::import = "None"]
pub fn get_attribute_changed_callback(this: &LifecycleCallbacks) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
    #[doc = "Change the `attributeChangedCallback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LifecycleCallbacks`*"]
#[js::import = "None = $1"]
pub fn set_attribute_changed_callback(this: &LifecycleCallbacks, val: &::js_sys::Function) { panic!("web-sys intrinsic") }
    #[doc = "Get the `connectedCallback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LifecycleCallbacks`*"]
#[js::import = "None"]
pub fn get_connected_callback(this: &LifecycleCallbacks) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
    #[doc = "Change the `connectedCallback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LifecycleCallbacks`*"]
#[js::import = "None = $1"]
pub fn set_connected_callback(this: &LifecycleCallbacks, val: &::js_sys::Function) { panic!("web-sys intrinsic") }
    #[doc = "Get the `disconnectedCallback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LifecycleCallbacks`*"]
#[js::import = "None"]
pub fn get_disconnected_callback(this: &LifecycleCallbacks) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
    #[doc = "Change the `disconnectedCallback` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LifecycleCallbacks`*"]
#[js::import = "None = $1"]
pub fn set_disconnected_callback(this: &LifecycleCallbacks, val: &::js_sys::Function) { panic!("web-sys intrinsic") }
impl LifecycleCallbacks {
    #[doc = "Construct a new `LifecycleCallbacks`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `LifecycleCallbacks`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_adopted_callback()` instead."]
#[js::import = "None"]
pub fn adopted_callback(&mut self, val: &::js_sys::Function) -> &mut Self { self.set_adopted_callback(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_attribute_changed_callback()` instead."]
#[js::import = "None"]
pub fn attribute_changed_callback(&mut self, val: &::js_sys::Function) -> &mut Self { self.set_attribute_changed_callback(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_connected_callback()` instead."]
#[js::import = "None"]
pub fn connected_callback(&mut self, val: &::js_sys::Function) -> &mut Self { self.set_connected_callback(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_disconnected_callback()` instead."]
#[js::import = "None"]
pub fn disconnected_callback(&mut self, val: &::js_sys::Function) -> &mut Self { self.set_disconnected_callback(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for LifecycleCallbacks {
    fn default() -> Self {
        Self::new()
    }
}
