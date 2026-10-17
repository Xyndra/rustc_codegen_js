#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UnderlyingSource` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
#[js::opaque]
#[derive(Copy)]
pub struct UnderlyingSource(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `autoAllocateChunkSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
#[js::import = "None"]
pub fn get_auto_allocate_chunk_size(this: &UnderlyingSource) -> Option<f64> { panic!("web-sys intrinsic") }
    #[doc = "Change the `autoAllocateChunkSize` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
#[js::import = "None = $1"]
pub fn set_auto_allocate_chunk_size(this: &UnderlyingSource, val: f64) { panic!("web-sys intrinsic") }
    #[doc = "Get the `cancel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
#[js::import = "None"]
pub fn get_cancel(this: &UnderlyingSource) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
    #[doc = "Change the `cancel` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
#[js::import = "None = $1"]
pub fn set_cancel(this: &UnderlyingSource, val: &::js_sys::Function) { panic!("web-sys intrinsic") }
    #[doc = "Get the `pull` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
#[js::import = "None"]
pub fn get_pull(this: &UnderlyingSource) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
    #[doc = "Change the `pull` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
#[js::import = "None = $1"]
pub fn set_pull(this: &UnderlyingSource, val: &::js_sys::Function) { panic!("web-sys intrinsic") }
    #[doc = "Get the `start` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
#[js::import = "None"]
pub fn get_start(this: &UnderlyingSource) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
    #[doc = "Change the `start` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
#[js::import = "None = $1"]
pub fn set_start(this: &UnderlyingSource, val: &::js_sys::Function) { panic!("web-sys intrinsic") }
    #[cfg(feature = "ReadableStreamType")]
    #[doc = "Get the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamType`, `UnderlyingSource`*"]
#[js::import = "None"]
pub fn get_type(this: &UnderlyingSource) -> Option<ReadableStreamType> { panic!("web-sys intrinsic") }
    #[cfg(feature = "ReadableStreamType")]
    #[doc = "Change the `type` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStreamType`, `UnderlyingSource`*"]
#[js::import = "None = $1"]
pub fn set_type(this: &UnderlyingSource, val: ReadableStreamType) { panic!("web-sys intrinsic") }
impl UnderlyingSource {
    #[doc = "Construct a new `UnderlyingSource`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UnderlyingSource`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_auto_allocate_chunk_size()` instead."]
#[js::import = "None"]
pub fn auto_allocate_chunk_size(&mut self, val: f64) -> &mut Self { self.set_auto_allocate_chunk_size(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_cancel()` instead."]
#[js::import = "None"]
pub fn cancel(&mut self, val: &::js_sys::Function) -> &mut Self { self.set_cancel(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_pull()` instead."]
#[js::import = "None"]
pub fn pull(&mut self, val: &::js_sys::Function) -> &mut Self { self.set_pull(val) { panic!("web-sys intrinsic") }
        self
    }
    #[deprecated = "Use `set_start()` instead."]
#[js::import = "None"]
pub fn start(&mut self, val: &::js_sys::Function) -> &mut Self { self.set_start(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "ReadableStreamType")]
    #[deprecated = "Use `set_type()` instead."]
#[js::import = "None"]
pub fn type_(&mut self, val: ReadableStreamType) -> &mut Self { self.set_type(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for UnderlyingSource {
    fn default() -> Self {
        Self::new()
    }
}
