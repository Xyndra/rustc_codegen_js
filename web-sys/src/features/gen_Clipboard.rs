#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Clipboard` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clipboard)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Clipboard`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Clipboard(pub ::wasm_bindgen::JsValue);
    #[doc = "The `read()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clipboard/read)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Clipboard`*"]
impl Clipboard {
    #[js::import = "$0.read()"]
pub fn read(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "ClipboardUnsanitizedFormats")]
    #[doc = "The `read()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clipboard/read)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Clipboard`, `ClipboardUnsanitizedFormats`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Clipboard {
    #[js::import = "$0.read($1)"]
pub fn read_with_formats(&self, formats: &ClipboardUnsanitizedFormats) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `readText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clipboard/readText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Clipboard`*"]
impl Clipboard {
    #[js::import = "$0.readText()"]
pub fn read_text(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clipboard/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Clipboard`*"]
impl Clipboard {
    #[js::import = "$0.write($1)"]
pub fn write(&self, data: &::wasm_bindgen::JsValue) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `writeText()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Clipboard/writeText)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Clipboard`*"]
impl Clipboard {
    #[js::import = "$0.writeText($1)"]
pub fn write_text(&self, data: &str) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
