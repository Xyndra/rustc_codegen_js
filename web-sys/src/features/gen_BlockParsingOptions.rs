#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `BlockParsingOptions` dictionary."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlockParsingOptions`*"]
#[js::opaque]
#[derive(Copy)]
pub struct BlockParsingOptions(pub ::wasm_bindgen::JsValue);
    #[doc = "Get the `blockScriptCreated` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlockParsingOptions`*"]
#[js::import = "None"]
pub fn get_block_script_created(this: &BlockParsingOptions) -> Option<bool> { panic!("web-sys intrinsic") }
    #[doc = "Change the `blockScriptCreated` field of this object."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlockParsingOptions`*"]
#[js::import = "None = $1"]
pub fn set_block_script_created(this: &BlockParsingOptions, val: bool) { panic!("web-sys intrinsic") }
impl BlockParsingOptions {
    #[doc = "Construct a new `BlockParsingOptions`."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BlockParsingOptions`*"]
#[js::import = "None"]
pub fn new() -> Self { #[allow(unused_mut)] let mut ret: Self = ::wasm_bindgen::JsCast::unchecked_into(::js_sys::Object::new()) { panic!("web-sys intrinsic") }
        ret
    }
    #[deprecated = "Use `set_block_script_created()` instead."]
#[js::import = "None"]
pub fn block_script_created(&mut self, val: bool) -> &mut Self { self.set_block_script_created(val) { panic!("web-sys intrinsic") }
        self
    }
}
impl Default for BlockParsingOptions {
    fn default() -> Self {
        Self::new()
    }
}
