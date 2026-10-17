#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FontFaceSetLoadEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSetLoadEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetLoadEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct FontFaceSetLoadEvent(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `fontfaces` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSetLoadEvent/fontfaces)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetLoadEvent`*"]
impl FontFaceSetLoadEvent {
    #[js::import = "$0.fontfaces"]
pub fn fontfaces(&self) -> ::js_sys::Array { panic!("web-sys intrinsic") }
}
    #[doc = "The `new FontFaceSetLoadEvent(..)` constructor, creating a new instance of `FontFaceSetLoadEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSetLoadEvent/FontFaceSetLoadEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetLoadEvent`*"]
impl FontFaceSetLoadEvent {
    #[js::import = "new FontFaceSetLoadEvent($0)"]
pub fn new(type_: &str) -> Result<FontFaceSetLoadEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FontFaceSetLoadEventInit")]
    #[doc = "The `new FontFaceSetLoadEvent(..)` constructor, creating a new instance of `FontFaceSetLoadEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSetLoadEvent/FontFaceSetLoadEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSetLoadEvent`, `FontFaceSetLoadEventInit`*"]
impl FontFaceSetLoadEvent {
    #[js::import = "new FontFaceSetLoadEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &FontFaceSetLoadEventInit,) -> Result<FontFaceSetLoadEvent, JsValue> { panic!("web-sys intrinsic") }
}
