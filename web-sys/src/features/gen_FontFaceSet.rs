#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FontFaceSet` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`*"]
#[js::opaque]
#[derive(Copy)]
pub struct FontFaceSet(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `size` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/size)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`*"]
impl FontFaceSet {
    #[js::import = "$0.size"]
pub fn size(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onloading` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/onloading)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`*"]
impl FontFaceSet {
    #[js::import = "$0.onloading"]
pub fn onloading(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onloading` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/onloading)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`*"]
impl FontFaceSet {
    #[js::import = "$0.onloading = $1"]
pub fn set_onloading(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onloadingdone` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/onloadingdone)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`*"]
impl FontFaceSet {
    #[js::import = "$0.onloadingdone"]
pub fn onloadingdone(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onloadingdone` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/onloadingdone)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`*"]
impl FontFaceSet {
    #[js::import = "$0.onloadingdone = $1"]
pub fn set_onloadingdone(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onloadingerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/onloadingerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`*"]
impl FontFaceSet {
    #[js::import = "$0.onloadingerror"]
pub fn onloadingerror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onloadingerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/onloadingerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`*"]
impl FontFaceSet {
    #[js::import = "$0.onloadingerror = $1"]
pub fn set_onloadingerror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ready` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/ready)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`*"]
impl FontFaceSet {
    #[js::import = "$0.ready"]
pub fn ready(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FontFaceSetLoadStatus")]
    #[doc = "Getter for the `status` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/status)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`, `FontFaceSetLoadStatus`*"]
impl FontFaceSet {
    #[js::import = "$0.status"]
pub fn status(&self) -> FontFaceSetLoadStatus { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FontFace")]
    #[doc = "The `add()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/add)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`, `FontFaceSet`*"]
impl FontFaceSet {
    #[js::import = "$0.add($1)"]
pub fn add(&self, font: &FontFace) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `check()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/check)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`*"]
impl FontFaceSet {
    #[js::import = "$0.check($1)"]
pub fn check(&self, font: &str) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `check()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/check)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`*"]
impl FontFaceSet {
    #[js::import = "$0.check($1, $2)"]
pub fn check_with_text(&self, font: &str, text: &str) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `clear()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/clear)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`*"]
impl FontFaceSet {
    #[js::import = "$0.clear()"]
pub fn clear(&self) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FontFace")]
    #[doc = "The `delete()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/delete)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`, `FontFaceSet`*"]
impl FontFaceSet {
    #[js::import = "$0.delete($1)"]
pub fn delete(&self, font: &FontFace) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FontFaceSetIterator")]
    #[doc = "The `entries()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/entries)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`, `FontFaceSetIterator`*"]
impl FontFaceSet {
    #[js::import = "$0.entries()"]
pub fn entries(&self) -> FontFaceSetIterator { panic!("web-sys intrinsic") }
}
    #[doc = "The `forEach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/forEach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`*"]
impl FontFaceSet {
    #[js::import = "$0.forEach($1)"]
pub fn for_each(&self, cb: &::js_sys::Function) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `forEach()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/forEach)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`*"]
impl FontFaceSet {
    #[js::import = "$0.forEach($1, $2)"]
pub fn for_each_with_this_arg(&self, cb: &::js_sys::Function, this_arg: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FontFace")]
    #[doc = "The `has()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/has)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFace`, `FontFaceSet`*"]
impl FontFaceSet {
    #[js::import = "$0.has($1)"]
pub fn has(&self, font: &FontFace) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `load()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/load)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`*"]
impl FontFaceSet {
    #[js::import = "$0.load($1)"]
pub fn load(&self, font: &str) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `load()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/load)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`*"]
impl FontFaceSet {
    #[js::import = "$0.load($1, $2)"]
pub fn load_with_text(&self, font: &str, text: &str) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "FontFaceSetIterator")]
    #[doc = "The `values()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceSet/values)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FontFaceSet`, `FontFaceSetIterator`*"]
impl FontFaceSet {
    #[js::import = "$0.values()"]
pub fn values(&self) -> FontFaceSetIterator { panic!("web-sys intrinsic") }
}
