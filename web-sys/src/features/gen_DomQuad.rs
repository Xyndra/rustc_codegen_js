#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `DomQuad` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuad`*"]
#[js::opaque]
#[derive(Copy)]
pub struct DomQuad(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "DomPoint")]
    #[doc = "Getter for the `p1` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/p1)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuad`*"]
impl DomQuad {
    #[js::import = "$0.p1"]
pub fn p1(&self) -> DomPoint { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomPoint")]
    #[doc = "Getter for the `p2` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/p2)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuad`*"]
impl DomQuad {
    #[js::import = "$0.p2"]
pub fn p2(&self) -> DomPoint { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomPoint")]
    #[doc = "Getter for the `p3` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/p3)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuad`*"]
impl DomQuad {
    #[js::import = "$0.p3"]
pub fn p3(&self) -> DomPoint { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomPoint")]
    #[doc = "Getter for the `p4` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/p4)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPoint`, `DomQuad`*"]
impl DomQuad {
    #[js::import = "$0.p4"]
pub fn p4(&self) -> DomPoint { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomRectReadOnly")]
    #[doc = "Getter for the `bounds` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/bounds)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuad`, `DomRectReadOnly`*"]
impl DomQuad {
    #[js::import = "$0.bounds"]
pub fn bounds(&self) -> DomRectReadOnly { panic!("web-sys intrinsic") }
}
    #[doc = "The `new DomQuad(..)` constructor, creating a new instance of `DomQuad`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/DOMQuad)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuad`*"]
impl DomQuad {
    #[js::import = "new DOMQuad()"]
pub fn new() -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomPointInit")]
    #[doc = "The `new DomQuad(..)` constructor, creating a new instance of `DomQuad`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/DOMQuad)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuad`*"]
impl DomQuad {
    #[js::import = "new DOMQuad($0)"]
pub fn new_with_dom_point_init(p1: &DomPointInit) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomPointInit")]
    #[doc = "The `new DomQuad(..)` constructor, creating a new instance of `DomQuad`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/DOMQuad)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuad`*"]
impl DomQuad {
    #[js::import = "new DOMQuad($0, $1)"]
pub fn new_with_dom_point_init_and_p2(p1: &DomPointInit, p2: &DomPointInit,) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomPointInit")]
    #[doc = "The `new DomQuad(..)` constructor, creating a new instance of `DomQuad`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/DOMQuad)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuad`*"]
impl DomQuad {
    #[js::import = "new DOMQuad($0, $1, $2)"]
pub fn new_with_dom_point_init_and_p2_and_p3(p1: &DomPointInit, p2: &DomPointInit, p3: &DomPointInit,) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomPointInit")]
    #[doc = "The `new DomQuad(..)` constructor, creating a new instance of `DomQuad`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/DOMQuad)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomPointInit`, `DomQuad`*"]
impl DomQuad {
    #[js::import = "new DOMQuad($0, $1, $2, $3)"]
pub fn new_with_dom_point_init_and_p2_and_p3_and_p4(p1: &DomPointInit, p2: &DomPointInit, p3: &DomPointInit, p4: &DomPointInit,) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomRectReadOnly")]
    #[doc = "The `new DomQuad(..)` constructor, creating a new instance of `DomQuad`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/DOMQuad)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuad`, `DomRectReadOnly`*"]
impl DomQuad {
    #[js::import = "new DOMQuad($0)"]
pub fn new_with_rect(rect: &DomRectReadOnly) -> Result<DomQuad, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomRectReadOnly")]
    #[doc = "The `getBounds()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/getBounds)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuad`, `DomRectReadOnly`*"]
impl DomQuad {
    #[js::import = "$0.getBounds()"]
pub fn get_bounds(&self) -> DomRectReadOnly { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "DomQuadJson")]
    #[doc = "The `toJSON()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/toJSON)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DomQuad`, `DomQuadJson`*"]
impl DomQuad {
    #[js::import = "$0.toJSON()"]
pub fn to_json(&self) -> DomQuadJson { panic!("web-sys intrinsic") }
}
