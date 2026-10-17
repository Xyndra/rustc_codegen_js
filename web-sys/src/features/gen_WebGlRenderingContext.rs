#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `WebGlRenderingContext` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
#[js::opaque]
#[derive(Copy)]
pub struct WebGlRenderingContext(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `canvas` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/canvas)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.canvas"]
pub fn canvas(&self) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `drawingBufferWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/drawingBufferWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.drawingBufferWidth"]
pub fn drawing_buffer_width(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `drawingBufferHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/drawingBufferHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.drawingBufferHeight"]
pub fn drawing_buffer_height(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `bufferData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.bufferData($1, $2, $3)"]
pub fn buffer_data_with_i32(&self, target: u32, size: i32, usage: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `bufferData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.bufferData($1, $2, $3)"]
pub fn buffer_data_with_f64(&self, target: u32, size: f64, usage: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `bufferData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.bufferData($1, $2, $3)"]
pub fn buffer_data_with_opt_array_buffer(&self, target: u32, data: Option<&::js_sys::ArrayBuffer>, usage: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `bufferData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.bufferData($1, $2, $3)"]
pub fn buffer_data_with_array_buffer_view(&self, target: u32, data: &::js_sys::Object, usage: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `bufferData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.bufferData($1, $2, $3)"]
pub fn buffer_data_with_u8_array(&self, target: u32, data: &[u8], usage: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `bufferData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.bufferData($1, $2, $3)"]
pub fn buffer_data_with_js_u8_array(&self, target: u32, data: &::js_sys::Uint8Array, usage: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `bufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.bufferSubData($1, $2, $3)"]
pub fn buffer_sub_data_with_i32_and_array_buffer(&self, target: u32, offset: i32, data: &::js_sys::ArrayBuffer) { panic!("web-sys intrinsic") }
}
    #[doc = "The `bufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.bufferSubData($1, $2, $3)"]
pub fn buffer_sub_data_with_f64_and_array_buffer(&self, target: u32, offset: f64, data: &::js_sys::ArrayBuffer) { panic!("web-sys intrinsic") }
}
    #[doc = "The `bufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.bufferSubData($1, $2, $3)"]
pub fn buffer_sub_data_with_i32_and_array_buffer_view(&self, target: u32, offset: i32, data: &::js_sys::Object) { panic!("web-sys intrinsic") }
}
    #[doc = "The `bufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.bufferSubData($1, $2, $3)"]
pub fn buffer_sub_data_with_f64_and_array_buffer_view(&self, target: u32, offset: f64, data: &::js_sys::Object) { panic!("web-sys intrinsic") }
}
    #[doc = "The `bufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.bufferSubData($1, $2, $3)"]
pub fn buffer_sub_data_with_i32_and_u8_array(&self, target: u32, offset: i32, data: &[u8]) { panic!("web-sys intrinsic") }
}
    #[doc = "The `bufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.bufferSubData($1, $2, $3)"]
pub fn buffer_sub_data_with_f64_and_u8_array(&self, target: u32, offset: f64, data: &[u8]) { panic!("web-sys intrinsic") }
}
    #[doc = "The `bufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.bufferSubData($1, $2, $3)"]
pub fn buffer_sub_data_with_i32_and_js_u8_array(&self, target: u32, offset: i32, data: &::js_sys::Uint8Array) { panic!("web-sys intrinsic") }
}
    #[doc = "The `bufferSubData()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bufferSubData)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.bufferSubData($1, $2, $3)"]
pub fn buffer_sub_data_with_f64_and_js_u8_array(&self, target: u32, offset: f64, data: &::js_sys::Uint8Array) { panic!("web-sys intrinsic") }
}
    #[doc = "The `commit()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/commit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.commit()"]
pub fn commit(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `compressedTexImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/compressedTexImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.compressedTexImage2D($1, $2, $3, $4, $5, $6, $7)"]
pub fn compressed_tex_image_2d_with_array_buffer_view(&self, target: u32, level: i32, internalformat: u32, width: i32, height: i32, border: i32, data: &::js_sys::Object) { panic!("web-sys intrinsic") }
}
    #[doc = "The `compressedTexImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/compressedTexImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.compressedTexImage2D($1, $2, $3, $4, $5, $6, $7)"]
pub fn compressed_tex_image_2d_with_u8_array(&self, target: u32, level: i32, internalformat: u32, width: i32, height: i32, border: i32, data: &[u8]) { panic!("web-sys intrinsic") }
}
    #[doc = "The `compressedTexImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/compressedTexImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.compressedTexImage2D($1, $2, $3, $4, $5, $6, $7)"]
pub fn compressed_tex_image_2d_with_js_u8_array(&self, target: u32, level: i32, internalformat: u32, width: i32, height: i32, border: i32, data: &::js_sys::Uint8Array) { panic!("web-sys intrinsic") }
}
    #[doc = "The `compressedTexSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/compressedTexSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.compressedTexSubImage2D($1, $2, $3, $4, $5, $6, $7, $8)"]
pub fn compressed_tex_sub_image_2d_with_array_buffer_view(&self, target: u32, level: i32, xoffset: i32, yoffset: i32, width: i32, height: i32, format: u32, data: &::js_sys::Object) { panic!("web-sys intrinsic") }
}
    #[doc = "The `compressedTexSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/compressedTexSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.compressedTexSubImage2D($1, $2, $3, $4, $5, $6, $7, $8)"]
pub fn compressed_tex_sub_image_2d_with_u8_array(&self, target: u32, level: i32, xoffset: i32, yoffset: i32, width: i32, height: i32, format: u32, data: &mut [u8]) { panic!("web-sys intrinsic") }
}
    #[doc = "The `compressedTexSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/compressedTexSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.compressedTexSubImage2D($1, $2, $3, $4, $5, $6, $7, $8)"]
pub fn compressed_tex_sub_image_2d_with_js_u8_array(&self, target: u32, level: i32, xoffset: i32, yoffset: i32, width: i32, height: i32, format: u32, data: &::js_sys::Uint8Array) { panic!("web-sys intrinsic") }
}
    #[doc = "The `readPixels()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/readPixels)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.readPixels($1, $2, $3, $4, $5, $6, $7)"]
pub fn read_pixels_with_opt_array_buffer_view(&self, x: i32, y: i32, width: i32, height: i32, format: u32, type_: u32, pixels: Option<&::js_sys::Object>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `readPixels()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/readPixels)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.readPixels($1, $2, $3, $4, $5, $6, $7)"]
pub fn read_pixels_with_opt_u8_array(&self, x: i32, y: i32, width: i32, height: i32, format: u32, type_: u32, pixels: Option<&mut [u8]>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `readPixels()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/readPixels)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.readPixels($1, $2, $3, $4, $5, $6, $7)"]
pub fn read_pixels_with_opt_js_u8_array(&self, x: i32, y: i32, width: i32, height: i32, format: u32, type_: u32, pixels: Option<&::js_sys::Uint8Array>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `texImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.texImage2D($1, $2, $3, $4, $5, $6, $7, $8, $9)"]
pub fn tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(&self, target: u32, level: i32, internalformat: i32, width: i32, height: i32, border: i32, format: u32, type_: u32, pixels: Option<&::js_sys::Object>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `texImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.texImage2D($1, $2, $3, $4, $5, $6, $7, $8, $9)"]
pub fn tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(&self, target: u32, level: i32, internalformat: i32, width: i32, height: i32, border: i32, format: u32, type_: u32, pixels: Option<&[u8]>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `texImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.texImage2D($1, $2, $3, $4, $5, $6, $7, $8, $9)"]
pub fn tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_js_u8_array(&self, target: u32, level: i32, internalformat: i32, width: i32, height: i32, border: i32, format: u32, type_: u32, pixels: Option<&::js_sys::Uint8Array>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ImageBitmap")]
    #[doc = "The `texImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.texImage2D($1, $2, $3, $4, $5, $6)"]
pub fn tex_image_2d_with_u32_and_u32_and_image_bitmap(&self, target: u32, level: i32, internalformat: i32, format: u32, type_: u32, pixels: &ImageBitmap) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ImageData")]
    #[doc = "The `texImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.texImage2D($1, $2, $3, $4, $5, $6)"]
pub fn tex_image_2d_with_u32_and_u32_and_image_data(&self, target: u32, level: i32, internalformat: i32, format: u32, type_: u32, pixels: &ImageData) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlImageElement")]
    #[doc = "The `texImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlImageElement`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.texImage2D($1, $2, $3, $4, $5, $6)"]
pub fn tex_image_2d_with_u32_and_u32_and_image(&self, target: u32, level: i32, internalformat: i32, format: u32, type_: u32, image: &HtmlImageElement) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlCanvasElement")]
    #[doc = "The `texImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.texImage2D($1, $2, $3, $4, $5, $6)"]
pub fn tex_image_2d_with_u32_and_u32_and_canvas(&self, target: u32, level: i32, internalformat: i32, format: u32, type_: u32, canvas: &HtmlCanvasElement) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlVideoElement")]
    #[doc = "The `texImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.texImage2D($1, $2, $3, $4, $5, $6)"]
pub fn tex_image_2d_with_u32_and_u32_and_video(&self, target: u32, level: i32, internalformat: i32, format: u32, type_: u32, video: &HtmlVideoElement) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoFrame")]
    #[doc = "The `texImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`, `WebGlRenderingContext`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl WebGlRenderingContext {
    #[js::import = "$0.texImage2D($1, $2, $3, $4, $5, $6)"]
pub fn tex_image_2d_with_u32_and_u32_and_video_frame(&self, target: u32, level: i32, internalformat: i32, format: u32, type_: u32, video_frame: &VideoFrame) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `texSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.texSubImage2D($1, $2, $3, $4, $5, $6, $7, $8, $9)"]
pub fn tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_array_buffer_view(&self, target: u32, level: i32, xoffset: i32, yoffset: i32, width: i32, height: i32, format: u32, type_: u32, pixels: Option<&::js_sys::Object>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `texSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.texSubImage2D($1, $2, $3, $4, $5, $6, $7, $8, $9)"]
pub fn tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_u8_array(&self, target: u32, level: i32, xoffset: i32, yoffset: i32, width: i32, height: i32, format: u32, type_: u32, pixels: Option<&[u8]>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `texSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.texSubImage2D($1, $2, $3, $4, $5, $6, $7, $8, $9)"]
pub fn tex_sub_image_2d_with_i32_and_i32_and_u32_and_type_and_opt_js_u8_array(&self, target: u32, level: i32, xoffset: i32, yoffset: i32, width: i32, height: i32, format: u32, type_: u32, pixels: Option<&::js_sys::Uint8Array>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ImageBitmap")]
    #[doc = "The `texSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.texSubImage2D($1, $2, $3, $4, $5, $6, $7)"]
pub fn tex_sub_image_2d_with_u32_and_u32_and_image_bitmap(&self, target: u32, level: i32, xoffset: i32, yoffset: i32, format: u32, type_: u32, pixels: &ImageBitmap) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ImageData")]
    #[doc = "The `texSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.texSubImage2D($1, $2, $3, $4, $5, $6, $7)"]
pub fn tex_sub_image_2d_with_u32_and_u32_and_image_data(&self, target: u32, level: i32, xoffset: i32, yoffset: i32, format: u32, type_: u32, pixels: &ImageData) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlImageElement")]
    #[doc = "The `texSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlImageElement`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.texSubImage2D($1, $2, $3, $4, $5, $6, $7)"]
pub fn tex_sub_image_2d_with_u32_and_u32_and_image(&self, target: u32, level: i32, xoffset: i32, yoffset: i32, format: u32, type_: u32, image: &HtmlImageElement) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlCanvasElement")]
    #[doc = "The `texSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.texSubImage2D($1, $2, $3, $4, $5, $6, $7)"]
pub fn tex_sub_image_2d_with_u32_and_u32_and_canvas(&self, target: u32, level: i32, xoffset: i32, yoffset: i32, format: u32, type_: u32, canvas: &HtmlCanvasElement) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlVideoElement")]
    #[doc = "The `texSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.texSubImage2D($1, $2, $3, $4, $5, $6, $7)"]
pub fn tex_sub_image_2d_with_u32_and_u32_and_video(&self, target: u32, level: i32, xoffset: i32, yoffset: i32, format: u32, type_: u32, video: &HtmlVideoElement) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoFrame")]
    #[doc = "The `texSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`, `WebGlRenderingContext`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl WebGlRenderingContext {
    #[js::import = "$0.texSubImage2D($1, $2, $3, $4, $5, $6, $7)"]
pub fn tex_sub_image_2d_with_u32_and_u32_and_video_frame(&self, target: u32, level: i32, xoffset: i32, yoffset: i32, format: u32, type_: u32, video_frame: &VideoFrame) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniform1fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform1fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniform1fv($1, $2)"]
pub fn uniform1fv_with_f32_array(&self, location: Option<&WebGlUniformLocation>, data: &[f32]) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniform1fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform1fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniform1fv($1, $2)"]
pub fn uniform1fv_with_js_f32_array(&self, location: Option<&WebGlUniformLocation>, data: &::js_sys::Float32Array) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniform1fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform1fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniform1fv($1, $2)"]
pub fn uniform1fv_with_f32_sequence(&self, location: Option<&WebGlUniformLocation>, data: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniform1iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform1iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniform1iv($1, $2)"]
pub fn uniform1iv_with_i32_array(&self, location: Option<&WebGlUniformLocation>, data: &[i32]) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniform1iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform1iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniform1iv($1, $2)"]
pub fn uniform1iv_with_js_i32_array(&self, location: Option<&WebGlUniformLocation>, data: &::js_sys::Int32Array) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniform1iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform1iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniform1iv($1, $2)"]
pub fn uniform1iv_with_i32_sequence(&self, location: Option<&WebGlUniformLocation>, data: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniform2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniform2fv($1, $2)"]
pub fn uniform2fv_with_f32_array(&self, location: Option<&WebGlUniformLocation>, data: &[f32]) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniform2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniform2fv($1, $2)"]
pub fn uniform2fv_with_js_f32_array(&self, location: Option<&WebGlUniformLocation>, data: &::js_sys::Float32Array) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniform2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniform2fv($1, $2)"]
pub fn uniform2fv_with_f32_sequence(&self, location: Option<&WebGlUniformLocation>, data: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniform2iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform2iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniform2iv($1, $2)"]
pub fn uniform2iv_with_i32_array(&self, location: Option<&WebGlUniformLocation>, data: &[i32]) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniform2iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform2iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniform2iv($1, $2)"]
pub fn uniform2iv_with_js_i32_array(&self, location: Option<&WebGlUniformLocation>, data: &::js_sys::Int32Array) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniform2iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform2iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniform2iv($1, $2)"]
pub fn uniform2iv_with_i32_sequence(&self, location: Option<&WebGlUniformLocation>, data: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniform3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniform3fv($1, $2)"]
pub fn uniform3fv_with_f32_array(&self, location: Option<&WebGlUniformLocation>, data: &[f32]) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniform3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniform3fv($1, $2)"]
pub fn uniform3fv_with_js_f32_array(&self, location: Option<&WebGlUniformLocation>, data: &::js_sys::Float32Array) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniform3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniform3fv($1, $2)"]
pub fn uniform3fv_with_f32_sequence(&self, location: Option<&WebGlUniformLocation>, data: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniform3iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform3iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniform3iv($1, $2)"]
pub fn uniform3iv_with_i32_array(&self, location: Option<&WebGlUniformLocation>, data: &[i32]) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniform3iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform3iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniform3iv($1, $2)"]
pub fn uniform3iv_with_js_i32_array(&self, location: Option<&WebGlUniformLocation>, data: &::js_sys::Int32Array) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniform3iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform3iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniform3iv($1, $2)"]
pub fn uniform3iv_with_i32_sequence(&self, location: Option<&WebGlUniformLocation>, data: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniform4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniform4fv($1, $2)"]
pub fn uniform4fv_with_f32_array(&self, location: Option<&WebGlUniformLocation>, data: &[f32]) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniform4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniform4fv($1, $2)"]
pub fn uniform4fv_with_js_f32_array(&self, location: Option<&WebGlUniformLocation>, data: &::js_sys::Float32Array) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniform4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniform4fv($1, $2)"]
pub fn uniform4fv_with_f32_sequence(&self, location: Option<&WebGlUniformLocation>, data: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniform4iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform4iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniform4iv($1, $2)"]
pub fn uniform4iv_with_i32_array(&self, location: Option<&WebGlUniformLocation>, data: &[i32]) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniform4iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform4iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniform4iv($1, $2)"]
pub fn uniform4iv_with_js_i32_array(&self, location: Option<&WebGlUniformLocation>, data: &::js_sys::Int32Array) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniform4iv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform4iv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniform4iv($1, $2)"]
pub fn uniform4iv_with_i32_sequence(&self, location: Option<&WebGlUniformLocation>, data: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniformMatrix2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniformMatrix2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniformMatrix2fv($1, $2, $3)"]
pub fn uniform_matrix2fv_with_f32_array(&self, location: Option<&WebGlUniformLocation>, transpose: bool, data: &[f32]) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniformMatrix2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniformMatrix2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniformMatrix2fv($1, $2, $3)"]
pub fn uniform_matrix2fv_with_js_f32_array(&self, location: Option<&WebGlUniformLocation>, transpose: bool, data: &::js_sys::Float32Array) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniformMatrix2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniformMatrix2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniformMatrix2fv($1, $2, $3)"]
pub fn uniform_matrix2fv_with_f32_sequence(&self, location: Option<&WebGlUniformLocation>, transpose: bool, data: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniformMatrix3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniformMatrix3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniformMatrix3fv($1, $2, $3)"]
pub fn uniform_matrix3fv_with_f32_array(&self, location: Option<&WebGlUniformLocation>, transpose: bool, data: &[f32]) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniformMatrix3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniformMatrix3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniformMatrix3fv($1, $2, $3)"]
pub fn uniform_matrix3fv_with_js_f32_array(&self, location: Option<&WebGlUniformLocation>, transpose: bool, data: &::js_sys::Float32Array) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniformMatrix3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniformMatrix3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniformMatrix3fv($1, $2, $3)"]
pub fn uniform_matrix3fv_with_f32_sequence(&self, location: Option<&WebGlUniformLocation>, transpose: bool, data: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniformMatrix4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniformMatrix4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniformMatrix4fv($1, $2, $3)"]
pub fn uniform_matrix4fv_with_f32_array(&self, location: Option<&WebGlUniformLocation>, transpose: bool, data: &[f32]) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniformMatrix4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniformMatrix4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniformMatrix4fv($1, $2, $3)"]
pub fn uniform_matrix4fv_with_js_f32_array(&self, location: Option<&WebGlUniformLocation>, transpose: bool, data: &::js_sys::Float32Array) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniformMatrix4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniformMatrix4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniformMatrix4fv($1, $2, $3)"]
pub fn uniform_matrix4fv_with_f32_sequence(&self, location: Option<&WebGlUniformLocation>, transpose: bool, data: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
}
    #[doc = "The `activeTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/activeTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.activeTexture($1)"]
pub fn active_texture(&self, texture: u32) { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "WebGlProgram", feature = "WebGlShader",))]
    #[doc = "The `attachShader()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/attachShader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`, `WebGlShader`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.attachShader($1, $2)"]
pub fn attach_shader(&self, program: &WebGlProgram, shader: &WebGlShader) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlProgram")]
    #[doc = "The `bindAttribLocation()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bindAttribLocation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.bindAttribLocation($1, $2, $3)"]
pub fn bind_attrib_location(&self, program: &WebGlProgram, index: u32, name: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlBuffer")]
    #[doc = "The `bindBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bindBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlBuffer`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.bindBuffer($1, $2)"]
pub fn bind_buffer(&self, target: u32, buffer: Option<&WebGlBuffer>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlFramebuffer")]
    #[doc = "The `bindFramebuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bindFramebuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlFramebuffer`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.bindFramebuffer($1, $2)"]
pub fn bind_framebuffer(&self, target: u32, framebuffer: Option<&WebGlFramebuffer>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlRenderbuffer")]
    #[doc = "The `bindRenderbuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bindRenderbuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderbuffer`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.bindRenderbuffer($1, $2)"]
pub fn bind_renderbuffer(&self, target: u32, renderbuffer: Option<&WebGlRenderbuffer>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlTexture")]
    #[doc = "The `bindTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/bindTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlTexture`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.bindTexture($1, $2)"]
pub fn bind_texture(&self, target: u32, texture: Option<&WebGlTexture>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `blendColor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/blendColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.blendColor($1, $2, $3, $4)"]
pub fn blend_color(&self, red: f32, green: f32, blue: f32, alpha: f32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `blendEquation()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/blendEquation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.blendEquation($1)"]
pub fn blend_equation(&self, mode: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `blendEquationSeparate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/blendEquationSeparate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.blendEquationSeparate($1, $2)"]
pub fn blend_equation_separate(&self, mode_rgb: u32, mode_alpha: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `blendFunc()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/blendFunc)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.blendFunc($1, $2)"]
pub fn blend_func(&self, sfactor: u32, dfactor: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `blendFuncSeparate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/blendFuncSeparate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.blendFuncSeparate($1, $2, $3, $4)"]
pub fn blend_func_separate(&self, src_rgb: u32, dst_rgb: u32, src_alpha: u32, dst_alpha: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `checkFramebufferStatus()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/checkFramebufferStatus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.checkFramebufferStatus($1)"]
pub fn check_framebuffer_status(&self, target: u32) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `clear()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/clear)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.clear($1)"]
pub fn clear(&self, mask: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `clearColor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/clearColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.clearColor($1, $2, $3, $4)"]
pub fn clear_color(&self, red: f32, green: f32, blue: f32, alpha: f32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `clearDepth()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/clearDepth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.clearDepth($1)"]
pub fn clear_depth(&self, depth: f32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `clearStencil()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/clearStencil)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.clearStencil($1)"]
pub fn clear_stencil(&self, s: i32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `colorMask()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/colorMask)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.colorMask($1, $2, $3, $4)"]
pub fn color_mask(&self, red: bool, green: bool, blue: bool, alpha: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlShader")]
    #[doc = "The `compileShader()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/compileShader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlShader`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.compileShader($1)"]
pub fn compile_shader(&self, shader: &WebGlShader) { panic!("web-sys intrinsic") }
}
    #[doc = "The `copyTexImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/copyTexImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.copyTexImage2D($1, $2, $3, $4, $5, $6, $7, $8)"]
pub fn copy_tex_image_2d(&self, target: u32, level: i32, internalformat: u32, x: i32, y: i32, width: i32, height: i32, border: i32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `copyTexSubImage2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/copyTexSubImage2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.copyTexSubImage2D($1, $2, $3, $4, $5, $6, $7, $8)"]
pub fn copy_tex_sub_image_2d(&self, target: u32, level: i32, xoffset: i32, yoffset: i32, x: i32, y: i32, width: i32, height: i32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlBuffer")]
    #[doc = "The `createBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/createBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlBuffer`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.createBuffer()"]
pub fn create_buffer(&self) -> Option<WebGlBuffer> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlFramebuffer")]
    #[doc = "The `createFramebuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/createFramebuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlFramebuffer`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.createFramebuffer()"]
pub fn create_framebuffer(&self) -> Option<WebGlFramebuffer> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlProgram")]
    #[doc = "The `createProgram()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/createProgram)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.createProgram()"]
pub fn create_program(&self) -> Option<WebGlProgram> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlRenderbuffer")]
    #[doc = "The `createRenderbuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/createRenderbuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderbuffer`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.createRenderbuffer()"]
pub fn create_renderbuffer(&self) -> Option<WebGlRenderbuffer> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlShader")]
    #[doc = "The `createShader()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/createShader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlShader`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.createShader($1)"]
pub fn create_shader(&self, type_: u32) -> Option<WebGlShader> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlTexture")]
    #[doc = "The `createTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/createTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlTexture`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.createTexture()"]
pub fn create_texture(&self) -> Option<WebGlTexture> { panic!("web-sys intrinsic") }
}
    #[doc = "The `cullFace()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/cullFace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.cullFace($1)"]
pub fn cull_face(&self, mode: u32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlBuffer")]
    #[doc = "The `deleteBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/deleteBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlBuffer`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.deleteBuffer($1)"]
pub fn delete_buffer(&self, buffer: Option<&WebGlBuffer>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlFramebuffer")]
    #[doc = "The `deleteFramebuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/deleteFramebuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlFramebuffer`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.deleteFramebuffer($1)"]
pub fn delete_framebuffer(&self, framebuffer: Option<&WebGlFramebuffer>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlProgram")]
    #[doc = "The `deleteProgram()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/deleteProgram)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.deleteProgram($1)"]
pub fn delete_program(&self, program: Option<&WebGlProgram>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlRenderbuffer")]
    #[doc = "The `deleteRenderbuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/deleteRenderbuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderbuffer`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.deleteRenderbuffer($1)"]
pub fn delete_renderbuffer(&self, renderbuffer: Option<&WebGlRenderbuffer>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlShader")]
    #[doc = "The `deleteShader()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/deleteShader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlShader`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.deleteShader($1)"]
pub fn delete_shader(&self, shader: Option<&WebGlShader>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlTexture")]
    #[doc = "The `deleteTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/deleteTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlTexture`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.deleteTexture($1)"]
pub fn delete_texture(&self, texture: Option<&WebGlTexture>) { panic!("web-sys intrinsic") }
}
    #[doc = "The `depthFunc()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/depthFunc)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.depthFunc($1)"]
pub fn depth_func(&self, func: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `depthMask()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/depthMask)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.depthMask($1)"]
pub fn depth_mask(&self, flag: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "The `depthRange()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/depthRange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.depthRange($1, $2)"]
pub fn depth_range(&self, z_near: f32, z_far: f32) { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "WebGlProgram", feature = "WebGlShader",))]
    #[doc = "The `detachShader()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/detachShader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`, `WebGlShader`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.detachShader($1, $2)"]
pub fn detach_shader(&self, program: &WebGlProgram, shader: &WebGlShader) { panic!("web-sys intrinsic") }
}
    #[doc = "The `disable()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/disable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.disable($1)"]
pub fn disable(&self, cap: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `disableVertexAttribArray()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/disableVertexAttribArray)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.disableVertexAttribArray($1)"]
pub fn disable_vertex_attrib_array(&self, index: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `drawArrays()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/drawArrays)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.drawArrays($1, $2, $3)"]
pub fn draw_arrays(&self, mode: u32, first: i32, count: i32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `drawElements()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/drawElements)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.drawElements($1, $2, $3, $4)"]
pub fn draw_elements_with_i32(&self, mode: u32, count: i32, type_: u32, offset: i32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `drawElements()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/drawElements)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.drawElements($1, $2, $3, $4)"]
pub fn draw_elements_with_f64(&self, mode: u32, count: i32, type_: u32, offset: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "The `enable()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/enable)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.enable($1)"]
pub fn enable(&self, cap: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `enableVertexAttribArray()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/enableVertexAttribArray)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.enableVertexAttribArray($1)"]
pub fn enable_vertex_attrib_array(&self, index: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `finish()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/finish)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.finish()"]
pub fn finish(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `flush()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/flush)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.flush()"]
pub fn flush(&self) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlRenderbuffer")]
    #[doc = "The `framebufferRenderbuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/framebufferRenderbuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderbuffer`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.framebufferRenderbuffer($1, $2, $3, $4)"]
pub fn framebuffer_renderbuffer(&self, target: u32, attachment: u32, renderbuffertarget: u32, renderbuffer: Option<&WebGlRenderbuffer>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlTexture")]
    #[doc = "The `framebufferTexture2D()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/framebufferTexture2D)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlTexture`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.framebufferTexture2D($1, $2, $3, $4, $5)"]
pub fn framebuffer_texture_2d(&self, target: u32, attachment: u32, textarget: u32, texture: Option<&WebGlTexture>, level: i32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `frontFace()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/frontFace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.frontFace($1)"]
pub fn front_face(&self, mode: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `generateMipmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/generateMipmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.generateMipmap($1)"]
pub fn generate_mipmap(&self, target: u32) { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "WebGlActiveInfo", feature = "WebGlProgram",))]
    #[doc = "The `getActiveAttrib()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getActiveAttrib)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlActiveInfo`, `WebGlProgram`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.getActiveAttrib($1, $2)"]
pub fn get_active_attrib(&self, program: &WebGlProgram, index: u32) -> Option<WebGlActiveInfo> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "WebGlActiveInfo", feature = "WebGlProgram",))]
    #[doc = "The `getActiveUniform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getActiveUniform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlActiveInfo`, `WebGlProgram`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.getActiveUniform($1, $2)"]
pub fn get_active_uniform(&self, program: &WebGlProgram, index: u32) -> Option<WebGlActiveInfo> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlProgram")]
    #[doc = "The `getAttachedShaders()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getAttachedShaders)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.getAttachedShaders($1)"]
pub fn get_attached_shaders(&self, program: &WebGlProgram) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlProgram")]
    #[doc = "The `getAttribLocation()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getAttribLocation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.getAttribLocation($1, $2)"]
pub fn get_attrib_location(&self, program: &WebGlProgram, name: &str) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `getBufferParameter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getBufferParameter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.getBufferParameter($1, $2)"]
pub fn get_buffer_parameter(&self, target: u32, pname: u32) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlContextAttributes")]
    #[doc = "The `getContextAttributes()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getContextAttributes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlContextAttributes`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.getContextAttributes()"]
pub fn get_context_attributes(&self) -> Option<WebGlContextAttributes> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getError()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getError)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.getError()"]
pub fn get_error(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `getExtension()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getExtension)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.getExtension($1)"]
pub fn get_extension(&self, name: &str) -> Result<Option<::js_sys::Object>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getFramebufferAttachmentParameter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getFramebufferAttachmentParameter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.getFramebufferAttachmentParameter($1, $2, $3)"]
pub fn get_framebuffer_attachment_parameter(&self, target: u32, attachment: u32, pname: u32) -> Result<::wasm_bindgen::JsValue, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getParameter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getParameter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.getParameter($1)"]
pub fn get_parameter(&self, pname: u32) -> Result<::wasm_bindgen::JsValue, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlProgram")]
    #[doc = "The `getProgramInfoLog()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getProgramInfoLog)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.getProgramInfoLog($1)"]
pub fn get_program_info_log(&self, program: &WebGlProgram) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlProgram")]
    #[doc = "The `getProgramParameter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getProgramParameter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.getProgramParameter($1, $2)"]
pub fn get_program_parameter(&self, program: &WebGlProgram, pname: u32) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
}
    #[doc = "The `getRenderbufferParameter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getRenderbufferParameter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.getRenderbufferParameter($1, $2)"]
pub fn get_renderbuffer_parameter(&self, target: u32, pname: u32) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlShader")]
    #[doc = "The `getShaderInfoLog()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getShaderInfoLog)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlShader`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.getShaderInfoLog($1)"]
pub fn get_shader_info_log(&self, shader: &WebGlShader) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlShader")]
    #[doc = "The `getShaderParameter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getShaderParameter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlShader`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.getShaderParameter($1, $2)"]
pub fn get_shader_parameter(&self, shader: &WebGlShader, pname: u32) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlShaderPrecisionFormat")]
    #[doc = "The `getShaderPrecisionFormat()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getShaderPrecisionFormat)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlShaderPrecisionFormat`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.getShaderPrecisionFormat($1, $2)"]
pub fn get_shader_precision_format(&self, shadertype: u32, precisiontype: u32) -> Option<WebGlShaderPrecisionFormat> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlShader")]
    #[doc = "The `getShaderSource()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getShaderSource)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlShader`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.getShaderSource($1)"]
pub fn get_shader_source(&self, shader: &WebGlShader) -> Option<::alloc::string::String> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getSupportedExtensions()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getSupportedExtensions)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.getSupportedExtensions()"]
pub fn get_supported_extensions(&self) -> Option<::js_sys::Array> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getTexParameter()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getTexParameter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.getTexParameter($1, $2)"]
pub fn get_tex_parameter(&self, target: u32, pname: u32) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "WebGlProgram", feature = "WebGlUniformLocation",))]
    #[doc = "The `getUniform()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getUniform)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.getUniform($1, $2)"]
pub fn get_uniform(&self, program: &WebGlProgram, location: &WebGlUniformLocation) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "WebGlProgram", feature = "WebGlUniformLocation",))]
    #[doc = "The `getUniformLocation()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getUniformLocation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.getUniformLocation($1, $2)"]
pub fn get_uniform_location(&self, program: &WebGlProgram, name: &str) -> Option<WebGlUniformLocation> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getVertexAttrib()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getVertexAttrib)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.getVertexAttrib($1, $2)"]
pub fn get_vertex_attrib(&self, index: u32, pname: u32) -> Result<::wasm_bindgen::JsValue, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `getVertexAttribOffset()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/getVertexAttribOffset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.getVertexAttribOffset($1, $2)"]
pub fn get_vertex_attrib_offset(&self, index: u32, pname: u32) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "The `hint()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/hint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.hint($1, $2)"]
pub fn hint(&self, target: u32, mode: u32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlBuffer")]
    #[doc = "The `isBuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/isBuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlBuffer`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.isBuffer($1)"]
pub fn is_buffer(&self, buffer: Option<&WebGlBuffer>) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `isContextLost()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/isContextLost)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.isContextLost()"]
pub fn is_context_lost(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `isEnabled()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/isEnabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.isEnabled($1)"]
pub fn is_enabled(&self, cap: u32) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlFramebuffer")]
    #[doc = "The `isFramebuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/isFramebuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlFramebuffer`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.isFramebuffer($1)"]
pub fn is_framebuffer(&self, framebuffer: Option<&WebGlFramebuffer>) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlProgram")]
    #[doc = "The `isProgram()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/isProgram)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.isProgram($1)"]
pub fn is_program(&self, program: Option<&WebGlProgram>) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlRenderbuffer")]
    #[doc = "The `isRenderbuffer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/isRenderbuffer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderbuffer`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.isRenderbuffer($1)"]
pub fn is_renderbuffer(&self, renderbuffer: Option<&WebGlRenderbuffer>) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlShader")]
    #[doc = "The `isShader()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/isShader)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlShader`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.isShader($1)"]
pub fn is_shader(&self, shader: Option<&WebGlShader>) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlTexture")]
    #[doc = "The `isTexture()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/isTexture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlTexture`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.isTexture($1)"]
pub fn is_texture(&self, texture: Option<&WebGlTexture>) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `lineWidth()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/lineWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.lineWidth($1)"]
pub fn line_width(&self, width: f32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlProgram")]
    #[doc = "The `linkProgram()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/linkProgram)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.linkProgram($1)"]
pub fn link_program(&self, program: &WebGlProgram) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `makeXRCompatible()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/makeXRCompatible)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl WebGlRenderingContext {
    #[js::import = "$0.makeXRCompatible()"]
pub fn make_xr_compatible(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `pixelStorei()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/pixelStorei)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.pixelStorei($1, $2)"]
pub fn pixel_storei(&self, pname: u32, param: i32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `polygonOffset()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/polygonOffset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.polygonOffset($1, $2)"]
pub fn polygon_offset(&self, factor: f32, units: f32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `renderbufferStorage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/renderbufferStorage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.renderbufferStorage($1, $2, $3, $4)"]
pub fn renderbuffer_storage(&self, target: u32, internalformat: u32, width: i32, height: i32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `sampleCoverage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/sampleCoverage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.sampleCoverage($1, $2)"]
pub fn sample_coverage(&self, value: f32, invert: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "The `scissor()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/scissor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.scissor($1, $2, $3, $4)"]
pub fn scissor(&self, x: i32, y: i32, width: i32, height: i32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlShader")]
    #[doc = "The `shaderSource()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/shaderSource)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlShader`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.shaderSource($1, $2)"]
pub fn shader_source(&self, shader: &WebGlShader, source: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `stencilFunc()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/stencilFunc)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.stencilFunc($1, $2, $3)"]
pub fn stencil_func(&self, func: u32, ref_: i32, mask: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `stencilFuncSeparate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/stencilFuncSeparate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.stencilFuncSeparate($1, $2, $3, $4)"]
pub fn stencil_func_separate(&self, face: u32, func: u32, ref_: i32, mask: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `stencilMask()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/stencilMask)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.stencilMask($1)"]
pub fn stencil_mask(&self, mask: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `stencilMaskSeparate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/stencilMaskSeparate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.stencilMaskSeparate($1, $2)"]
pub fn stencil_mask_separate(&self, face: u32, mask: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `stencilOp()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/stencilOp)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.stencilOp($1, $2, $3)"]
pub fn stencil_op(&self, fail: u32, zfail: u32, zpass: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `stencilOpSeparate()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/stencilOpSeparate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.stencilOpSeparate($1, $2, $3, $4)"]
pub fn stencil_op_separate(&self, face: u32, fail: u32, zfail: u32, zpass: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `texParameterf()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texParameterf)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.texParameterf($1, $2, $3)"]
pub fn tex_parameterf(&self, target: u32, pname: u32, param: f32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `texParameteri()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/texParameteri)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.texParameteri($1, $2, $3)"]
pub fn tex_parameteri(&self, target: u32, pname: u32, param: i32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniform1f()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform1f)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniform1f($1, $2)"]
pub fn uniform1f(&self, location: Option<&WebGlUniformLocation>, x: f32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniform1i()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform1i)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniform1i($1, $2)"]
pub fn uniform1i(&self, location: Option<&WebGlUniformLocation>, x: i32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniform2f()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform2f)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniform2f($1, $2, $3)"]
pub fn uniform2f(&self, location: Option<&WebGlUniformLocation>, x: f32, y: f32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniform2i()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform2i)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniform2i($1, $2, $3)"]
pub fn uniform2i(&self, location: Option<&WebGlUniformLocation>, x: i32, y: i32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniform3f()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform3f)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniform3f($1, $2, $3, $4)"]
pub fn uniform3f(&self, location: Option<&WebGlUniformLocation>, x: f32, y: f32, z: f32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniform3i()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform3i)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniform3i($1, $2, $3, $4)"]
pub fn uniform3i(&self, location: Option<&WebGlUniformLocation>, x: i32, y: i32, z: i32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniform4f()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform4f)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniform4f($1, $2, $3, $4, $5)"]
pub fn uniform4f(&self, location: Option<&WebGlUniformLocation>, x: f32, y: f32, z: f32, w: f32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlUniformLocation")]
    #[doc = "The `uniform4i()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/uniform4i)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`, `WebGlUniformLocation`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.uniform4i($1, $2, $3, $4, $5)"]
pub fn uniform4i(&self, location: Option<&WebGlUniformLocation>, x: i32, y: i32, z: i32, w: i32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlProgram")]
    #[doc = "The `useProgram()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/useProgram)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.useProgram($1)"]
pub fn use_program(&self, program: Option<&WebGlProgram>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "WebGlProgram")]
    #[doc = "The `validateProgram()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/validateProgram)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlProgram`, `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.validateProgram($1)"]
pub fn validate_program(&self, program: &WebGlProgram) { panic!("web-sys intrinsic") }
}
    #[doc = "The `vertexAttrib1f()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib1f)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.vertexAttrib1f($1, $2)"]
pub fn vertex_attrib1f(&self, indx: u32, x: f32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `vertexAttrib1fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib1fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.vertexAttrib1fv($1, $2)"]
pub fn vertex_attrib1fv_with_f32_array(&self, indx: u32, values: &[f32]) { panic!("web-sys intrinsic") }
}
    #[doc = "The `vertexAttrib1fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib1fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.vertexAttrib1fv($1, $2)"]
pub fn vertex_attrib1fv_with_js_f32_array(&self, indx: u32, values: &::js_sys::Float32Array) { panic!("web-sys intrinsic") }
}
    #[doc = "The `vertexAttrib1fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib1fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.vertexAttrib1fv($1, $2)"]
pub fn vertex_attrib1fv_with_f32_sequence(&self, indx: u32, values: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
}
    #[doc = "The `vertexAttrib2f()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib2f)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.vertexAttrib2f($1, $2, $3)"]
pub fn vertex_attrib2f(&self, indx: u32, x: f32, y: f32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `vertexAttrib2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.vertexAttrib2fv($1, $2)"]
pub fn vertex_attrib2fv_with_f32_array(&self, indx: u32, values: &[f32]) { panic!("web-sys intrinsic") }
}
    #[doc = "The `vertexAttrib2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.vertexAttrib2fv($1, $2)"]
pub fn vertex_attrib2fv_with_js_f32_array(&self, indx: u32, values: &::js_sys::Float32Array) { panic!("web-sys intrinsic") }
}
    #[doc = "The `vertexAttrib2fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib2fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.vertexAttrib2fv($1, $2)"]
pub fn vertex_attrib2fv_with_f32_sequence(&self, indx: u32, values: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
}
    #[doc = "The `vertexAttrib3f()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib3f)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.vertexAttrib3f($1, $2, $3, $4)"]
pub fn vertex_attrib3f(&self, indx: u32, x: f32, y: f32, z: f32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `vertexAttrib3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.vertexAttrib3fv($1, $2)"]
pub fn vertex_attrib3fv_with_f32_array(&self, indx: u32, values: &[f32]) { panic!("web-sys intrinsic") }
}
    #[doc = "The `vertexAttrib3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.vertexAttrib3fv($1, $2)"]
pub fn vertex_attrib3fv_with_js_f32_array(&self, indx: u32, values: &::js_sys::Float32Array) { panic!("web-sys intrinsic") }
}
    #[doc = "The `vertexAttrib3fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib3fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.vertexAttrib3fv($1, $2)"]
pub fn vertex_attrib3fv_with_f32_sequence(&self, indx: u32, values: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
}
    #[doc = "The `vertexAttrib4f()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib4f)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.vertexAttrib4f($1, $2, $3, $4, $5)"]
pub fn vertex_attrib4f(&self, indx: u32, x: f32, y: f32, z: f32, w: f32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `vertexAttrib4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.vertexAttrib4fv($1, $2)"]
pub fn vertex_attrib4fv_with_f32_array(&self, indx: u32, values: &[f32]) { panic!("web-sys intrinsic") }
}
    #[doc = "The `vertexAttrib4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.vertexAttrib4fv($1, $2)"]
pub fn vertex_attrib4fv_with_js_f32_array(&self, indx: u32, values: &::js_sys::Float32Array) { panic!("web-sys intrinsic") }
}
    #[doc = "The `vertexAttrib4fv()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttrib4fv)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.vertexAttrib4fv($1, $2)"]
pub fn vertex_attrib4fv_with_f32_sequence(&self, indx: u32, values: &::wasm_bindgen::JsValue) { panic!("web-sys intrinsic") }
}
    #[doc = "The `vertexAttribPointer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttribPointer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.vertexAttribPointer($1, $2, $3, $4, $5, $6)"]
pub fn vertex_attrib_pointer_with_i32(&self, indx: u32, size: i32, type_: u32, normalized: bool, stride: i32, offset: i32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `vertexAttribPointer()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/vertexAttribPointer)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.vertexAttribPointer($1, $2, $3, $4, $5, $6)"]
pub fn vertex_attrib_pointer_with_f64(&self, indx: u32, size: i32, type_: u32, normalized: bool, stride: i32, offset: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "The `viewport()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/WebGLRenderingContext/viewport)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
impl WebGlRenderingContext {
    #[js::import = "$0.viewport($1, $2, $3, $4)"]
pub fn viewport(&self, x: i32, y: i32, width: i32, height: i32) { panic!("web-sys intrinsic") }
}
impl WebGlRenderingContext {
    #[doc = "The `WebGLRenderingContext.DEPTH_BUFFER_BIT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const DEPTH_BUFFER_BIT: u32 = 256u64 as u32;
    #[doc = "The `WebGLRenderingContext.STENCIL_BUFFER_BIT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const STENCIL_BUFFER_BIT: u32 = 1024u64 as u32;
    #[doc = "The `WebGLRenderingContext.COLOR_BUFFER_BIT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const COLOR_BUFFER_BIT: u32 = 16384u64 as u32;
    #[doc = "The `WebGLRenderingContext.POINTS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const POINTS: u32 = 0u64 as u32;
    #[doc = "The `WebGLRenderingContext.LINES` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const LINES: u32 = 1u64 as u32;
    #[doc = "The `WebGLRenderingContext.LINE_LOOP` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const LINE_LOOP: u32 = 2u64 as u32;
    #[doc = "The `WebGLRenderingContext.LINE_STRIP` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const LINE_STRIP: u32 = 3u64 as u32;
    #[doc = "The `WebGLRenderingContext.TRIANGLES` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TRIANGLES: u32 = 4u64 as u32;
    #[doc = "The `WebGLRenderingContext.TRIANGLE_STRIP` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TRIANGLE_STRIP: u32 = 5u64 as u32;
    #[doc = "The `WebGLRenderingContext.TRIANGLE_FAN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TRIANGLE_FAN: u32 = 6u64 as u32;
    #[doc = "The `WebGLRenderingContext.ZERO` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const ZERO: u32 = 0i64 as u32;
    #[doc = "The `WebGLRenderingContext.ONE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const ONE: u32 = 1u64 as u32;
    #[doc = "The `WebGLRenderingContext.SRC_COLOR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const SRC_COLOR: u32 = 768u64 as u32;
    #[doc = "The `WebGLRenderingContext.ONE_MINUS_SRC_COLOR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const ONE_MINUS_SRC_COLOR: u32 = 769u64 as u32;
    #[doc = "The `WebGLRenderingContext.SRC_ALPHA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const SRC_ALPHA: u32 = 770u64 as u32;
    #[doc = "The `WebGLRenderingContext.ONE_MINUS_SRC_ALPHA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const ONE_MINUS_SRC_ALPHA: u32 = 771u64 as u32;
    #[doc = "The `WebGLRenderingContext.DST_ALPHA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const DST_ALPHA: u32 = 772u64 as u32;
    #[doc = "The `WebGLRenderingContext.ONE_MINUS_DST_ALPHA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const ONE_MINUS_DST_ALPHA: u32 = 773u64 as u32;
    #[doc = "The `WebGLRenderingContext.DST_COLOR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const DST_COLOR: u32 = 774u64 as u32;
    #[doc = "The `WebGLRenderingContext.ONE_MINUS_DST_COLOR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const ONE_MINUS_DST_COLOR: u32 = 775u64 as u32;
    #[doc = "The `WebGLRenderingContext.SRC_ALPHA_SATURATE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const SRC_ALPHA_SATURATE: u32 = 776u64 as u32;
    #[doc = "The `WebGLRenderingContext.FUNC_ADD` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const FUNC_ADD: u32 = 32774u64 as u32;
    #[doc = "The `WebGLRenderingContext.BLEND_EQUATION` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const BLEND_EQUATION: u32 = 32777u64 as u32;
    #[doc = "The `WebGLRenderingContext.BLEND_EQUATION_RGB` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const BLEND_EQUATION_RGB: u32 = 32777u64 as u32;
    #[doc = "The `WebGLRenderingContext.BLEND_EQUATION_ALPHA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const BLEND_EQUATION_ALPHA: u32 = 34877u64 as u32;
    #[doc = "The `WebGLRenderingContext.FUNC_SUBTRACT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const FUNC_SUBTRACT: u32 = 32778u64 as u32;
    #[doc = "The `WebGLRenderingContext.FUNC_REVERSE_SUBTRACT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const FUNC_REVERSE_SUBTRACT: u32 = 32779u64 as u32;
    #[doc = "The `WebGLRenderingContext.BLEND_DST_RGB` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const BLEND_DST_RGB: u32 = 32968u64 as u32;
    #[doc = "The `WebGLRenderingContext.BLEND_SRC_RGB` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const BLEND_SRC_RGB: u32 = 32969u64 as u32;
    #[doc = "The `WebGLRenderingContext.BLEND_DST_ALPHA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const BLEND_DST_ALPHA: u32 = 32970u64 as u32;
    #[doc = "The `WebGLRenderingContext.BLEND_SRC_ALPHA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const BLEND_SRC_ALPHA: u32 = 32971u64 as u32;
    #[doc = "The `WebGLRenderingContext.CONSTANT_COLOR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const CONSTANT_COLOR: u32 = 32769u64 as u32;
    #[doc = "The `WebGLRenderingContext.ONE_MINUS_CONSTANT_COLOR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const ONE_MINUS_CONSTANT_COLOR: u32 = 32770u64 as u32;
    #[doc = "The `WebGLRenderingContext.CONSTANT_ALPHA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const CONSTANT_ALPHA: u32 = 32771u64 as u32;
    #[doc = "The `WebGLRenderingContext.ONE_MINUS_CONSTANT_ALPHA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const ONE_MINUS_CONSTANT_ALPHA: u32 = 32772u64 as u32;
    #[doc = "The `WebGLRenderingContext.BLEND_COLOR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const BLEND_COLOR: u32 = 32773u64 as u32;
    #[doc = "The `WebGLRenderingContext.ARRAY_BUFFER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const ARRAY_BUFFER: u32 = 34962u64 as u32;
    #[doc = "The `WebGLRenderingContext.ELEMENT_ARRAY_BUFFER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const ELEMENT_ARRAY_BUFFER: u32 = 34963u64 as u32;
    #[doc = "The `WebGLRenderingContext.ARRAY_BUFFER_BINDING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const ARRAY_BUFFER_BINDING: u32 = 34964u64 as u32;
    #[doc = "The `WebGLRenderingContext.ELEMENT_ARRAY_BUFFER_BINDING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const ELEMENT_ARRAY_BUFFER_BINDING: u32 = 34965u64 as u32;
    #[doc = "The `WebGLRenderingContext.STREAM_DRAW` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const STREAM_DRAW: u32 = 35040u64 as u32;
    #[doc = "The `WebGLRenderingContext.STATIC_DRAW` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const STATIC_DRAW: u32 = 35044u64 as u32;
    #[doc = "The `WebGLRenderingContext.DYNAMIC_DRAW` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const DYNAMIC_DRAW: u32 = 35048u64 as u32;
    #[doc = "The `WebGLRenderingContext.BUFFER_SIZE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const BUFFER_SIZE: u32 = 34660u64 as u32;
    #[doc = "The `WebGLRenderingContext.BUFFER_USAGE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const BUFFER_USAGE: u32 = 34661u64 as u32;
    #[doc = "The `WebGLRenderingContext.CURRENT_VERTEX_ATTRIB` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const CURRENT_VERTEX_ATTRIB: u32 = 34342u64 as u32;
    #[doc = "The `WebGLRenderingContext.FRONT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const FRONT: u32 = 1028u64 as u32;
    #[doc = "The `WebGLRenderingContext.BACK` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const BACK: u32 = 1029u64 as u32;
    #[doc = "The `WebGLRenderingContext.FRONT_AND_BACK` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const FRONT_AND_BACK: u32 = 1032u64 as u32;
    #[doc = "The `WebGLRenderingContext.CULL_FACE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const CULL_FACE: u32 = 2884u64 as u32;
    #[doc = "The `WebGLRenderingContext.BLEND` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const BLEND: u32 = 3042u64 as u32;
    #[doc = "The `WebGLRenderingContext.DITHER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const DITHER: u32 = 3024u64 as u32;
    #[doc = "The `WebGLRenderingContext.STENCIL_TEST` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const STENCIL_TEST: u32 = 2960u64 as u32;
    #[doc = "The `WebGLRenderingContext.DEPTH_TEST` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const DEPTH_TEST: u32 = 2929u64 as u32;
    #[doc = "The `WebGLRenderingContext.SCISSOR_TEST` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const SCISSOR_TEST: u32 = 3089u64 as u32;
    #[doc = "The `WebGLRenderingContext.POLYGON_OFFSET_FILL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const POLYGON_OFFSET_FILL: u32 = 32823u64 as u32;
    #[doc = "The `WebGLRenderingContext.SAMPLE_ALPHA_TO_COVERAGE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const SAMPLE_ALPHA_TO_COVERAGE: u32 = 32926u64 as u32;
    #[doc = "The `WebGLRenderingContext.SAMPLE_COVERAGE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const SAMPLE_COVERAGE: u32 = 32928u64 as u32;
    #[doc = "The `WebGLRenderingContext.NO_ERROR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const NO_ERROR: u32 = 0i64 as u32;
    #[doc = "The `WebGLRenderingContext.INVALID_ENUM` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const INVALID_ENUM: u32 = 1280u64 as u32;
    #[doc = "The `WebGLRenderingContext.INVALID_VALUE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const INVALID_VALUE: u32 = 1281u64 as u32;
    #[doc = "The `WebGLRenderingContext.INVALID_OPERATION` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const INVALID_OPERATION: u32 = 1282u64 as u32;
    #[doc = "The `WebGLRenderingContext.OUT_OF_MEMORY` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const OUT_OF_MEMORY: u32 = 1285u64 as u32;
    #[doc = "The `WebGLRenderingContext.CW` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const CW: u32 = 2304u64 as u32;
    #[doc = "The `WebGLRenderingContext.CCW` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const CCW: u32 = 2305u64 as u32;
    #[doc = "The `WebGLRenderingContext.LINE_WIDTH` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const LINE_WIDTH: u32 = 2849u64 as u32;
    #[doc = "The `WebGLRenderingContext.ALIASED_POINT_SIZE_RANGE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const ALIASED_POINT_SIZE_RANGE: u32 = 33901u64 as u32;
    #[doc = "The `WebGLRenderingContext.ALIASED_LINE_WIDTH_RANGE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const ALIASED_LINE_WIDTH_RANGE: u32 = 33902u64 as u32;
    #[doc = "The `WebGLRenderingContext.CULL_FACE_MODE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const CULL_FACE_MODE: u32 = 2885u64 as u32;
    #[doc = "The `WebGLRenderingContext.FRONT_FACE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const FRONT_FACE: u32 = 2886u64 as u32;
    #[doc = "The `WebGLRenderingContext.DEPTH_RANGE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const DEPTH_RANGE: u32 = 2928u64 as u32;
    #[doc = "The `WebGLRenderingContext.DEPTH_WRITEMASK` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const DEPTH_WRITEMASK: u32 = 2930u64 as u32;
    #[doc = "The `WebGLRenderingContext.DEPTH_CLEAR_VALUE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const DEPTH_CLEAR_VALUE: u32 = 2931u64 as u32;
    #[doc = "The `WebGLRenderingContext.DEPTH_FUNC` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const DEPTH_FUNC: u32 = 2932u64 as u32;
    #[doc = "The `WebGLRenderingContext.STENCIL_CLEAR_VALUE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const STENCIL_CLEAR_VALUE: u32 = 2961u64 as u32;
    #[doc = "The `WebGLRenderingContext.STENCIL_FUNC` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const STENCIL_FUNC: u32 = 2962u64 as u32;
    #[doc = "The `WebGLRenderingContext.STENCIL_FAIL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const STENCIL_FAIL: u32 = 2964u64 as u32;
    #[doc = "The `WebGLRenderingContext.STENCIL_PASS_DEPTH_FAIL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const STENCIL_PASS_DEPTH_FAIL: u32 = 2965u64 as u32;
    #[doc = "The `WebGLRenderingContext.STENCIL_PASS_DEPTH_PASS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const STENCIL_PASS_DEPTH_PASS: u32 = 2966u64 as u32;
    #[doc = "The `WebGLRenderingContext.STENCIL_REF` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const STENCIL_REF: u32 = 2967u64 as u32;
    #[doc = "The `WebGLRenderingContext.STENCIL_VALUE_MASK` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const STENCIL_VALUE_MASK: u32 = 2963u64 as u32;
    #[doc = "The `WebGLRenderingContext.STENCIL_WRITEMASK` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const STENCIL_WRITEMASK: u32 = 2968u64 as u32;
    #[doc = "The `WebGLRenderingContext.STENCIL_BACK_FUNC` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const STENCIL_BACK_FUNC: u32 = 34816u64 as u32;
    #[doc = "The `WebGLRenderingContext.STENCIL_BACK_FAIL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const STENCIL_BACK_FAIL: u32 = 34817u64 as u32;
    #[doc = "The `WebGLRenderingContext.STENCIL_BACK_PASS_DEPTH_FAIL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const STENCIL_BACK_PASS_DEPTH_FAIL: u32 = 34818u64 as u32;
    #[doc = "The `WebGLRenderingContext.STENCIL_BACK_PASS_DEPTH_PASS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const STENCIL_BACK_PASS_DEPTH_PASS: u32 = 34819u64 as u32;
    #[doc = "The `WebGLRenderingContext.STENCIL_BACK_REF` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const STENCIL_BACK_REF: u32 = 36003u64 as u32;
    #[doc = "The `WebGLRenderingContext.STENCIL_BACK_VALUE_MASK` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const STENCIL_BACK_VALUE_MASK: u32 = 36004u64 as u32;
    #[doc = "The `WebGLRenderingContext.STENCIL_BACK_WRITEMASK` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const STENCIL_BACK_WRITEMASK: u32 = 36005u64 as u32;
    #[doc = "The `WebGLRenderingContext.VIEWPORT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const VIEWPORT: u32 = 2978u64 as u32;
    #[doc = "The `WebGLRenderingContext.SCISSOR_BOX` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const SCISSOR_BOX: u32 = 3088u64 as u32;
    #[doc = "The `WebGLRenderingContext.COLOR_CLEAR_VALUE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const COLOR_CLEAR_VALUE: u32 = 3106u64 as u32;
    #[doc = "The `WebGLRenderingContext.COLOR_WRITEMASK` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const COLOR_WRITEMASK: u32 = 3107u64 as u32;
    #[doc = "The `WebGLRenderingContext.UNPACK_ALIGNMENT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const UNPACK_ALIGNMENT: u32 = 3317u64 as u32;
    #[doc = "The `WebGLRenderingContext.PACK_ALIGNMENT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const PACK_ALIGNMENT: u32 = 3333u64 as u32;
    #[doc = "The `WebGLRenderingContext.MAX_TEXTURE_SIZE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const MAX_TEXTURE_SIZE: u32 = 3379u64 as u32;
    #[doc = "The `WebGLRenderingContext.MAX_VIEWPORT_DIMS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const MAX_VIEWPORT_DIMS: u32 = 3386u64 as u32;
    #[doc = "The `WebGLRenderingContext.SUBPIXEL_BITS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const SUBPIXEL_BITS: u32 = 3408u64 as u32;
    #[doc = "The `WebGLRenderingContext.RED_BITS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const RED_BITS: u32 = 3410u64 as u32;
    #[doc = "The `WebGLRenderingContext.GREEN_BITS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const GREEN_BITS: u32 = 3411u64 as u32;
    #[doc = "The `WebGLRenderingContext.BLUE_BITS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const BLUE_BITS: u32 = 3412u64 as u32;
    #[doc = "The `WebGLRenderingContext.ALPHA_BITS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const ALPHA_BITS: u32 = 3413u64 as u32;
    #[doc = "The `WebGLRenderingContext.DEPTH_BITS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const DEPTH_BITS: u32 = 3414u64 as u32;
    #[doc = "The `WebGLRenderingContext.STENCIL_BITS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const STENCIL_BITS: u32 = 3415u64 as u32;
    #[doc = "The `WebGLRenderingContext.POLYGON_OFFSET_UNITS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const POLYGON_OFFSET_UNITS: u32 = 10752u64 as u32;
    #[doc = "The `WebGLRenderingContext.POLYGON_OFFSET_FACTOR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const POLYGON_OFFSET_FACTOR: u32 = 32824u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE_BINDING_2D` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE_BINDING_2D: u32 = 32873u64 as u32;
    #[doc = "The `WebGLRenderingContext.SAMPLE_BUFFERS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const SAMPLE_BUFFERS: u32 = 32936u64 as u32;
    #[doc = "The `WebGLRenderingContext.SAMPLES` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const SAMPLES: u32 = 32937u64 as u32;
    #[doc = "The `WebGLRenderingContext.SAMPLE_COVERAGE_VALUE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const SAMPLE_COVERAGE_VALUE: u32 = 32938u64 as u32;
    #[doc = "The `WebGLRenderingContext.SAMPLE_COVERAGE_INVERT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const SAMPLE_COVERAGE_INVERT: u32 = 32939u64 as u32;
    #[doc = "The `WebGLRenderingContext.COMPRESSED_TEXTURE_FORMATS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const COMPRESSED_TEXTURE_FORMATS: u32 = 34467u64 as u32;
    #[doc = "The `WebGLRenderingContext.DONT_CARE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const DONT_CARE: u32 = 4352u64 as u32;
    #[doc = "The `WebGLRenderingContext.FASTEST` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const FASTEST: u32 = 4353u64 as u32;
    #[doc = "The `WebGLRenderingContext.NICEST` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const NICEST: u32 = 4354u64 as u32;
    #[doc = "The `WebGLRenderingContext.GENERATE_MIPMAP_HINT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const GENERATE_MIPMAP_HINT: u32 = 33170u64 as u32;
    #[doc = "The `WebGLRenderingContext.BYTE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const BYTE: u32 = 5120u64 as u32;
    #[doc = "The `WebGLRenderingContext.UNSIGNED_BYTE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const UNSIGNED_BYTE: u32 = 5121u64 as u32;
    #[doc = "The `WebGLRenderingContext.SHORT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const SHORT: u32 = 5122u64 as u32;
    #[doc = "The `WebGLRenderingContext.UNSIGNED_SHORT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const UNSIGNED_SHORT: u32 = 5123u64 as u32;
    #[doc = "The `WebGLRenderingContext.INT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const INT: u32 = 5124u64 as u32;
    #[doc = "The `WebGLRenderingContext.UNSIGNED_INT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const UNSIGNED_INT: u32 = 5125u64 as u32;
    #[doc = "The `WebGLRenderingContext.FLOAT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const FLOAT: u32 = 5126u64 as u32;
    #[doc = "The `WebGLRenderingContext.DEPTH_COMPONENT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const DEPTH_COMPONENT: u32 = 6402u64 as u32;
    #[doc = "The `WebGLRenderingContext.ALPHA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const ALPHA: u32 = 6406u64 as u32;
    #[doc = "The `WebGLRenderingContext.RGB` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const RGB: u32 = 6407u64 as u32;
    #[doc = "The `WebGLRenderingContext.RGBA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const RGBA: u32 = 6408u64 as u32;
    #[doc = "The `WebGLRenderingContext.LUMINANCE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const LUMINANCE: u32 = 6409u64 as u32;
    #[doc = "The `WebGLRenderingContext.LUMINANCE_ALPHA` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const LUMINANCE_ALPHA: u32 = 6410u64 as u32;
    #[doc = "The `WebGLRenderingContext.UNSIGNED_SHORT_4_4_4_4` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const UNSIGNED_SHORT_4_4_4_4: u32 = 32819u64 as u32;
    #[doc = "The `WebGLRenderingContext.UNSIGNED_SHORT_5_5_5_1` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const UNSIGNED_SHORT_5_5_5_1: u32 = 32820u64 as u32;
    #[doc = "The `WebGLRenderingContext.UNSIGNED_SHORT_5_6_5` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const UNSIGNED_SHORT_5_6_5: u32 = 33635u64 as u32;
    #[doc = "The `WebGLRenderingContext.FRAGMENT_SHADER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const FRAGMENT_SHADER: u32 = 35632u64 as u32;
    #[doc = "The `WebGLRenderingContext.VERTEX_SHADER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const VERTEX_SHADER: u32 = 35633u64 as u32;
    #[doc = "The `WebGLRenderingContext.MAX_VERTEX_ATTRIBS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const MAX_VERTEX_ATTRIBS: u32 = 34921u64 as u32;
    #[doc = "The `WebGLRenderingContext.MAX_VERTEX_UNIFORM_VECTORS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const MAX_VERTEX_UNIFORM_VECTORS: u32 = 36347u64 as u32;
    #[doc = "The `WebGLRenderingContext.MAX_VARYING_VECTORS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const MAX_VARYING_VECTORS: u32 = 36348u64 as u32;
    #[doc = "The `WebGLRenderingContext.MAX_COMBINED_TEXTURE_IMAGE_UNITS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const MAX_COMBINED_TEXTURE_IMAGE_UNITS: u32 = 35661u64 as u32;
    #[doc = "The `WebGLRenderingContext.MAX_VERTEX_TEXTURE_IMAGE_UNITS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const MAX_VERTEX_TEXTURE_IMAGE_UNITS: u32 = 35660u64 as u32;
    #[doc = "The `WebGLRenderingContext.MAX_TEXTURE_IMAGE_UNITS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const MAX_TEXTURE_IMAGE_UNITS: u32 = 34930u64 as u32;
    #[doc = "The `WebGLRenderingContext.MAX_FRAGMENT_UNIFORM_VECTORS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const MAX_FRAGMENT_UNIFORM_VECTORS: u32 = 36349u64 as u32;
    #[doc = "The `WebGLRenderingContext.SHADER_TYPE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const SHADER_TYPE: u32 = 35663u64 as u32;
    #[doc = "The `WebGLRenderingContext.DELETE_STATUS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const DELETE_STATUS: u32 = 35712u64 as u32;
    #[doc = "The `WebGLRenderingContext.LINK_STATUS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const LINK_STATUS: u32 = 35714u64 as u32;
    #[doc = "The `WebGLRenderingContext.VALIDATE_STATUS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const VALIDATE_STATUS: u32 = 35715u64 as u32;
    #[doc = "The `WebGLRenderingContext.ATTACHED_SHADERS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const ATTACHED_SHADERS: u32 = 35717u64 as u32;
    #[doc = "The `WebGLRenderingContext.ACTIVE_UNIFORMS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const ACTIVE_UNIFORMS: u32 = 35718u64 as u32;
    #[doc = "The `WebGLRenderingContext.ACTIVE_ATTRIBUTES` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const ACTIVE_ATTRIBUTES: u32 = 35721u64 as u32;
    #[doc = "The `WebGLRenderingContext.SHADING_LANGUAGE_VERSION` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const SHADING_LANGUAGE_VERSION: u32 = 35724u64 as u32;
    #[doc = "The `WebGLRenderingContext.CURRENT_PROGRAM` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const CURRENT_PROGRAM: u32 = 35725u64 as u32;
    #[doc = "The `WebGLRenderingContext.NEVER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const NEVER: u32 = 512u64 as u32;
    #[doc = "The `WebGLRenderingContext.LESS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const LESS: u32 = 513u64 as u32;
    #[doc = "The `WebGLRenderingContext.EQUAL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const EQUAL: u32 = 514u64 as u32;
    #[doc = "The `WebGLRenderingContext.LEQUAL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const LEQUAL: u32 = 515u64 as u32;
    #[doc = "The `WebGLRenderingContext.GREATER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const GREATER: u32 = 516u64 as u32;
    #[doc = "The `WebGLRenderingContext.NOTEQUAL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const NOTEQUAL: u32 = 517u64 as u32;
    #[doc = "The `WebGLRenderingContext.GEQUAL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const GEQUAL: u32 = 518u64 as u32;
    #[doc = "The `WebGLRenderingContext.ALWAYS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const ALWAYS: u32 = 519u64 as u32;
    #[doc = "The `WebGLRenderingContext.KEEP` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const KEEP: u32 = 7680u64 as u32;
    #[doc = "The `WebGLRenderingContext.REPLACE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const REPLACE: u32 = 7681u64 as u32;
    #[doc = "The `WebGLRenderingContext.INCR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const INCR: u32 = 7682u64 as u32;
    #[doc = "The `WebGLRenderingContext.DECR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const DECR: u32 = 7683u64 as u32;
    #[doc = "The `WebGLRenderingContext.INVERT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const INVERT: u32 = 5386u64 as u32;
    #[doc = "The `WebGLRenderingContext.INCR_WRAP` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const INCR_WRAP: u32 = 34055u64 as u32;
    #[doc = "The `WebGLRenderingContext.DECR_WRAP` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const DECR_WRAP: u32 = 34056u64 as u32;
    #[doc = "The `WebGLRenderingContext.VENDOR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const VENDOR: u32 = 7936u64 as u32;
    #[doc = "The `WebGLRenderingContext.RENDERER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const RENDERER: u32 = 7937u64 as u32;
    #[doc = "The `WebGLRenderingContext.VERSION` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const VERSION: u32 = 7938u64 as u32;
    #[doc = "The `WebGLRenderingContext.NEAREST` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const NEAREST: u32 = 9728u64 as u32;
    #[doc = "The `WebGLRenderingContext.LINEAR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const LINEAR: u32 = 9729u64 as u32;
    #[doc = "The `WebGLRenderingContext.NEAREST_MIPMAP_NEAREST` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const NEAREST_MIPMAP_NEAREST: u32 = 9984u64 as u32;
    #[doc = "The `WebGLRenderingContext.LINEAR_MIPMAP_NEAREST` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const LINEAR_MIPMAP_NEAREST: u32 = 9985u64 as u32;
    #[doc = "The `WebGLRenderingContext.NEAREST_MIPMAP_LINEAR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const NEAREST_MIPMAP_LINEAR: u32 = 9986u64 as u32;
    #[doc = "The `WebGLRenderingContext.LINEAR_MIPMAP_LINEAR` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const LINEAR_MIPMAP_LINEAR: u32 = 9987u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE_MAG_FILTER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE_MAG_FILTER: u32 = 10240u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE_MIN_FILTER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE_MIN_FILTER: u32 = 10241u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE_WRAP_S` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE_WRAP_S: u32 = 10242u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE_WRAP_T` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE_WRAP_T: u32 = 10243u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE_2D` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE_2D: u32 = 3553u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE: u32 = 5890u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE_CUBE_MAP` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE_CUBE_MAP: u32 = 34067u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE_BINDING_CUBE_MAP` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE_BINDING_CUBE_MAP: u32 = 34068u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE_CUBE_MAP_POSITIVE_X` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE_CUBE_MAP_POSITIVE_X: u32 = 34069u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE_CUBE_MAP_NEGATIVE_X` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE_CUBE_MAP_NEGATIVE_X: u32 = 34070u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE_CUBE_MAP_POSITIVE_Y` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE_CUBE_MAP_POSITIVE_Y: u32 = 34071u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE_CUBE_MAP_NEGATIVE_Y` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Y: u32 = 34072u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE_CUBE_MAP_POSITIVE_Z` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE_CUBE_MAP_POSITIVE_Z: u32 = 34073u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE_CUBE_MAP_NEGATIVE_Z` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE_CUBE_MAP_NEGATIVE_Z: u32 = 34074u64 as u32;
    #[doc = "The `WebGLRenderingContext.MAX_CUBE_MAP_TEXTURE_SIZE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const MAX_CUBE_MAP_TEXTURE_SIZE: u32 = 34076u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE0` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE0: u32 = 33984u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE1` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE1: u32 = 33985u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE2` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE2: u32 = 33986u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE3` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE3: u32 = 33987u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE4` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE4: u32 = 33988u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE5` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE5: u32 = 33989u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE6` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE6: u32 = 33990u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE7` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE7: u32 = 33991u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE8` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE8: u32 = 33992u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE9` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE9: u32 = 33993u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE10` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE10: u32 = 33994u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE11` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE11: u32 = 33995u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE12` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE12: u32 = 33996u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE13` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE13: u32 = 33997u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE14` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE14: u32 = 33998u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE15` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE15: u32 = 33999u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE16` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE16: u32 = 34000u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE17` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE17: u32 = 34001u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE18` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE18: u32 = 34002u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE19` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE19: u32 = 34003u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE20` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE20: u32 = 34004u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE21` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE21: u32 = 34005u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE22` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE22: u32 = 34006u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE23` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE23: u32 = 34007u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE24` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE24: u32 = 34008u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE25` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE25: u32 = 34009u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE26` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE26: u32 = 34010u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE27` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE27: u32 = 34011u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE28` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE28: u32 = 34012u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE29` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE29: u32 = 34013u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE30` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE30: u32 = 34014u64 as u32;
    #[doc = "The `WebGLRenderingContext.TEXTURE31` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const TEXTURE31: u32 = 34015u64 as u32;
    #[doc = "The `WebGLRenderingContext.ACTIVE_TEXTURE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const ACTIVE_TEXTURE: u32 = 34016u64 as u32;
    #[doc = "The `WebGLRenderingContext.REPEAT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const REPEAT: u32 = 10497u64 as u32;
    #[doc = "The `WebGLRenderingContext.CLAMP_TO_EDGE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const CLAMP_TO_EDGE: u32 = 33071u64 as u32;
    #[doc = "The `WebGLRenderingContext.MIRRORED_REPEAT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const MIRRORED_REPEAT: u32 = 33648u64 as u32;
    #[doc = "The `WebGLRenderingContext.FLOAT_VEC2` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const FLOAT_VEC2: u32 = 35664u64 as u32;
    #[doc = "The `WebGLRenderingContext.FLOAT_VEC3` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const FLOAT_VEC3: u32 = 35665u64 as u32;
    #[doc = "The `WebGLRenderingContext.FLOAT_VEC4` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const FLOAT_VEC4: u32 = 35666u64 as u32;
    #[doc = "The `WebGLRenderingContext.INT_VEC2` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const INT_VEC2: u32 = 35667u64 as u32;
    #[doc = "The `WebGLRenderingContext.INT_VEC3` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const INT_VEC3: u32 = 35668u64 as u32;
    #[doc = "The `WebGLRenderingContext.INT_VEC4` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const INT_VEC4: u32 = 35669u64 as u32;
    #[doc = "The `WebGLRenderingContext.BOOL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const BOOL: u32 = 35670u64 as u32;
    #[doc = "The `WebGLRenderingContext.BOOL_VEC2` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const BOOL_VEC2: u32 = 35671u64 as u32;
    #[doc = "The `WebGLRenderingContext.BOOL_VEC3` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const BOOL_VEC3: u32 = 35672u64 as u32;
    #[doc = "The `WebGLRenderingContext.BOOL_VEC4` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const BOOL_VEC4: u32 = 35673u64 as u32;
    #[doc = "The `WebGLRenderingContext.FLOAT_MAT2` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const FLOAT_MAT2: u32 = 35674u64 as u32;
    #[doc = "The `WebGLRenderingContext.FLOAT_MAT3` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const FLOAT_MAT3: u32 = 35675u64 as u32;
    #[doc = "The `WebGLRenderingContext.FLOAT_MAT4` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const FLOAT_MAT4: u32 = 35676u64 as u32;
    #[doc = "The `WebGLRenderingContext.SAMPLER_2D` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const SAMPLER_2D: u32 = 35678u64 as u32;
    #[doc = "The `WebGLRenderingContext.SAMPLER_CUBE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const SAMPLER_CUBE: u32 = 35680u64 as u32;
    #[doc = "The `WebGLRenderingContext.VERTEX_ATTRIB_ARRAY_ENABLED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const VERTEX_ATTRIB_ARRAY_ENABLED: u32 = 34338u64 as u32;
    #[doc = "The `WebGLRenderingContext.VERTEX_ATTRIB_ARRAY_SIZE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const VERTEX_ATTRIB_ARRAY_SIZE: u32 = 34339u64 as u32;
    #[doc = "The `WebGLRenderingContext.VERTEX_ATTRIB_ARRAY_STRIDE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const VERTEX_ATTRIB_ARRAY_STRIDE: u32 = 34340u64 as u32;
    #[doc = "The `WebGLRenderingContext.VERTEX_ATTRIB_ARRAY_TYPE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const VERTEX_ATTRIB_ARRAY_TYPE: u32 = 34341u64 as u32;
    #[doc = "The `WebGLRenderingContext.VERTEX_ATTRIB_ARRAY_NORMALIZED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const VERTEX_ATTRIB_ARRAY_NORMALIZED: u32 = 34922u64 as u32;
    #[doc = "The `WebGLRenderingContext.VERTEX_ATTRIB_ARRAY_POINTER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const VERTEX_ATTRIB_ARRAY_POINTER: u32 = 34373u64 as u32;
    #[doc = "The `WebGLRenderingContext.VERTEX_ATTRIB_ARRAY_BUFFER_BINDING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: u32 = 34975u64 as u32;
    #[doc = "The `WebGLRenderingContext.IMPLEMENTATION_COLOR_READ_TYPE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const IMPLEMENTATION_COLOR_READ_TYPE: u32 = 35738u64 as u32;
    #[doc = "The `WebGLRenderingContext.IMPLEMENTATION_COLOR_READ_FORMAT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const IMPLEMENTATION_COLOR_READ_FORMAT: u32 = 35739u64 as u32;
    #[doc = "The `WebGLRenderingContext.COMPILE_STATUS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const COMPILE_STATUS: u32 = 35713u64 as u32;
    #[doc = "The `WebGLRenderingContext.LOW_FLOAT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const LOW_FLOAT: u32 = 36336u64 as u32;
    #[doc = "The `WebGLRenderingContext.MEDIUM_FLOAT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const MEDIUM_FLOAT: u32 = 36337u64 as u32;
    #[doc = "The `WebGLRenderingContext.HIGH_FLOAT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const HIGH_FLOAT: u32 = 36338u64 as u32;
    #[doc = "The `WebGLRenderingContext.LOW_INT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const LOW_INT: u32 = 36339u64 as u32;
    #[doc = "The `WebGLRenderingContext.MEDIUM_INT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const MEDIUM_INT: u32 = 36340u64 as u32;
    #[doc = "The `WebGLRenderingContext.HIGH_INT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const HIGH_INT: u32 = 36341u64 as u32;
    #[doc = "The `WebGLRenderingContext.FRAMEBUFFER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const FRAMEBUFFER: u32 = 36160u64 as u32;
    #[doc = "The `WebGLRenderingContext.RENDERBUFFER` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const RENDERBUFFER: u32 = 36161u64 as u32;
    #[doc = "The `WebGLRenderingContext.RGBA4` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const RGBA4: u32 = 32854u64 as u32;
    #[doc = "The `WebGLRenderingContext.RGB5_A1` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const RGB5_A1: u32 = 32855u64 as u32;
    #[doc = "The `WebGLRenderingContext.RGB565` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const RGB565: u32 = 36194u64 as u32;
    #[doc = "The `WebGLRenderingContext.DEPTH_COMPONENT16` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const DEPTH_COMPONENT16: u32 = 33189u64 as u32;
    #[doc = "The `WebGLRenderingContext.STENCIL_INDEX8` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const STENCIL_INDEX8: u32 = 36168u64 as u32;
    #[doc = "The `WebGLRenderingContext.DEPTH_STENCIL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const DEPTH_STENCIL: u32 = 34041u64 as u32;
    #[doc = "The `WebGLRenderingContext.RENDERBUFFER_WIDTH` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const RENDERBUFFER_WIDTH: u32 = 36162u64 as u32;
    #[doc = "The `WebGLRenderingContext.RENDERBUFFER_HEIGHT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const RENDERBUFFER_HEIGHT: u32 = 36163u64 as u32;
    #[doc = "The `WebGLRenderingContext.RENDERBUFFER_INTERNAL_FORMAT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const RENDERBUFFER_INTERNAL_FORMAT: u32 = 36164u64 as u32;
    #[doc = "The `WebGLRenderingContext.RENDERBUFFER_RED_SIZE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const RENDERBUFFER_RED_SIZE: u32 = 36176u64 as u32;
    #[doc = "The `WebGLRenderingContext.RENDERBUFFER_GREEN_SIZE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const RENDERBUFFER_GREEN_SIZE: u32 = 36177u64 as u32;
    #[doc = "The `WebGLRenderingContext.RENDERBUFFER_BLUE_SIZE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const RENDERBUFFER_BLUE_SIZE: u32 = 36178u64 as u32;
    #[doc = "The `WebGLRenderingContext.RENDERBUFFER_ALPHA_SIZE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const RENDERBUFFER_ALPHA_SIZE: u32 = 36179u64 as u32;
    #[doc = "The `WebGLRenderingContext.RENDERBUFFER_DEPTH_SIZE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const RENDERBUFFER_DEPTH_SIZE: u32 = 36180u64 as u32;
    #[doc = "The `WebGLRenderingContext.RENDERBUFFER_STENCIL_SIZE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const RENDERBUFFER_STENCIL_SIZE: u32 = 36181u64 as u32;
    #[doc = "The `WebGLRenderingContext.FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: u32 = 36048u64 as u32;
    #[doc = "The `WebGLRenderingContext.FRAMEBUFFER_ATTACHMENT_OBJECT_NAME` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: u32 = 36049u64 as u32;
    #[doc = "The `WebGLRenderingContext.FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: u32 = 36050u64 as u32;
    #[doc = "The `WebGLRenderingContext.FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: u32 = 36051u64 as u32;
    #[doc = "The `WebGLRenderingContext.COLOR_ATTACHMENT0` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const COLOR_ATTACHMENT0: u32 = 36064u64 as u32;
    #[doc = "The `WebGLRenderingContext.DEPTH_ATTACHMENT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const DEPTH_ATTACHMENT: u32 = 36096u64 as u32;
    #[doc = "The `WebGLRenderingContext.STENCIL_ATTACHMENT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const STENCIL_ATTACHMENT: u32 = 36128u64 as u32;
    #[doc = "The `WebGLRenderingContext.DEPTH_STENCIL_ATTACHMENT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const DEPTH_STENCIL_ATTACHMENT: u32 = 33306u64 as u32;
    #[doc = "The `WebGLRenderingContext.NONE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const NONE: u32 = 0i64 as u32;
    #[doc = "The `WebGLRenderingContext.FRAMEBUFFER_COMPLETE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const FRAMEBUFFER_COMPLETE: u32 = 36053u64 as u32;
    #[doc = "The `WebGLRenderingContext.FRAMEBUFFER_INCOMPLETE_ATTACHMENT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const FRAMEBUFFER_INCOMPLETE_ATTACHMENT: u32 = 36054u64 as u32;
    #[doc = "The `WebGLRenderingContext.FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: u32 = 36055u64 as u32;
    #[doc = "The `WebGLRenderingContext.FRAMEBUFFER_INCOMPLETE_DIMENSIONS` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const FRAMEBUFFER_INCOMPLETE_DIMENSIONS: u32 = 36057u64 as u32;
    #[doc = "The `WebGLRenderingContext.FRAMEBUFFER_UNSUPPORTED` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const FRAMEBUFFER_UNSUPPORTED: u32 = 36061u64 as u32;
    #[doc = "The `WebGLRenderingContext.FRAMEBUFFER_BINDING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const FRAMEBUFFER_BINDING: u32 = 36006u64 as u32;
    #[doc = "The `WebGLRenderingContext.RENDERBUFFER_BINDING` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const RENDERBUFFER_BINDING: u32 = 36007u64 as u32;
    #[doc = "The `WebGLRenderingContext.MAX_RENDERBUFFER_SIZE` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const MAX_RENDERBUFFER_SIZE: u32 = 34024u64 as u32;
    #[doc = "The `WebGLRenderingContext.INVALID_FRAMEBUFFER_OPERATION` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const INVALID_FRAMEBUFFER_OPERATION: u32 = 1286u64 as u32;
    #[doc = "The `WebGLRenderingContext.UNPACK_FLIP_Y_WEBGL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const UNPACK_FLIP_Y_WEBGL: u32 = 37440u64 as u32;
    #[doc = "The `WebGLRenderingContext.UNPACK_PREMULTIPLY_ALPHA_WEBGL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const UNPACK_PREMULTIPLY_ALPHA_WEBGL: u32 = 37441u64 as u32;
    #[doc = "The `WebGLRenderingContext.CONTEXT_LOST_WEBGL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const CONTEXT_LOST_WEBGL: u32 = 37442u64 as u32;
    #[doc = "The `WebGLRenderingContext.UNPACK_COLORSPACE_CONVERSION_WEBGL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const UNPACK_COLORSPACE_CONVERSION_WEBGL: u32 = 37443u64 as u32;
    #[doc = "The `WebGLRenderingContext.BROWSER_DEFAULT_WEBGL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `WebGlRenderingContext`*"]
    pub const BROWSER_DEFAULT_WEBGL: u32 = 37444u64 as u32;
}
