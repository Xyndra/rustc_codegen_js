#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `IdbKeyRange` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbKeyRange`*"]
#[js::opaque]
#[derive(Copy)]
pub struct IdbKeyRange(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `lower` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/lower)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbKeyRange`*"]
impl IdbKeyRange {
    #[js::import = "$0.lower"]
pub fn lower(&self) -> Result<::wasm_bindgen::JsValue, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `upper` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/upper)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbKeyRange`*"]
impl IdbKeyRange {
    #[js::import = "$0.upper"]
pub fn upper(&self) -> Result<::wasm_bindgen::JsValue, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `lowerOpen` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/lowerOpen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbKeyRange`*"]
impl IdbKeyRange {
    #[js::import = "$0.lowerOpen"]
pub fn lower_open(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `upperOpen` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/upperOpen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbKeyRange`*"]
impl IdbKeyRange {
    #[js::import = "$0.upperOpen"]
pub fn upper_open(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[doc = "The `bound()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/bound_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbKeyRange`*"]
impl IdbKeyRange {
    #[js::import = "IDBKeyRange.bound"]
pub fn bound(lower: &::wasm_bindgen::JsValue, upper: &::wasm_bindgen::JsValue,) -> Result<IdbKeyRange, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `bound()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/bound_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbKeyRange`*"]
impl IdbKeyRange {
    #[js::import = "IDBKeyRange.bound"]
pub fn bound_with_lower_open(lower: &::wasm_bindgen::JsValue, upper: &::wasm_bindgen::JsValue, lower_open: bool,) -> Result<IdbKeyRange, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `bound()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/bound_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbKeyRange`*"]
impl IdbKeyRange {
    #[js::import = "IDBKeyRange.bound"]
pub fn bound_with_lower_open_and_upper_open(lower: &::wasm_bindgen::JsValue, upper: &::wasm_bindgen::JsValue, lower_open: bool, upper_open: bool,) -> Result<IdbKeyRange, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `includes()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/includes)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbKeyRange`*"]
impl IdbKeyRange {
    #[js::import = "$0.includes($1)"]
pub fn includes(&self, key: &::wasm_bindgen::JsValue) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `lowerBound()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/lowerBound_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbKeyRange`*"]
impl IdbKeyRange {
    #[js::import = "IDBKeyRange.lowerBound"]
pub fn lower_bound(lower: &::wasm_bindgen::JsValue) -> Result<IdbKeyRange, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `lowerBound()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/lowerBound_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbKeyRange`*"]
impl IdbKeyRange {
    #[js::import = "IDBKeyRange.lowerBound"]
pub fn lower_bound_with_open(lower: &::wasm_bindgen::JsValue, open: bool,) -> Result<IdbKeyRange, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `only()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/only_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbKeyRange`*"]
impl IdbKeyRange {
    #[js::import = "IDBKeyRange.only"]
pub fn only(value: &::wasm_bindgen::JsValue) -> Result<IdbKeyRange, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `upperBound()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/upperBound_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbKeyRange`*"]
impl IdbKeyRange {
    #[js::import = "IDBKeyRange.upperBound"]
pub fn upper_bound(upper: &::wasm_bindgen::JsValue) -> Result<IdbKeyRange, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `upperBound()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/IDBKeyRange/upperBound_static)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbKeyRange`*"]
impl IdbKeyRange {
    #[js::import = "IDBKeyRange.upperBound"]
pub fn upper_bound_with_open(upper: &::wasm_bindgen::JsValue, open: bool,) -> Result<IdbKeyRange, JsValue> { panic!("web-sys intrinsic") }
}
