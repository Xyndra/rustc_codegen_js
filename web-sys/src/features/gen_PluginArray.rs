#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `PluginArray` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PluginArray)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginArray`*"]
#[js::opaque]
#[derive(Copy)]
pub struct PluginArray(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PluginArray/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginArray`*"]
impl PluginArray {
    #[js::import = "$0.length"]
pub fn length(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Plugin")]
    #[doc = "The `item()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PluginArray/item)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Plugin`, `PluginArray`*"]
impl PluginArray {
    #[js::import = "$0.item($1)"]
pub fn item(&self, index: u32) -> Option<Plugin> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Plugin")]
    #[doc = "The `namedItem()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PluginArray/namedItem)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Plugin`, `PluginArray`*"]
impl PluginArray {
    #[js::import = "$0.namedItem($1)"]
pub fn named_item(&self, name: &str) -> Option<Plugin> { panic!("web-sys intrinsic") }
}
    #[doc = "The `refresh()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PluginArray/refresh)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginArray`*"]
impl PluginArray {
    #[js::import = "$0.refresh()"]
pub fn refresh(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `refresh()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PluginArray/refresh)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `PluginArray`*"]
impl PluginArray {
    #[js::import = "$0.refresh($1)"]
pub fn refresh_with_reload_documents(&self, reload_documents: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Plugin")]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Plugin`, `PluginArray`*"]
impl PluginArray {
    #[js::import = "$0.None($1)"]
pub fn get_with_index(&self, index: u32) -> Option<Plugin> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Plugin")]
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Plugin`, `PluginArray`*"]
impl PluginArray {
    #[js::import = "$0.None($1)"]
pub fn get_with_name(&self, name: &str) -> Option<Plugin> { panic!("web-sys intrinsic") }
}
