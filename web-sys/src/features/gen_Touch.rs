#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Touch` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Touch`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Touch(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `identifier` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/identifier)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Touch`*"]
impl Touch {
    #[js::import = "$0.identifier"]
pub fn identifier(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "EventTarget")]
    #[doc = "Getter for the `target` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/target)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `EventTarget`, `Touch`*"]
impl Touch {
    #[js::import = "$0.target"]
pub fn target(&self) -> Option<EventTarget> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `screenX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/screenX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Touch`*"]
impl Touch {
    #[js::import = "$0.screenX"]
pub fn screen_x(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `screenY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/screenY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Touch`*"]
impl Touch {
    #[js::import = "$0.screenY"]
pub fn screen_y(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `clientX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/clientX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Touch`*"]
impl Touch {
    #[js::import = "$0.clientX"]
pub fn client_x(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `clientY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/clientY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Touch`*"]
impl Touch {
    #[js::import = "$0.clientY"]
pub fn client_y(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `pageX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/pageX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Touch`*"]
impl Touch {
    #[js::import = "$0.pageX"]
pub fn page_x(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `pageY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/pageY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Touch`*"]
impl Touch {
    #[js::import = "$0.pageY"]
pub fn page_y(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `radiusX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/radiusX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Touch`*"]
impl Touch {
    #[js::import = "$0.radiusX"]
pub fn radius_x(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `radiusY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/radiusY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Touch`*"]
impl Touch {
    #[js::import = "$0.radiusY"]
pub fn radius_y(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `rotationAngle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/rotationAngle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Touch`*"]
impl Touch {
    #[js::import = "$0.rotationAngle"]
pub fn rotation_angle(&self) -> f32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `force` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/force)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Touch`*"]
impl Touch {
    #[js::import = "$0.force"]
pub fn force(&self) -> f32 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "TouchInit")]
    #[doc = "The `new Touch(..)` constructor, creating a new instance of `Touch`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Touch/Touch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Touch`, `TouchInit`*"]
impl Touch {
    #[js::import = "new Touch($0)"]
pub fn new(touch_init_dict: &TouchInit) -> Result<Touch, JsValue> { panic!("web-sys intrinsic") }
}
