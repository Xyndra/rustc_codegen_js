#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbMutableFile` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbMutableFile`*"]
    #[deprecated]
#[js::opaque]
#[derive(Copy)]
pub struct IdbMutableFile(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbMutableFile`*"]
    #[deprecated]
impl IdbMutableFile {
    #[js::import = "$0.name"]
pub fn name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `type` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile/type)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbMutableFile`*"]
    #[deprecated]
impl IdbMutableFile {
    #[js::import = "$0.type"]
pub fn type_(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbDatabase")]
    #[doc = "Getter for the `database` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile/database)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbDatabase`, `IdbMutableFile`*"]
    #[deprecated]
impl IdbMutableFile {
    #[js::import = "$0.database"]
pub fn database(&self) -> IdbDatabase { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbMutableFile`*"]
    #[deprecated]
impl IdbMutableFile {
    #[js::import = "$0.onabort"]
pub fn onabort(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbMutableFile`*"]
    #[deprecated]
impl IdbMutableFile {
    #[js::import = "$0.onabort = $1"]
pub fn set_onabort(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbMutableFile`*"]
    #[deprecated]
impl IdbMutableFile {
    #[js::import = "$0.onerror"]
pub fn onerror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbMutableFile`*"]
    #[deprecated]
impl IdbMutableFile {
    #[js::import = "$0.onerror = $1"]
pub fn set_onerror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomRequest")]
    #[doc = "The `getFile()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile/getFile)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomRequest`, `IdbMutableFile`*"]
    #[deprecated]
impl IdbMutableFile {
    #[js::import = "$0.getFile()"]
pub fn get_file(&self) -> Result<DomRequest, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbFileHandle")]
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBMutableFile/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFileHandle`, `IdbMutableFile`*"]
    #[deprecated]
impl IdbMutableFile {
    #[js::import = "$0.open()"]
pub fn open(&self) -> Result<IdbFileHandle, JsValue> { panic!("web-sys intrinsic") }
}
