#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Comment` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Comment)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Comment`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Comment(pub ::wasm_bindgen::JsValue);
    #[doc = "The `new Comment(..)` constructor, creating a new instance of `Comment`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Comment/Comment)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Comment`*"]
impl Comment {
    #[js::import = "new Comment()"]
pub fn new() -> Result<Comment, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `new Comment(..)` constructor, creating a new instance of `Comment`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Comment/Comment)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Comment`*"]
impl Comment {
    #[js::import = "new Comment($0)"]
pub fn new_with_data(data: &str) -> Result<Comment, JsValue> { panic!("web-sys intrinsic") }
}
