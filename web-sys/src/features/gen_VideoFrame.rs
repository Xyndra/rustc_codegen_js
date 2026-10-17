#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
#[cfg(web_sys_unstable_apis)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `VideoFrame` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
#[js::opaque]
#[derive(Copy)]
pub struct VideoFrame(pub ::wasm_bindgen::JsValue);
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoPixelFormat")]
    #[doc = "Getter for the `format` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/format)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`, `VideoPixelFormat`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "$0.format"]
pub fn format(&self) -> Option<VideoPixelFormat> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `codedWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/codedWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "$0.codedWidth"]
pub fn coded_width(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `codedHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/codedHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "$0.codedHeight"]
pub fn coded_height(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DomRectReadOnly")]
    #[doc = "Getter for the `codedRect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/codedRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectReadOnly`, `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "$0.codedRect"]
pub fn coded_rect(&self) -> Option<DomRectReadOnly> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DomRectReadOnly")]
    #[doc = "Getter for the `visibleRect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/visibleRect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRectReadOnly`, `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "$0.visibleRect"]
pub fn visible_rect(&self) -> Option<DomRectReadOnly> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `displayWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/displayWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "$0.displayWidth"]
pub fn display_width(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `displayHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/displayHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "$0.displayHeight"]
pub fn display_height(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `duration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/duration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "$0.duration"]
pub fn duration(&self) -> Option<f64> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `timestamp` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/timestamp)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "$0.timestamp"]
pub fn timestamp(&self) -> Option<f64> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoColorSpace")]
    #[doc = "Getter for the `colorSpace` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/colorSpace)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoColorSpace`, `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "$0.colorSpace"]
pub fn color_space(&self) -> VideoColorSpace { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HtmlImageElement")]
    #[doc = "The `new VideoFrame(..)` constructor, creating a new instance of `VideoFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/VideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlImageElement`, `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "new VideoFrame($0)"]
pub fn new_with_html_image_element(image: &HtmlImageElement) -> Result<VideoFrame, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SvgImageElement")]
    #[doc = "The `new VideoFrame(..)` constructor, creating a new instance of `VideoFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/VideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgImageElement`, `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "new VideoFrame($0)"]
pub fn new_with_svg_image_element(image: &SvgImageElement) -> Result<VideoFrame, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HtmlCanvasElement")]
    #[doc = "The `new VideoFrame(..)` constructor, creating a new instance of `VideoFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/VideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "new VideoFrame($0)"]
pub fn new_with_html_canvas_element(image: &HtmlCanvasElement) -> Result<VideoFrame, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "HtmlVideoElement")]
    #[doc = "The `new VideoFrame(..)` constructor, creating a new instance of `VideoFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/VideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`, `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "new VideoFrame($0)"]
pub fn new_with_html_video_element(image: &HtmlVideoElement) -> Result<VideoFrame, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ImageBitmap")]
    #[doc = "The `new VideoFrame(..)` constructor, creating a new instance of `VideoFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/VideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`, `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "new VideoFrame($0)"]
pub fn new_with_image_bitmap(image: &ImageBitmap) -> Result<VideoFrame, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "OffscreenCanvas")]
    #[doc = "The `new VideoFrame(..)` constructor, creating a new instance of `VideoFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/VideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvas`, `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "new VideoFrame($0)"]
pub fn new_with_offscreen_canvas(image: &OffscreenCanvas) -> Result<VideoFrame, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `new VideoFrame(..)` constructor, creating a new instance of `VideoFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/VideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "new VideoFrame($0)"]
pub fn new_with_video_frame(image: &VideoFrame) -> Result<VideoFrame, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "HtmlImageElement", feature = "VideoFrameInit",))]
    #[doc = "The `new VideoFrame(..)` constructor, creating a new instance of `VideoFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/VideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlImageElement`, `VideoFrame`, `VideoFrameInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "new VideoFrame($0, $1)"]
pub fn new_with_html_image_element_and_video_frame_init(image: &HtmlImageElement, init: &VideoFrameInit,) -> Result<VideoFrame, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "SvgImageElement", feature = "VideoFrameInit",))]
    #[doc = "The `new VideoFrame(..)` constructor, creating a new instance of `VideoFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/VideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgImageElement`, `VideoFrame`, `VideoFrameInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "new VideoFrame($0, $1)"]
pub fn new_with_svg_image_element_and_video_frame_init(image: &SvgImageElement, init: &VideoFrameInit,) -> Result<VideoFrame, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "HtmlCanvasElement", feature = "VideoFrameInit",))]
    #[doc = "The `new VideoFrame(..)` constructor, creating a new instance of `VideoFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/VideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `VideoFrame`, `VideoFrameInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "new VideoFrame($0, $1)"]
pub fn new_with_html_canvas_element_and_video_frame_init(image: &HtmlCanvasElement, init: &VideoFrameInit,) -> Result<VideoFrame, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "HtmlVideoElement", feature = "VideoFrameInit",))]
    #[doc = "The `new VideoFrame(..)` constructor, creating a new instance of `VideoFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/VideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`, `VideoFrame`, `VideoFrameInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "new VideoFrame($0, $1)"]
pub fn new_with_html_video_element_and_video_frame_init(image: &HtmlVideoElement, init: &VideoFrameInit,) -> Result<VideoFrame, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "ImageBitmap", feature = "VideoFrameInit",))]
    #[doc = "The `new VideoFrame(..)` constructor, creating a new instance of `VideoFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/VideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`, `VideoFrame`, `VideoFrameInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "new VideoFrame($0, $1)"]
pub fn new_with_image_bitmap_and_video_frame_init(image: &ImageBitmap, init: &VideoFrameInit,) -> Result<VideoFrame, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "OffscreenCanvas", feature = "VideoFrameInit",))]
    #[doc = "The `new VideoFrame(..)` constructor, creating a new instance of `VideoFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/VideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvas`, `VideoFrame`, `VideoFrameInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "new VideoFrame($0, $1)"]
pub fn new_with_offscreen_canvas_and_video_frame_init(image: &OffscreenCanvas, init: &VideoFrameInit,) -> Result<VideoFrame, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoFrameInit")]
    #[doc = "The `new VideoFrame(..)` constructor, creating a new instance of `VideoFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/VideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`, `VideoFrameInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "new VideoFrame($0, $1)"]
pub fn new_with_video_frame_and_video_frame_init(image: &VideoFrame, init: &VideoFrameInit,) -> Result<VideoFrame, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoFrameBufferInit")]
    #[doc = "The `new VideoFrame(..)` constructor, creating a new instance of `VideoFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/VideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`, `VideoFrameBufferInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "new VideoFrame($0, $1)"]
pub fn new_with_buffer_source_and_video_frame_buffer_init(data: &::js_sys::Object, init: &VideoFrameBufferInit,) -> Result<VideoFrame, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoFrameBufferInit")]
    #[doc = "The `new VideoFrame(..)` constructor, creating a new instance of `VideoFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/VideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`, `VideoFrameBufferInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "new VideoFrame($0, $1)"]
pub fn new_with_u8_slice_and_video_frame_buffer_init(data: &mut [u8], init: &VideoFrameBufferInit,) -> Result<VideoFrame, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoFrameBufferInit")]
    #[doc = "The `new VideoFrame(..)` constructor, creating a new instance of `VideoFrame`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/VideoFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`, `VideoFrameBufferInit`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "new VideoFrame($0, $1)"]
pub fn new_with_u8_array_and_video_frame_buffer_init(data: &::js_sys::Uint8Array, init: &VideoFrameBufferInit,) -> Result<VideoFrame, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `allocationSize()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/allocationSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "$0.allocationSize()"]
pub fn allocation_size(&self) -> Result<u32, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoFrameCopyToOptions")]
    #[doc = "The `allocationSize()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/allocationSize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`, `VideoFrameCopyToOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "$0.allocationSize($1)"]
pub fn allocation_size_with_options(&self, options: &VideoFrameCopyToOptions) -> Result<u32, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `clone()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/clone)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "$0.clone()"]
pub fn clone(&self) -> Result<VideoFrame, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "$0.close()"]
pub fn close(&self) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `copyTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/copyTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "$0.copyTo($1)"]
pub fn copy_to_with_buffer_source(&self, destination: &::js_sys::Object) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `copyTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/copyTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "$0.copyTo($1)"]
pub fn copy_to_with_u8_slice(&self, destination: &mut [u8]) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `copyTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/copyTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "$0.copyTo($1)"]
pub fn copy_to_with_u8_array(&self, destination: &::js_sys::Uint8Array) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoFrameCopyToOptions")]
    #[doc = "The `copyTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/copyTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`, `VideoFrameCopyToOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "$0.copyTo($1, $2)"]
pub fn copy_to_with_buffer_source_and_options(&self, destination: &::js_sys::Object, options: &VideoFrameCopyToOptions) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoFrameCopyToOptions")]
    #[doc = "The `copyTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/copyTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`, `VideoFrameCopyToOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "$0.copyTo($1, $2)"]
pub fn copy_to_with_u8_slice_and_options(&self, destination: &mut [u8], options: &VideoFrameCopyToOptions) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoFrameCopyToOptions")]
    #[doc = "The `copyTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/VideoFrame/copyTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`, `VideoFrameCopyToOptions`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl VideoFrame {
    #[js::import = "$0.copyTo($1, $2)"]
pub fn copy_to_with_u8_array_and_options(&self, destination: &::js_sys::Uint8Array, options: &VideoFrameCopyToOptions) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
