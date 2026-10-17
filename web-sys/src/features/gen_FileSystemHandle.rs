#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `FileSystemHandle` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemHandle`*"]
#[js::opaque]
#[derive(Copy)]
pub struct FileSystemHandle(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "FileSystemHandleKind")]
    #[doc = "Getter for the `kind` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle/kind)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemHandle`, `FileSystemHandleKind`*"]
impl FileSystemHandle {
    #[js::import = "$0.kind"]
pub fn kind(&self) -> FileSystemHandleKind { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemHandle`*"]
impl FileSystemHandle {
    #[js::import = "$0.name"]
pub fn name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "The `isSameEntry()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle/isSameEntry)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemHandle`*"]
impl FileSystemHandle {
    #[js::import = "$0.isSameEntry($1)"]
pub fn is_same_entry(&self, other: &FileSystemHandle) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `queryPermission()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle/queryPermission)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemHandle`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl FileSystemHandle {
    #[js::import = "$0.queryPermission()"]
pub fn query_permission(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "FileSystemHandlePermissionDescriptor")]
    #[doc = "The `queryPermission()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle/queryPermission)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemHandle`, `FileSystemHandlePermissionDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl FileSystemHandle {
    #[js::import = "$0.queryPermission($1)"]
pub fn query_permission_with_descriptor(&self, descriptor: &FileSystemHandlePermissionDescriptor) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `requestPermission()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle/requestPermission)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemHandle`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl FileSystemHandle {
    #[js::import = "$0.requestPermission()"]
pub fn request_permission(&self) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "FileSystemHandlePermissionDescriptor")]
    #[doc = "The `requestPermission()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/FileSystemHandle/requestPermission)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `FileSystemHandle`, `FileSystemHandlePermissionDescriptor`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl FileSystemHandle {
    #[js::import = "$0.requestPermission($1)"]
pub fn request_permission_with_descriptor(&self, descriptor: &FileSystemHandlePermissionDescriptor) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
