#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `HtmlDocument` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
#[js::opaque]
#[derive(Copy)]
pub struct HtmlDocument(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `domain` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/domain)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.domain"]
pub fn domain(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `domain` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/domain)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.domain = $1"]
pub fn set_domain(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `cookie` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/cookie)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.cookie"]
pub fn cookie(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `cookie` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/cookie)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.cookie = $1"]
pub fn set_cookie(&self, value: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `designMode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/designMode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.designMode"]
pub fn design_mode(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `designMode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/designMode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.designMode = $1"]
pub fn set_design_mode(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `fgColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/fgColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.fgColor"]
pub fn fg_color(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `fgColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/fgColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.fgColor = $1"]
pub fn set_fg_color(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `linkColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/linkColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.linkColor"]
pub fn link_color(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `linkColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/linkColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.linkColor = $1"]
pub fn set_link_color(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `vlinkColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/vlinkColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.vlinkColor"]
pub fn vlink_color(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `vlinkColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/vlinkColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.vlinkColor = $1"]
pub fn set_vlink_color(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `alinkColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/alinkColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.alinkColor"]
pub fn alink_color(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `alinkColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/alinkColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.alinkColor = $1"]
pub fn set_alink_color(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `bgColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/bgColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.bgColor"]
pub fn bg_color(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `bgColor` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/bgColor)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.bgColor = $1"]
pub fn set_bg_color(&self, value: &str) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlAllCollection")]
    #[doc = "Getter for the `all` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/all)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlAllCollection`, `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.all"]
pub fn all(&self) -> HtmlAllCollection { panic!("web-sys intrinsic") }
}
    #[doc = "The `captureEvents()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/captureEvents)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.captureEvents()"]
pub fn capture_events(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `clear()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/clear)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.clear()"]
pub fn clear(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.close()"]
pub fn close(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `execCommand()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/execCommand)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.execCommand($1)"]
pub fn exec_command(&self, command_id: &str) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `execCommand()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/execCommand)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.execCommand($1, $2)"]
pub fn exec_command_with_show_ui(&self, command_id: &str, show_ui: bool) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `execCommand()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/execCommand)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.execCommand($1, $2, $3)"]
pub fn exec_command_with_show_ui_and_value(&self, command_id: &str, show_ui: bool, value: &str) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.open()"]
pub fn open(&self) -> Result<Document, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.open($1)"]
pub fn open_with_type(&self, type_: &str) -> Result<Document, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.open($1, $2)"]
pub fn open_with_type_and_replace(&self, type_: &str, replace: &str) -> Result<Document, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`, `Window`*"]
impl HtmlDocument {
    #[js::import = "$0.open($1, $2, $3)"]
pub fn open_with_url_and_name_and_features(&self, url: &str, name: &str, features: &str) -> Result<Option<Window>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`, `Window`*"]
impl HtmlDocument {
    #[js::import = "$0.open($1, $2, $3, $4)"]
pub fn open_with_url_and_name_and_features_and_replace(&self, url: &str, name: &str, features: &str, replace: bool) -> Result<Option<Window>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `queryCommandEnabled()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/queryCommandEnabled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.queryCommandEnabled($1)"]
pub fn query_command_enabled(&self, command_id: &str) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `queryCommandIndeterm()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/queryCommandIndeterm)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.queryCommandIndeterm($1)"]
pub fn query_command_indeterm(&self, command_id: &str) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `queryCommandState()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/queryCommandState)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.queryCommandState($1)"]
pub fn query_command_state(&self, command_id: &str) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `queryCommandSupported()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/queryCommandSupported)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.queryCommandSupported($1)"]
pub fn query_command_supported(&self, command_id: &str) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `queryCommandValue()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/queryCommandValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.queryCommandValue($1)"]
pub fn query_command_value(&self, command_id: &str) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `releaseEvents()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/releaseEvents)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.releaseEvents()"]
pub fn release_events(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.write($1)"]
pub fn write(&self, text: &::js_sys::Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.write()"]
pub fn write_0(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.write($1)"]
pub fn write_1(&self, text_1: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.write($1, $2)"]
pub fn write_2(&self, text_1: &str, text_2: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.write($1, $2, $3)"]
pub fn write_3(&self, text_1: &str, text_2: &str, text_3: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.write($1, $2, $3, $4)"]
pub fn write_4(&self, text_1: &str, text_2: &str, text_3: &str, text_4: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.write($1, $2, $3, $4, $5)"]
pub fn write_5(&self, text_1: &str, text_2: &str, text_3: &str, text_4: &str, text_5: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.write($1, $2, $3, $4, $5, $6)"]
pub fn write_6(&self, text_1: &str, text_2: &str, text_3: &str, text_4: &str, text_5: &str, text_6: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `write()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/write)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.write($1, $2, $3, $4, $5, $6, $7)"]
pub fn write_7(&self, text_1: &str, text_2: &str, text_3: &str, text_4: &str, text_5: &str, text_6: &str, text_7: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `writeln()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.writeln($1)"]
pub fn writeln(&self, text: &::js_sys::Array) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `writeln()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.writeln()"]
pub fn writeln_0(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `writeln()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.writeln($1)"]
pub fn writeln_1(&self, text_1: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `writeln()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.writeln($1, $2)"]
pub fn writeln_2(&self, text_1: &str, text_2: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `writeln()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.writeln($1, $2, $3)"]
pub fn writeln_3(&self, text_1: &str, text_2: &str, text_3: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `writeln()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.writeln($1, $2, $3, $4)"]
pub fn writeln_4(&self, text_1: &str, text_2: &str, text_3: &str, text_4: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `writeln()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.writeln($1, $2, $3, $4, $5)"]
pub fn writeln_5(&self, text_1: &str, text_2: &str, text_3: &str, text_4: &str, text_5: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `writeln()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.writeln($1, $2, $3, $4, $5, $6)"]
pub fn writeln_6(&self, text_1: &str, text_2: &str, text_3: &str, text_4: &str, text_5: &str, text_6: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `writeln()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLDocument/writeln)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.writeln($1, $2, $3, $4, $5, $6, $7)"]
pub fn writeln_7(&self, text_1: &str, text_2: &str, text_3: &str, text_4: &str, text_5: &str, text_6: &str, text_7: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlDocument`*"]
impl HtmlDocument {
    #[js::import = "$0.None($1)"]
pub fn get(&self, name: &str) -> Result<::js_sys::Object, JsValue> { panic!("web-sys intrinsic") }
}
