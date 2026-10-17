#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `ReadableWritablePair` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableWritablePair`*"]
#[js::opaque]
#[derive(Copy)]
pub struct ReadableWritablePair(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "ReadableStream")]
    #[doc = "Get the `readable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `ReadableWritablePair`*"]
#[js::import = "None"]
pub fn get_readable(this: &ReadableWritablePair) -> ReadableStream { panic!("web-sys intrinsic") }
    #[cfg(feature = "ReadableStream")]
    #[doc = "Change the `readable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `ReadableWritablePair`*"]
#[js::import = "None = $1"]
pub fn set_readable(this: &ReadableWritablePair, val: &ReadableStream) { panic!("web-sys intrinsic") }
    #[cfg(feature = "WritableStream")]
    #[doc = "Get the `writable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableWritablePair`, `WritableStream`*"]
#[js::import = "None"]
pub fn get_writable(this: &ReadableWritablePair) -> WritableStream { panic!("web-sys intrinsic") }
    #[cfg(feature = "WritableStream")]
    #[doc = "Change the `writable` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableWritablePair`, `WritableStream`*"]
#[js::import = "None = $1"]
pub fn set_writable(this: &ReadableWritablePair, val: &WritableStream) { panic!("web-sys intrinsic") }
impl ReadableWritablePair {
    #[cfg(all(feature = "ReadableStream", feature = "WritableStream",))]
    #[doc = "Construct a new `ReadableWritablePair`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ReadableStream`, `ReadableWritablePair`, `WritableStream`*"]
#[js::import = "None"]
pub fn new(readable: &ReadableStream, writable: &WritableStream) -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret.set_readable(readable);
        ret.set_writable(writable);
        ret
    }
    #[cfg(feature = "ReadableStream")]
    #[deprecated = "Use `set_readable()` instead."]
#[js::import = "None"]
pub fn readable(&mut self, val: &ReadableStream) -> &mut Self { self.set_readable(val) { panic!("web-sys intrinsic") }
        self
    }
    #[cfg(feature = "WritableStream")]
    #[deprecated = "Use `set_writable()` instead."]
#[js::import = "None"]
pub fn writable(&mut self, val: &WritableStream) -> &mut Self { self.set_writable(val) { panic!("web-sys intrinsic") }
        self
    }
}
