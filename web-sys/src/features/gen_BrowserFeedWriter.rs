#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BrowserFeedWriter` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BrowserFeedWriter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserFeedWriter`*"]
#[js::opaque]
#[derive(Copy)]
pub struct BrowserFeedWriter(pub ::wasm_bindgen::JsValue);
    #[doc = "The `new BrowserFeedWriter(..)` constructor, creating a new instance of `BrowserFeedWriter`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BrowserFeedWriter/BrowserFeedWriter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserFeedWriter`*"]
impl BrowserFeedWriter {
    #[js::import = "new BrowserFeedWriter()"]
pub fn new() -> Result<BrowserFeedWriter, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BrowserFeedWriter/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserFeedWriter`*"]
impl BrowserFeedWriter {
    #[js::import = "$0.close()"]
pub fn close(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `writeContent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/BrowserFeedWriter/writeContent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BrowserFeedWriter`*"]
impl BrowserFeedWriter {
    #[js::import = "$0.writeContent()"]
pub fn write_content(&self) { panic!("web-sys intrinsic") }
}
