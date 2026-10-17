#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DocumentTimeline` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentTimeline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentTimeline`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DocumentTimeline(pub ::wasm_bindgen::JsValue);
    #[doc = "The `new DocumentTimeline(..)` constructor, creating a new instance of `DocumentTimeline`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentTimeline/DocumentTimeline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentTimeline`*"]
impl DocumentTimeline {
    #[js::import = "new DocumentTimeline()"]
pub fn new() -> Result<DocumentTimeline, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DocumentTimelineOptions")]
    #[doc = "The `new DocumentTimeline(..)` constructor, creating a new instance of `DocumentTimeline`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DocumentTimeline/DocumentTimeline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DocumentTimeline`, `DocumentTimelineOptions`*"]
impl DocumentTimeline {
    #[js::import = "new DocumentTimeline($0)"]
pub fn new_with_options(options: &DocumentTimelineOptions) -> Result<DocumentTimeline, JsValue> { panic!("web-sys intrinsic") }
}
