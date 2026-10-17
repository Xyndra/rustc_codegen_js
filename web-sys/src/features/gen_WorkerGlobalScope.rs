#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WorkerGlobalScope` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
#[js::opaque]
#[derive(Copy)]
pub struct WorkerGlobalScope(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `self` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/self)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.self"]
pub fn self_(&self) -> WorkerGlobalScope { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WorkerLocation")]
    #[doc = "Getter for the `location` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/location)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`, `WorkerLocation`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.location"]
pub fn location(&self) -> WorkerLocation { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WorkerNavigator")]
    #[doc = "Getter for the `navigator` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/navigator)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`, `WorkerNavigator`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.navigator"]
pub fn navigator(&self) -> WorkerNavigator { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.onerror"]
pub fn onerror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.onerror = $1"]
pub fn set_onerror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onoffline` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/onoffline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.onoffline"]
pub fn onoffline(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onoffline` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/onoffline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.onoffline = $1"]
pub fn set_onoffline(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ononline` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/ononline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.ononline"]
pub fn ononline(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ononline` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/ononline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.ononline = $1"]
pub fn set_ononline(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Crypto")]
    #[doc = "Getter for the `crypto` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/crypto)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Crypto`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.crypto"]
pub fn crypto(&self) -> Result<Crypto, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbFactory")]
    #[doc = "Getter for the `indexedDB` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/indexedDB)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFactory`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.indexedDB"]
pub fn indexed_db(&self) -> Result<Option<IdbFactory>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Performance")]
    #[doc = "Getter for the `performance` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/performance)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Performance`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.performance"]
pub fn performance(&self) -> Option<Performance> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `origin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/origin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.origin"]
pub fn origin(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `isSecureContext` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/isSecureContext)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.isSecureContext"]
pub fn is_secure_context(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CacheStorage")]
    #[doc = "Getter for the `caches` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/caches)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheStorage`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.caches"]
pub fn caches(&self) -> Result<CacheStorage, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Scheduler")]
    #[doc = "Getter for the `scheduler` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/scheduler)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Scheduler`, `WorkerGlobalScope`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl WorkerGlobalScope {
    #[js::import = "$0.scheduler"]
pub fn scheduler(&self) -> Scheduler { panic!("web-sys intrinsic") }
}
    #[doc = "The `importScripts()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/importScripts)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.importScripts($1)"]
pub fn import_scripts(&self, urls: &::js_sys::Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `importScripts()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/importScripts)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.importScripts()"]
pub fn import_scripts_0(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `importScripts()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/importScripts)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.importScripts($1)"]
pub fn import_scripts_1(&self, urls_1: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `importScripts()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/importScripts)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.importScripts($1, $2)"]
pub fn import_scripts_2(&self, urls_1: &str, urls_2: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `importScripts()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/importScripts)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.importScripts($1, $2, $3)"]
pub fn import_scripts_3(&self, urls_1: &str, urls_2: &str, urls_3: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `importScripts()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/importScripts)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.importScripts($1, $2, $3, $4)"]
pub fn import_scripts_4(&self, urls_1: &str, urls_2: &str, urls_3: &str, urls_4: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `importScripts()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/importScripts)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.importScripts($1, $2, $3, $4, $5)"]
pub fn import_scripts_5(&self, urls_1: &str, urls_2: &str, urls_3: &str, urls_4: &str, urls_5: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `importScripts()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/importScripts)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.importScripts($1, $2, $3, $4, $5, $6)"]
pub fn import_scripts_6(&self, urls_1: &str, urls_2: &str, urls_3: &str, urls_4: &str, urls_5: &str, urls_6: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `importScripts()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/importScripts)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.importScripts($1, $2, $3, $4, $5, $6, $7)"]
pub fn import_scripts_7(&self, urls_1: &str, urls_2: &str, urls_3: &str, urls_4: &str, urls_5: &str, urls_6: &str, urls_7: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `atob()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/atob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.atob($1)"]
pub fn atob(&self, atob: &str) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `btoa()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/btoa)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.btoa($1)"]
pub fn btoa(&self, btoa: &str) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `clearInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/clearInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.clearInterval()"]
pub fn clear_interval(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `clearInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/clearInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.clearInterval($1)"]
pub fn clear_interval_with_handle(&self, handle: i32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `clearTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/clearTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.clearTimeout()"]
pub fn clear_timeout(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `clearTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/clearTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.clearTimeout($1)"]
pub fn clear_timeout_with_handle(&self, handle: i32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlImageElement")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlImageElement`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.createImageBitmap($1)"]
pub fn create_image_bitmap_with_html_image_element(&self, a_image: &HtmlImageElement) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgImageElement")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgImageElement`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.createImageBitmap($1)"]
pub fn create_image_bitmap_with_svg_image_element(&self, a_image: &SvgImageElement) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlCanvasElement")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.createImageBitmap($1)"]
pub fn create_image_bitmap_with_html_canvas_element(&self, a_image: &HtmlCanvasElement) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlVideoElement")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.createImageBitmap($1)"]
pub fn create_image_bitmap_with_html_video_element(&self, a_image: &HtmlVideoElement) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.createImageBitmap($1)"]
pub fn create_image_bitmap_with_image_bitmap(&self, a_image: &ImageBitmap) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "OffscreenCanvas")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvas`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.createImageBitmap($1)"]
pub fn create_image_bitmap_with_offscreen_canvas(&self, a_image: &OffscreenCanvas) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoFrame")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`, `WorkerGlobalScope`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl WorkerGlobalScope {
    #[js::import = "$0.createImageBitmap($1)"]
pub fn create_image_bitmap_with_video_frame(&self, a_image: &VideoFrame) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Blob")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.createImageBitmap($1)"]
pub fn create_image_bitmap_with_blob(&self, a_image: &Blob) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ImageData")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.createImageBitmap($1)"]
pub fn create_image_bitmap_with_image_data(&self, a_image: &ImageData) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "HtmlImageElement", feature = "ImageBitmapOptions",))]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlImageElement`, `ImageBitmapOptions`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.createImageBitmap($1, $2)"]
pub fn create_image_bitmap_with_html_image_element_and_image_bitmap_options(&self, a_image: &HtmlImageElement, a_options: &ImageBitmapOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "ImageBitmapOptions", feature = "SvgImageElement",))]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `SvgImageElement`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.createImageBitmap($1, $2)"]
pub fn create_image_bitmap_with_svg_image_element_and_image_bitmap_options(&self, a_image: &SvgImageElement, a_options: &ImageBitmapOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "HtmlCanvasElement", feature = "ImageBitmapOptions",))]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `ImageBitmapOptions`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.createImageBitmap($1, $2)"]
pub fn create_image_bitmap_with_html_canvas_element_and_image_bitmap_options(&self, a_image: &HtmlCanvasElement, a_options: &ImageBitmapOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "HtmlVideoElement", feature = "ImageBitmapOptions",))]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`, `ImageBitmapOptions`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.createImageBitmap($1, $2)"]
pub fn create_image_bitmap_with_html_video_element_and_image_bitmap_options(&self, a_image: &HtmlVideoElement, a_options: &ImageBitmapOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "ImageBitmap", feature = "ImageBitmapOptions",))]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`, `ImageBitmapOptions`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.createImageBitmap($1, $2)"]
pub fn create_image_bitmap_with_image_bitmap_and_image_bitmap_options(&self, a_image: &ImageBitmap, a_options: &ImageBitmapOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "ImageBitmapOptions", feature = "OffscreenCanvas",))]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `OffscreenCanvas`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.createImageBitmap($1, $2)"]
pub fn create_image_bitmap_with_offscreen_canvas_and_image_bitmap_options(&self, a_image: &OffscreenCanvas, a_options: &ImageBitmapOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "ImageBitmapOptions", feature = "VideoFrame",))]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `VideoFrame`, `WorkerGlobalScope`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl WorkerGlobalScope {
    #[js::import = "$0.createImageBitmap($1, $2)"]
pub fn create_image_bitmap_with_video_frame_and_image_bitmap_options(&self, a_image: &VideoFrame, a_options: &ImageBitmapOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "Blob", feature = "ImageBitmapOptions",))]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `ImageBitmapOptions`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.createImageBitmap($1, $2)"]
pub fn create_image_bitmap_with_blob_and_image_bitmap_options(&self, a_image: &Blob, a_options: &ImageBitmapOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "ImageBitmapOptions", feature = "ImageData",))]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `ImageData`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.createImageBitmap($1, $2)"]
pub fn create_image_bitmap_with_image_data_and_image_bitmap_options(&self, a_image: &ImageData, a_options: &ImageBitmapOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlImageElement")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlImageElement`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.createImageBitmap($1, $2, $3, $4, $5)"]
pub fn create_image_bitmap_with_html_image_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh(&self, a_image: &HtmlImageElement, a_sx: i32, a_sy: i32, a_sw: i32, a_sh: i32) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgImageElement")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgImageElement`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.createImageBitmap($1, $2, $3, $4, $5)"]
pub fn create_image_bitmap_with_svg_image_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh(&self, a_image: &SvgImageElement, a_sx: i32, a_sy: i32, a_sw: i32, a_sh: i32) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlCanvasElement")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.createImageBitmap($1, $2, $3, $4, $5)"]
pub fn create_image_bitmap_with_html_canvas_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh(&self, a_image: &HtmlCanvasElement, a_sx: i32, a_sy: i32, a_sw: i32, a_sh: i32) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlVideoElement")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.createImageBitmap($1, $2, $3, $4, $5)"]
pub fn create_image_bitmap_with_html_video_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh(&self, a_image: &HtmlVideoElement, a_sx: i32, a_sy: i32, a_sw: i32, a_sh: i32) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.createImageBitmap($1, $2, $3, $4, $5)"]
pub fn create_image_bitmap_with_image_bitmap_and_a_sx_and_a_sy_and_a_sw_and_a_sh(&self, a_image: &ImageBitmap, a_sx: i32, a_sy: i32, a_sw: i32, a_sh: i32) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "OffscreenCanvas")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvas`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.createImageBitmap($1, $2, $3, $4, $5)"]
pub fn create_image_bitmap_with_offscreen_canvas_and_a_sx_and_a_sy_and_a_sw_and_a_sh(&self, a_image: &OffscreenCanvas, a_sx: i32, a_sy: i32, a_sw: i32, a_sh: i32) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoFrame")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`, `WorkerGlobalScope`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl WorkerGlobalScope {
    #[js::import = "$0.createImageBitmap($1, $2, $3, $4, $5)"]
pub fn create_image_bitmap_with_video_frame_and_a_sx_and_a_sy_and_a_sw_and_a_sh(&self, a_image: &VideoFrame, a_sx: i32, a_sy: i32, a_sw: i32, a_sh: i32) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Blob")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.createImageBitmap($1, $2, $3, $4, $5)"]
pub fn create_image_bitmap_with_blob_and_a_sx_and_a_sy_and_a_sw_and_a_sh(&self, a_image: &Blob, a_sx: i32, a_sy: i32, a_sw: i32, a_sh: i32) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ImageData")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.createImageBitmap($1, $2, $3, $4, $5)"]
pub fn create_image_bitmap_with_image_data_and_a_sx_and_a_sy_and_a_sw_and_a_sh(&self, a_image: &ImageData, a_sx: i32, a_sy: i32, a_sw: i32, a_sh: i32) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "HtmlImageElement", feature = "ImageBitmapOptions",))]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlImageElement`, `ImageBitmapOptions`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.createImageBitmap($1, $2, $3, $4, $5, $6)"]
pub fn create_image_bitmap_with_html_image_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh_and_a_options(&self, a_image: &HtmlImageElement, a_sx: i32, a_sy: i32, a_sw: i32, a_sh: i32, a_options: &ImageBitmapOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "ImageBitmapOptions", feature = "SvgImageElement",))]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `SvgImageElement`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.createImageBitmap($1, $2, $3, $4, $5, $6)"]
pub fn create_image_bitmap_with_svg_image_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh_and_a_options(&self, a_image: &SvgImageElement, a_sx: i32, a_sy: i32, a_sw: i32, a_sh: i32, a_options: &ImageBitmapOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "HtmlCanvasElement", feature = "ImageBitmapOptions",))]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `ImageBitmapOptions`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.createImageBitmap($1, $2, $3, $4, $5, $6)"]
pub fn create_image_bitmap_with_html_canvas_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh_and_a_options(&self, a_image: &HtmlCanvasElement, a_sx: i32, a_sy: i32, a_sw: i32, a_sh: i32, a_options: &ImageBitmapOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "HtmlVideoElement", feature = "ImageBitmapOptions",))]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`, `ImageBitmapOptions`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.createImageBitmap($1, $2, $3, $4, $5, $6)"]
pub fn create_image_bitmap_with_html_video_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh_and_a_options(&self, a_image: &HtmlVideoElement, a_sx: i32, a_sy: i32, a_sw: i32, a_sh: i32, a_options: &ImageBitmapOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "ImageBitmap", feature = "ImageBitmapOptions",))]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`, `ImageBitmapOptions`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.createImageBitmap($1, $2, $3, $4, $5, $6)"]
pub fn create_image_bitmap_with_image_bitmap_and_a_sx_and_a_sy_and_a_sw_and_a_sh_and_a_options(&self, a_image: &ImageBitmap, a_sx: i32, a_sy: i32, a_sw: i32, a_sh: i32, a_options: &ImageBitmapOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "ImageBitmapOptions", feature = "OffscreenCanvas",))]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `OffscreenCanvas`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.createImageBitmap($1, $2, $3, $4, $5, $6)"]
pub fn create_image_bitmap_with_offscreen_canvas_and_a_sx_and_a_sy_and_a_sw_and_a_sh_and_a_options(&self, a_image: &OffscreenCanvas, a_sx: i32, a_sy: i32, a_sw: i32, a_sh: i32, a_options: &ImageBitmapOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "ImageBitmapOptions", feature = "VideoFrame",))]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `VideoFrame`, `WorkerGlobalScope`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl WorkerGlobalScope {
    #[js::import = "$0.createImageBitmap($1, $2, $3, $4, $5, $6)"]
pub fn create_image_bitmap_with_video_frame_and_a_sx_and_a_sy_and_a_sw_and_a_sh_and_a_options(&self, a_image: &VideoFrame, a_sx: i32, a_sy: i32, a_sw: i32, a_sh: i32, a_options: &ImageBitmapOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "Blob", feature = "ImageBitmapOptions",))]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `ImageBitmapOptions`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.createImageBitmap($1, $2, $3, $4, $5, $6)"]
pub fn create_image_bitmap_with_blob_and_a_sx_and_a_sy_and_a_sw_and_a_sh_and_a_options(&self, a_image: &Blob, a_sx: i32, a_sy: i32, a_sw: i32, a_sh: i32, a_options: &ImageBitmapOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "ImageBitmapOptions", feature = "ImageData",))]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `ImageData`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.createImageBitmap($1, $2, $3, $4, $5, $6)"]
pub fn create_image_bitmap_with_image_data_and_a_sx_and_a_sy_and_a_sw_and_a_sh_and_a_options(&self, a_image: &ImageData, a_sx: i32, a_sy: i32, a_sw: i32, a_sh: i32, a_options: &ImageBitmapOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Request")]
    #[doc = "The `fetch()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/fetch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.fetch($1)"]
pub fn fetch_with_request(&self, input: &Request) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `fetch()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/fetch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.fetch($1)"]
pub fn fetch_with_str(&self, input: &str) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "Request", feature = "RequestInit",))]
    #[doc = "The `fetch()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/fetch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`, `RequestInit`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.fetch($1, $2)"]
pub fn fetch_with_request_and_init(&self, input: &Request, init: &RequestInit) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RequestInit")]
    #[doc = "The `fetch()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/fetch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`, `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.fetch($1, $2)"]
pub fn fetch_with_str_and_init(&self, input: &str, init: &RequestInit) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `queueMicrotask()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/queueMicrotask)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.queueMicrotask($1)"]
pub fn queue_microtask(&self, callback: &::js_sys::Function) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setInterval($1)"]
pub fn set_interval_with_callback(&self, handler: &::js_sys::Function) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setInterval($1, $2, $3)"]
pub fn set_interval_with_callback_and_timeout_and_arguments(&self, handler: &::js_sys::Function, timeout: i32, arguments: &::js_sys::Array) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setInterval($1, $2)"]
pub fn set_interval_with_callback_and_timeout_and_arguments_0(&self, handler: &::js_sys::Function, timeout: i32) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setInterval($1, $2, $3)"]
pub fn set_interval_with_callback_and_timeout_and_arguments_1(&self, handler: &::js_sys::Function, timeout: i32, arguments_1: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setInterval($1, $2, $3, $4)"]
pub fn set_interval_with_callback_and_timeout_and_arguments_2(&self, handler: &::js_sys::Function, timeout: i32, arguments_1: &::wasm_bindgen::JsValue, arguments_2: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setInterval($1, $2, $3, $4, $5)"]
pub fn set_interval_with_callback_and_timeout_and_arguments_3(&self, handler: &::js_sys::Function, timeout: i32, arguments_1: &::wasm_bindgen::JsValue, arguments_2: &::wasm_bindgen::JsValue, arguments_3: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setInterval($1, $2, $3, $4, $5, $6)"]
pub fn set_interval_with_callback_and_timeout_and_arguments_4(&self, handler: &::js_sys::Function, timeout: i32, arguments_1: &::wasm_bindgen::JsValue, arguments_2: &::wasm_bindgen::JsValue, arguments_3: &::wasm_bindgen::JsValue, arguments_4: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setInterval($1, $2, $3, $4, $5, $6, $7)"]
pub fn set_interval_with_callback_and_timeout_and_arguments_5(&self, handler: &::js_sys::Function, timeout: i32, arguments_1: &::wasm_bindgen::JsValue, arguments_2: &::wasm_bindgen::JsValue, arguments_3: &::wasm_bindgen::JsValue, arguments_4: &::wasm_bindgen::JsValue, arguments_5: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setInterval($1, $2, $3, $4, $5, $6, $7, $8)"]
pub fn set_interval_with_callback_and_timeout_and_arguments_6(&self, handler: &::js_sys::Function, timeout: i32, arguments_1: &::wasm_bindgen::JsValue, arguments_2: &::wasm_bindgen::JsValue, arguments_3: &::wasm_bindgen::JsValue, arguments_4: &::wasm_bindgen::JsValue, arguments_5: &::wasm_bindgen::JsValue, arguments_6: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setInterval($1, $2, $3, $4, $5, $6, $7, $8, $9)"]
pub fn set_interval_with_callback_and_timeout_and_arguments_7(&self, handler: &::js_sys::Function, timeout: i32, arguments_1: &::wasm_bindgen::JsValue, arguments_2: &::wasm_bindgen::JsValue, arguments_3: &::wasm_bindgen::JsValue, arguments_4: &::wasm_bindgen::JsValue, arguments_5: &::wasm_bindgen::JsValue, arguments_6: &::wasm_bindgen::JsValue, arguments_7: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setInterval($1)"]
pub fn set_interval_with_str(&self, handler: &str) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setInterval($1, $2, $3)"]
pub fn set_interval_with_str_and_timeout_and_unused(&self, handler: &str, timeout: i32, unused: &::js_sys::Array) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setInterval($1, $2)"]
pub fn set_interval_with_str_and_timeout_and_unused_0(&self, handler: &str, timeout: i32) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setInterval($1, $2, $3)"]
pub fn set_interval_with_str_and_timeout_and_unused_1(&self, handler: &str, timeout: i32, unused_1: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setInterval($1, $2, $3, $4)"]
pub fn set_interval_with_str_and_timeout_and_unused_2(&self, handler: &str, timeout: i32, unused_1: &::wasm_bindgen::JsValue, unused_2: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setInterval($1, $2, $3, $4, $5)"]
pub fn set_interval_with_str_and_timeout_and_unused_3(&self, handler: &str, timeout: i32, unused_1: &::wasm_bindgen::JsValue, unused_2: &::wasm_bindgen::JsValue, unused_3: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setInterval($1, $2, $3, $4, $5, $6)"]
pub fn set_interval_with_str_and_timeout_and_unused_4(&self, handler: &str, timeout: i32, unused_1: &::wasm_bindgen::JsValue, unused_2: &::wasm_bindgen::JsValue, unused_3: &::wasm_bindgen::JsValue, unused_4: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setInterval($1, $2, $3, $4, $5, $6, $7)"]
pub fn set_interval_with_str_and_timeout_and_unused_5(&self, handler: &str, timeout: i32, unused_1: &::wasm_bindgen::JsValue, unused_2: &::wasm_bindgen::JsValue, unused_3: &::wasm_bindgen::JsValue, unused_4: &::wasm_bindgen::JsValue, unused_5: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setInterval($1, $2, $3, $4, $5, $6, $7, $8)"]
pub fn set_interval_with_str_and_timeout_and_unused_6(&self, handler: &str, timeout: i32, unused_1: &::wasm_bindgen::JsValue, unused_2: &::wasm_bindgen::JsValue, unused_3: &::wasm_bindgen::JsValue, unused_4: &::wasm_bindgen::JsValue, unused_5: &::wasm_bindgen::JsValue, unused_6: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setInterval($1, $2, $3, $4, $5, $6, $7, $8, $9)"]
pub fn set_interval_with_str_and_timeout_and_unused_7(&self, handler: &str, timeout: i32, unused_1: &::wasm_bindgen::JsValue, unused_2: &::wasm_bindgen::JsValue, unused_3: &::wasm_bindgen::JsValue, unused_4: &::wasm_bindgen::JsValue, unused_5: &::wasm_bindgen::JsValue, unused_6: &::wasm_bindgen::JsValue, unused_7: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setTimeout($1)"]
pub fn set_timeout_with_callback(&self, handler: &::js_sys::Function) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setTimeout($1, $2, $3)"]
pub fn set_timeout_with_callback_and_timeout_and_arguments(&self, handler: &::js_sys::Function, timeout: i32, arguments: &::js_sys::Array) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setTimeout($1, $2)"]
pub fn set_timeout_with_callback_and_timeout_and_arguments_0(&self, handler: &::js_sys::Function, timeout: i32) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setTimeout($1, $2, $3)"]
pub fn set_timeout_with_callback_and_timeout_and_arguments_1(&self, handler: &::js_sys::Function, timeout: i32, arguments_1: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setTimeout($1, $2, $3, $4)"]
pub fn set_timeout_with_callback_and_timeout_and_arguments_2(&self, handler: &::js_sys::Function, timeout: i32, arguments_1: &::wasm_bindgen::JsValue, arguments_2: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setTimeout($1, $2, $3, $4, $5)"]
pub fn set_timeout_with_callback_and_timeout_and_arguments_3(&self, handler: &::js_sys::Function, timeout: i32, arguments_1: &::wasm_bindgen::JsValue, arguments_2: &::wasm_bindgen::JsValue, arguments_3: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setTimeout($1, $2, $3, $4, $5, $6)"]
pub fn set_timeout_with_callback_and_timeout_and_arguments_4(&self, handler: &::js_sys::Function, timeout: i32, arguments_1: &::wasm_bindgen::JsValue, arguments_2: &::wasm_bindgen::JsValue, arguments_3: &::wasm_bindgen::JsValue, arguments_4: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setTimeout($1, $2, $3, $4, $5, $6, $7)"]
pub fn set_timeout_with_callback_and_timeout_and_arguments_5(&self, handler: &::js_sys::Function, timeout: i32, arguments_1: &::wasm_bindgen::JsValue, arguments_2: &::wasm_bindgen::JsValue, arguments_3: &::wasm_bindgen::JsValue, arguments_4: &::wasm_bindgen::JsValue, arguments_5: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setTimeout($1, $2, $3, $4, $5, $6, $7, $8)"]
pub fn set_timeout_with_callback_and_timeout_and_arguments_6(&self, handler: &::js_sys::Function, timeout: i32, arguments_1: &::wasm_bindgen::JsValue, arguments_2: &::wasm_bindgen::JsValue, arguments_3: &::wasm_bindgen::JsValue, arguments_4: &::wasm_bindgen::JsValue, arguments_5: &::wasm_bindgen::JsValue, arguments_6: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setTimeout($1, $2, $3, $4, $5, $6, $7, $8, $9)"]
pub fn set_timeout_with_callback_and_timeout_and_arguments_7(&self, handler: &::js_sys::Function, timeout: i32, arguments_1: &::wasm_bindgen::JsValue, arguments_2: &::wasm_bindgen::JsValue, arguments_3: &::wasm_bindgen::JsValue, arguments_4: &::wasm_bindgen::JsValue, arguments_5: &::wasm_bindgen::JsValue, arguments_6: &::wasm_bindgen::JsValue, arguments_7: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setTimeout($1)"]
pub fn set_timeout_with_str(&self, handler: &str) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setTimeout($1, $2, $3)"]
pub fn set_timeout_with_str_and_timeout_and_unused(&self, handler: &str, timeout: i32, unused: &::js_sys::Array) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setTimeout($1, $2)"]
pub fn set_timeout_with_str_and_timeout_and_unused_0(&self, handler: &str, timeout: i32) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setTimeout($1, $2, $3)"]
pub fn set_timeout_with_str_and_timeout_and_unused_1(&self, handler: &str, timeout: i32, unused_1: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setTimeout($1, $2, $3, $4)"]
pub fn set_timeout_with_str_and_timeout_and_unused_2(&self, handler: &str, timeout: i32, unused_1: &::wasm_bindgen::JsValue, unused_2: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setTimeout($1, $2, $3, $4, $5)"]
pub fn set_timeout_with_str_and_timeout_and_unused_3(&self, handler: &str, timeout: i32, unused_1: &::wasm_bindgen::JsValue, unused_2: &::wasm_bindgen::JsValue, unused_3: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setTimeout($1, $2, $3, $4, $5, $6)"]
pub fn set_timeout_with_str_and_timeout_and_unused_4(&self, handler: &str, timeout: i32, unused_1: &::wasm_bindgen::JsValue, unused_2: &::wasm_bindgen::JsValue, unused_3: &::wasm_bindgen::JsValue, unused_4: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setTimeout($1, $2, $3, $4, $5, $6, $7)"]
pub fn set_timeout_with_str_and_timeout_and_unused_5(&self, handler: &str, timeout: i32, unused_1: &::wasm_bindgen::JsValue, unused_2: &::wasm_bindgen::JsValue, unused_3: &::wasm_bindgen::JsValue, unused_4: &::wasm_bindgen::JsValue, unused_5: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setTimeout($1, $2, $3, $4, $5, $6, $7, $8)"]
pub fn set_timeout_with_str_and_timeout_and_unused_6(&self, handler: &str, timeout: i32, unused_1: &::wasm_bindgen::JsValue, unused_2: &::wasm_bindgen::JsValue, unused_3: &::wasm_bindgen::JsValue, unused_4: &::wasm_bindgen::JsValue, unused_5: &::wasm_bindgen::JsValue, unused_6: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WorkerGlobalScope`*"]
impl WorkerGlobalScope {
    #[js::import = "$0.setTimeout($1, $2, $3, $4, $5, $6, $7, $8, $9)"]
pub fn set_timeout_with_str_and_timeout_and_unused_7(&self, handler: &str, timeout: i32, unused_1: &::wasm_bindgen::JsValue, unused_2: &::wasm_bindgen::JsValue, unused_3: &::wasm_bindgen::JsValue, unused_4: &::wasm_bindgen::JsValue, unused_5: &::wasm_bindgen::JsValue, unused_6: &::wasm_bindgen::JsValue, unused_7: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
