#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `Window` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
#[js::opaque]
#[derive(Copy)]
pub struct Window(pub ::wasm_bindgen::JsValue);
    #[doc = "Getter for the `window` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/window)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.window"]
pub fn window(&self) -> Window { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `self` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/self)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.self"]
pub fn self_(&self) -> Window { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Document")]
    #[doc = "Getter for the `document` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/document)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Document`, `Window`*"]
impl Window {
    #[js::import = "$0.document"]
pub fn document(&self) -> Option<Document> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.name"]
pub fn name(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `name` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/name)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.name = $1"]
pub fn set_name(&self, value: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Location")]
    #[doc = "Getter for the `location` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/location)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Location`, `Window`*"]
impl Window {
    #[js::import = "$0.location"]
pub fn location(&self) -> Location { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "History")]
    #[doc = "Getter for the `history` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/history)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `History`, `Window`*"]
impl Window {
    #[js::import = "$0.history"]
pub fn history(&self) -> Result<History, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CustomElementRegistry")]
    #[doc = "Getter for the `customElements` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/customElements)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CustomElementRegistry`, `Window`*"]
impl Window {
    #[js::import = "$0.customElements"]
pub fn custom_elements(&self) -> CustomElementRegistry { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "BarProp")]
    #[doc = "Getter for the `locationbar` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/locationbar)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BarProp`, `Window`*"]
impl Window {
    #[js::import = "$0.locationbar"]
pub fn locationbar(&self) -> Result<BarProp, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "BarProp")]
    #[doc = "Getter for the `menubar` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/menubar)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BarProp`, `Window`*"]
impl Window {
    #[js::import = "$0.menubar"]
pub fn menubar(&self) -> Result<BarProp, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "BarProp")]
    #[doc = "Getter for the `personalbar` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/personalbar)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BarProp`, `Window`*"]
impl Window {
    #[js::import = "$0.personalbar"]
pub fn personalbar(&self) -> Result<BarProp, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "BarProp")]
    #[doc = "Getter for the `scrollbars` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollbars)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BarProp`, `Window`*"]
impl Window {
    #[js::import = "$0.scrollbars"]
pub fn scrollbars(&self) -> Result<BarProp, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "BarProp")]
    #[doc = "Getter for the `statusbar` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/statusbar)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BarProp`, `Window`*"]
impl Window {
    #[js::import = "$0.statusbar"]
pub fn statusbar(&self) -> Result<BarProp, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "BarProp")]
    #[doc = "Getter for the `toolbar` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/toolbar)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `BarProp`, `Window`*"]
impl Window {
    #[js::import = "$0.toolbar"]
pub fn toolbar(&self) -> Result<BarProp, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `status` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/status)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.status"]
pub fn status(&self) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `status` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/status)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.status = $1"]
pub fn set_status(&self, value: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `closed` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/closed)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.closed"]
pub fn closed(&self) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `event` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/event)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.event"]
pub fn event(&self) -> ::wasm_bindgen::JsValue { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `frames` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/frames)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.frames"]
pub fn frames(&self) -> Result<Window, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `length` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/length)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.length"]
pub fn length(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `top` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/top)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.top"]
pub fn top(&self) -> Result<Option<Window>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `opener` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/opener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.opener"]
pub fn opener(&self) -> Result<::wasm_bindgen::JsValue, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `opener` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/opener)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.opener = $1"]
pub fn set_opener(&self, value: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `parent` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/parent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.parent"]
pub fn parent(&self) -> Result<Option<Window>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Element")]
    #[doc = "Getter for the `frameElement` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/frameElement)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Element`, `Window`*"]
impl Window {
    #[js::import = "$0.frameElement"]
pub fn frame_element(&self) -> Result<Option<Element>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Navigator")]
    #[doc = "Getter for the `navigator` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/navigator)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Navigator`, `Window`*"]
impl Window {
    #[js::import = "$0.navigator"]
pub fn navigator(&self) -> Navigator { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "External")]
    #[doc = "Getter for the `external` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/external)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `External`, `Window`*"]
impl Window {
    #[js::import = "$0.external"]
pub fn external(&self) -> Result<External, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onappinstalled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onappinstalled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onappinstalled"]
pub fn onappinstalled(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onappinstalled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onappinstalled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onappinstalled = $1"]
pub fn set_onappinstalled(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Screen")]
    #[doc = "Getter for the `screen` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/screen)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Screen`, `Window`*"]
impl Window {
    #[js::import = "$0.screen"]
pub fn screen(&self) -> Result<Screen, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "VisualViewport")]
    #[doc = "Getter for the `visualViewport` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/visualViewport)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VisualViewport`, `Window`*"]
impl Window {
    #[js::import = "$0.visualViewport"]
pub fn visual_viewport(&self) -> Option<VisualViewport> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `innerWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/innerWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.innerWidth"]
pub fn inner_width(&self) -> Result<::wasm_bindgen::JsValue, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `innerWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/innerWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.innerWidth = $1"]
pub fn set_inner_width(&self, value: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `innerHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/innerHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.innerHeight"]
pub fn inner_height(&self) -> Result<::wasm_bindgen::JsValue, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `innerHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/innerHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.innerHeight = $1"]
pub fn set_inner_height(&self, value: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `scrollX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.scrollX"]
pub fn scroll_x(&self) -> Result<f64, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `pageXOffset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/pageXOffset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.pageXOffset"]
pub fn page_x_offset(&self) -> Result<f64, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `scrollY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.scrollY"]
pub fn scroll_y(&self) -> Result<f64, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `pageYOffset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/pageYOffset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.pageYOffset"]
pub fn page_y_offset(&self) -> Result<f64, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `screenX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/screenX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.screenX"]
pub fn screen_x(&self) -> Result<::wasm_bindgen::JsValue, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `screenX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/screenX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.screenX = $1"]
pub fn set_screen_x(&self, value: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `screenY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/screenY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.screenY"]
pub fn screen_y(&self) -> Result<::wasm_bindgen::JsValue, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `screenY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/screenY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.screenY = $1"]
pub fn set_screen_y(&self, value: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `outerWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/outerWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.outerWidth"]
pub fn outer_width(&self) -> Result<::wasm_bindgen::JsValue, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `outerWidth` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/outerWidth)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.outerWidth = $1"]
pub fn set_outer_width(&self, value: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `outerHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/outerHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.outerHeight"]
pub fn outer_height(&self) -> Result<::wasm_bindgen::JsValue, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `outerHeight` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/outerHeight)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.outerHeight = $1"]
pub fn set_outer_height(&self, value: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `devicePixelRatio` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/devicePixelRatio)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.devicePixelRatio"]
pub fn device_pixel_ratio(&self) -> f64 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `orientation` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/orientation)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.orientation"]
pub fn orientation(&self) -> i16 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onorientationchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onorientationchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onorientationchange"]
pub fn onorientationchange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onorientationchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onorientationchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onorientationchange = $1"]
pub fn set_onorientationchange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onvrdisplayconnect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplayconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onvrdisplayconnect"]
pub fn onvrdisplayconnect(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onvrdisplayconnect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplayconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onvrdisplayconnect = $1"]
pub fn set_onvrdisplayconnect(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onvrdisplaydisconnect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplaydisconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onvrdisplaydisconnect"]
pub fn onvrdisplaydisconnect(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onvrdisplaydisconnect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplaydisconnect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onvrdisplaydisconnect = $1"]
pub fn set_onvrdisplaydisconnect(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onvrdisplayactivate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplayactivate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onvrdisplayactivate"]
pub fn onvrdisplayactivate(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onvrdisplayactivate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplayactivate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onvrdisplayactivate = $1"]
pub fn set_onvrdisplayactivate(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onvrdisplaydeactivate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplaydeactivate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onvrdisplaydeactivate"]
pub fn onvrdisplaydeactivate(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onvrdisplaydeactivate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplaydeactivate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onvrdisplaydeactivate = $1"]
pub fn set_onvrdisplaydeactivate(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onvrdisplaypresentchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplaypresentchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onvrdisplaypresentchange"]
pub fn onvrdisplaypresentchange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onvrdisplaypresentchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvrdisplaypresentchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onvrdisplaypresentchange = $1"]
pub fn set_onvrdisplaypresentchange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Worklet")]
    #[doc = "Getter for the `paintWorklet` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/paintWorklet)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`, `Worklet`*"]
impl Window {
    #[js::import = "$0.paintWorklet"]
pub fn paint_worklet(&self) -> Result<Worklet, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Crypto")]
    #[doc = "Getter for the `crypto` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/crypto)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Crypto`, `Window`*"]
impl Window {
    #[js::import = "$0.crypto"]
pub fn crypto(&self) -> Result<Crypto, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onabort"]
pub fn onabort(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onabort` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onabort)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onabort = $1"]
pub fn set_onabort(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onblur` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onblur)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onblur"]
pub fn onblur(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onblur` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onblur)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onblur = $1"]
pub fn set_onblur(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onfocus` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onfocus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onfocus"]
pub fn onfocus(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onfocus` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onfocus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onfocus = $1"]
pub fn set_onfocus(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onauxclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onauxclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onauxclick"]
pub fn onauxclick(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onauxclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onauxclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onauxclick = $1"]
pub fn set_onauxclick(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onbeforetoggle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onbeforetoggle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onbeforetoggle"]
pub fn onbeforetoggle(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onbeforetoggle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onbeforetoggle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onbeforetoggle = $1"]
pub fn set_onbeforetoggle(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `oncanplay` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oncanplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.oncanplay"]
pub fn oncanplay(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `oncanplay` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oncanplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.oncanplay = $1"]
pub fn set_oncanplay(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `oncanplaythrough` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oncanplaythrough)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.oncanplaythrough"]
pub fn oncanplaythrough(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `oncanplaythrough` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oncanplaythrough)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.oncanplaythrough = $1"]
pub fn set_oncanplaythrough(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onchange"]
pub fn onchange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onchange = $1"]
pub fn set_onchange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onclick"]
pub fn onclick(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onclick = $1"]
pub fn set_onclick(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onclose"]
pub fn onclose(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onclose` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onclose)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onclose = $1"]
pub fn set_onclose(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `oncontextmenu` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oncontextmenu)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.oncontextmenu"]
pub fn oncontextmenu(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `oncontextmenu` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oncontextmenu)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.oncontextmenu = $1"]
pub fn set_oncontextmenu(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ondblclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondblclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ondblclick"]
pub fn ondblclick(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondblclick` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondblclick)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ondblclick = $1"]
pub fn set_ondblclick(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ondrag` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondrag)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ondrag"]
pub fn ondrag(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondrag` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondrag)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ondrag = $1"]
pub fn set_ondrag(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ondragend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ondragend"]
pub fn ondragend(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondragend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ondragend = $1"]
pub fn set_ondragend(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ondragenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ondragenter"]
pub fn ondragenter(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondragenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ondragenter = $1"]
pub fn set_ondragenter(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ondragexit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragexit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ondragexit"]
pub fn ondragexit(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondragexit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragexit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ondragexit = $1"]
pub fn set_ondragexit(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ondragleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ondragleave"]
pub fn ondragleave(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondragleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ondragleave = $1"]
pub fn set_ondragleave(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ondragover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ondragover"]
pub fn ondragover(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondragover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ondragover = $1"]
pub fn set_ondragover(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ondragstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ondragstart"]
pub fn ondragstart(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondragstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondragstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ondragstart = $1"]
pub fn set_ondragstart(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ondrop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondrop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ondrop"]
pub fn ondrop(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondrop` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondrop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ondrop = $1"]
pub fn set_ondrop(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ondurationchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondurationchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ondurationchange"]
pub fn ondurationchange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ondurationchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondurationchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ondurationchange = $1"]
pub fn set_ondurationchange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onemptied` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onemptied)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onemptied"]
pub fn onemptied(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onemptied` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onemptied)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onemptied = $1"]
pub fn set_onemptied(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onended"]
pub fn onended(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onended` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onended)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onended = $1"]
pub fn set_onended(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `oninput` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oninput)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.oninput"]
pub fn oninput(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `oninput` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oninput)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.oninput = $1"]
pub fn set_oninput(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onbeforeinput` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onbeforeinput)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onbeforeinput"]
pub fn onbeforeinput(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onbeforeinput` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onbeforeinput)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onbeforeinput = $1"]
pub fn set_onbeforeinput(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `oninvalid` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oninvalid)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.oninvalid"]
pub fn oninvalid(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `oninvalid` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/oninvalid)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.oninvalid = $1"]
pub fn set_oninvalid(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onkeydown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onkeydown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onkeydown"]
pub fn onkeydown(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onkeydown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onkeydown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onkeydown = $1"]
pub fn set_onkeydown(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onkeypress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onkeypress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onkeypress"]
pub fn onkeypress(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onkeypress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onkeypress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onkeypress = $1"]
pub fn set_onkeypress(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onkeyup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onkeyup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onkeyup"]
pub fn onkeyup(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onkeyup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onkeyup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onkeyup = $1"]
pub fn set_onkeyup(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onload"]
pub fn onload(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onload = $1"]
pub fn set_onload(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onloadeddata` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onloadeddata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onloadeddata"]
pub fn onloadeddata(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onloadeddata` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onloadeddata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onloadeddata = $1"]
pub fn set_onloadeddata(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onloadedmetadata` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onloadedmetadata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onloadedmetadata"]
pub fn onloadedmetadata(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onloadedmetadata` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onloadedmetadata)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onloadedmetadata = $1"]
pub fn set_onloadedmetadata(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onloadend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onloadend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onloadend"]
pub fn onloadend(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onloadend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onloadend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onloadend = $1"]
pub fn set_onloadend(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onloadstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onloadstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onloadstart"]
pub fn onloadstart(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onloadstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onloadstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onloadstart = $1"]
pub fn set_onloadstart(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmousedown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmousedown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onmousedown"]
pub fn onmousedown(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmousedown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmousedown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onmousedown = $1"]
pub fn set_onmousedown(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmouseenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onmouseenter"]
pub fn onmouseenter(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmouseenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onmouseenter = $1"]
pub fn set_onmouseenter(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmouseleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onmouseleave"]
pub fn onmouseleave(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmouseleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onmouseleave = $1"]
pub fn set_onmouseleave(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmousemove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmousemove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onmousemove"]
pub fn onmousemove(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmousemove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmousemove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onmousemove = $1"]
pub fn set_onmousemove(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmouseout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onmouseout"]
pub fn onmouseout(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmouseout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onmouseout = $1"]
pub fn set_onmouseout(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmouseover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onmouseover"]
pub fn onmouseover(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmouseover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onmouseover = $1"]
pub fn set_onmouseover(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmouseup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onmouseup"]
pub fn onmouseup(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmouseup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmouseup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onmouseup = $1"]
pub fn set_onmouseup(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onwheel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwheel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onwheel"]
pub fn onwheel(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onwheel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwheel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onwheel = $1"]
pub fn set_onwheel(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpause` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpause)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onpause"]
pub fn onpause(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpause` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpause)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onpause = $1"]
pub fn set_onpause(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onplay` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onplay"]
pub fn onplay(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onplay` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onplay)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onplay = $1"]
pub fn set_onplay(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onplaying` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onplaying)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onplaying"]
pub fn onplaying(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onplaying` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onplaying)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onplaying = $1"]
pub fn set_onplaying(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onprogress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onprogress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onprogress"]
pub fn onprogress(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onprogress` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onprogress)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onprogress = $1"]
pub fn set_onprogress(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onratechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onratechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onratechange"]
pub fn onratechange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onratechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onratechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onratechange = $1"]
pub fn set_onratechange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onreset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onreset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onreset"]
pub fn onreset(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onreset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onreset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onreset = $1"]
pub fn set_onreset(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onresize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onresize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onresize"]
pub fn onresize(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onresize` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onresize)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onresize = $1"]
pub fn set_onresize(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onscroll` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onscroll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onscroll"]
pub fn onscroll(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onscroll` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onscroll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onscroll = $1"]
pub fn set_onscroll(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onseeked` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onseeked)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onseeked"]
pub fn onseeked(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onseeked` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onseeked)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onseeked = $1"]
pub fn set_onseeked(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onseeking` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onseeking)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onseeking"]
pub fn onseeking(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onseeking` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onseeking)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onseeking = $1"]
pub fn set_onseeking(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onselect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onselect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onselect"]
pub fn onselect(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onselect` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onselect)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onselect = $1"]
pub fn set_onselect(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onshow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onshow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onshow"]
pub fn onshow(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onshow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onshow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onshow = $1"]
pub fn set_onshow(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onstalled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onstalled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onstalled"]
pub fn onstalled(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onstalled` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onstalled)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onstalled = $1"]
pub fn set_onstalled(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onsubmit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onsubmit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onsubmit"]
pub fn onsubmit(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onsubmit` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onsubmit)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onsubmit = $1"]
pub fn set_onsubmit(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onsuspend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onsuspend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onsuspend"]
pub fn onsuspend(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onsuspend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onsuspend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onsuspend = $1"]
pub fn set_onsuspend(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ontimeupdate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontimeupdate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ontimeupdate"]
pub fn ontimeupdate(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ontimeupdate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontimeupdate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ontimeupdate = $1"]
pub fn set_ontimeupdate(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onvolumechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvolumechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onvolumechange"]
pub fn onvolumechange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onvolumechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onvolumechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onvolumechange = $1"]
pub fn set_onvolumechange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onwaiting` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwaiting)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onwaiting"]
pub fn onwaiting(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onwaiting` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwaiting)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onwaiting = $1"]
pub fn set_onwaiting(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onselectstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onselectstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onselectstart"]
pub fn onselectstart(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onselectstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onselectstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onselectstart = $1"]
pub fn set_onselectstart(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ontoggle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontoggle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ontoggle"]
pub fn ontoggle(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ontoggle` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontoggle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ontoggle = $1"]
pub fn set_ontoggle(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpointercancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointercancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onpointercancel"]
pub fn onpointercancel(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpointercancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointercancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onpointercancel = $1"]
pub fn set_onpointercancel(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpointerdown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerdown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onpointerdown"]
pub fn onpointerdown(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpointerdown` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerdown)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onpointerdown = $1"]
pub fn set_onpointerdown(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpointerup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onpointerup"]
pub fn onpointerup(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpointerup` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerup)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onpointerup = $1"]
pub fn set_onpointerup(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpointermove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointermove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onpointermove"]
pub fn onpointermove(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpointermove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointermove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onpointermove = $1"]
pub fn set_onpointermove(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpointerout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onpointerout"]
pub fn onpointerout(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpointerout` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onpointerout = $1"]
pub fn set_onpointerout(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpointerover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onpointerover"]
pub fn onpointerover(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpointerover` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerover)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onpointerover = $1"]
pub fn set_onpointerover(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpointerenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onpointerenter"]
pub fn onpointerenter(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpointerenter` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerenter)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onpointerenter = $1"]
pub fn set_onpointerenter(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpointerleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onpointerleave"]
pub fn onpointerleave(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpointerleave` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpointerleave)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onpointerleave = $1"]
pub fn set_onpointerleave(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ongotpointercapture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ongotpointercapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ongotpointercapture"]
pub fn ongotpointercapture(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ongotpointercapture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ongotpointercapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ongotpointercapture = $1"]
pub fn set_ongotpointercapture(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onlostpointercapture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onlostpointercapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onlostpointercapture"]
pub fn onlostpointercapture(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onlostpointercapture` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onlostpointercapture)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onlostpointercapture = $1"]
pub fn set_onlostpointercapture(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onanimationcancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onanimationcancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onanimationcancel"]
pub fn onanimationcancel(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onanimationcancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onanimationcancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onanimationcancel = $1"]
pub fn set_onanimationcancel(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onanimationend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onanimationend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onanimationend"]
pub fn onanimationend(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onanimationend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onanimationend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onanimationend = $1"]
pub fn set_onanimationend(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onanimationiteration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onanimationiteration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onanimationiteration"]
pub fn onanimationiteration(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onanimationiteration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onanimationiteration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onanimationiteration = $1"]
pub fn set_onanimationiteration(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onanimationstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onanimationstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onanimationstart"]
pub fn onanimationstart(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onanimationstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onanimationstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onanimationstart = $1"]
pub fn set_onanimationstart(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ontransitioncancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontransitioncancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ontransitioncancel"]
pub fn ontransitioncancel(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ontransitioncancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontransitioncancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ontransitioncancel = $1"]
pub fn set_ontransitioncancel(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ontransitionend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontransitionend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ontransitionend"]
pub fn ontransitionend(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ontransitionend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontransitionend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ontransitionend = $1"]
pub fn set_ontransitionend(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ontransitionrun` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontransitionrun)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ontransitionrun"]
pub fn ontransitionrun(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ontransitionrun` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontransitionrun)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ontransitionrun = $1"]
pub fn set_ontransitionrun(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ontransitionstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontransitionstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ontransitionstart"]
pub fn ontransitionstart(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ontransitionstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontransitionstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ontransitionstart = $1"]
pub fn set_ontransitionstart(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onwebkitanimationend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwebkitanimationend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onwebkitanimationend"]
pub fn onwebkitanimationend(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onwebkitanimationend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwebkitanimationend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onwebkitanimationend = $1"]
pub fn set_onwebkitanimationend(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onwebkitanimationiteration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwebkitanimationiteration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onwebkitanimationiteration"]
pub fn onwebkitanimationiteration(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onwebkitanimationiteration` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwebkitanimationiteration)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onwebkitanimationiteration = $1"]
pub fn set_onwebkitanimationiteration(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onwebkitanimationstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwebkitanimationstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onwebkitanimationstart"]
pub fn onwebkitanimationstart(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onwebkitanimationstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwebkitanimationstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onwebkitanimationstart = $1"]
pub fn set_onwebkitanimationstart(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onwebkittransitionend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwebkittransitionend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onwebkittransitionend"]
pub fn onwebkittransitionend(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onwebkittransitionend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onwebkittransitionend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onwebkittransitionend = $1"]
pub fn set_onwebkittransitionend(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "U2f")]
    #[doc = "Getter for the `u2f` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/u2f)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `U2f`, `Window`*"]
impl Window {
    #[js::import = "$0.u2f"]
pub fn u2f(&self) -> Result<U2f, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onerror"]
pub fn onerror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onerror = $1"]
pub fn set_onerror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SpeechSynthesis")]
    #[doc = "Getter for the `speechSynthesis` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/speechSynthesis)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SpeechSynthesis`, `Window`*"]
impl Window {
    #[js::import = "$0.speechSynthesis"]
pub fn speech_synthesis(&self) -> Result<SpeechSynthesis, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ontouchstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontouchstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ontouchstart"]
pub fn ontouchstart(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ontouchstart` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontouchstart)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ontouchstart = $1"]
pub fn set_ontouchstart(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ontouchend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontouchend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ontouchend"]
pub fn ontouchend(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ontouchend` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontouchend)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ontouchend = $1"]
pub fn set_ontouchend(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ontouchmove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontouchmove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ontouchmove"]
pub fn ontouchmove(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ontouchmove` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontouchmove)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ontouchmove = $1"]
pub fn set_ontouchmove(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ontouchcancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontouchcancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ontouchcancel"]
pub fn ontouchcancel(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ontouchcancel` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ontouchcancel)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ontouchcancel = $1"]
pub fn set_ontouchcancel(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onafterprint` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onafterprint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onafterprint"]
pub fn onafterprint(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onafterprint` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onafterprint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onafterprint = $1"]
pub fn set_onafterprint(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onbeforeprint` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onbeforeprint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onbeforeprint"]
pub fn onbeforeprint(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onbeforeprint` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onbeforeprint)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onbeforeprint = $1"]
pub fn set_onbeforeprint(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onbeforeunload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onbeforeunload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onbeforeunload"]
pub fn onbeforeunload(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onbeforeunload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onbeforeunload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onbeforeunload = $1"]
pub fn set_onbeforeunload(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onhashchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onhashchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onhashchange"]
pub fn onhashchange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onhashchange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onhashchange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onhashchange = $1"]
pub fn set_onhashchange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onlanguagechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onlanguagechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onlanguagechange"]
pub fn onlanguagechange(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onlanguagechange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onlanguagechange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onlanguagechange = $1"]
pub fn set_onlanguagechange(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onmessage"]
pub fn onmessage(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmessage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onmessage = $1"]
pub fn set_onmessage(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onmessageerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmessageerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onmessageerror"]
pub fn onmessageerror(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onmessageerror` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onmessageerror)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onmessageerror = $1"]
pub fn set_onmessageerror(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onoffline` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onoffline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onoffline"]
pub fn onoffline(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onoffline` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onoffline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onoffline = $1"]
pub fn set_onoffline(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `ononline` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ononline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ononline"]
pub fn ononline(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `ononline` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ononline)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.ononline = $1"]
pub fn set_ononline(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpagehide` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpagehide)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onpagehide"]
pub fn onpagehide(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpagehide` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpagehide)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onpagehide = $1"]
pub fn set_onpagehide(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpageshow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpageshow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onpageshow"]
pub fn onpageshow(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpageshow` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpageshow)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onpageshow = $1"]
pub fn set_onpageshow(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onpopstate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpopstate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onpopstate"]
pub fn onpopstate(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onpopstate` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onpopstate)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onpopstate = $1"]
pub fn set_onpopstate(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onstorage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onstorage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onstorage"]
pub fn onstorage(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onstorage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onstorage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onstorage = $1"]
pub fn set_onstorage(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `onunload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onunload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onunload"]
pub fn onunload(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[doc = "Setter for the `onunload` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/onunload)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.onunload = $1"]
pub fn set_onunload(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `ongamepadconnected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ongamepadconnected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Window {
    #[js::import = "$0.ongamepadconnected"]
pub fn ongamepadconnected(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Setter for the `ongamepadconnected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ongamepadconnected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Window {
    #[js::import = "$0.ongamepadconnected = $1"]
pub fn set_ongamepadconnected(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Getter for the `ongamepaddisconnected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ongamepaddisconnected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Window {
    #[js::import = "$0.ongamepaddisconnected"]
pub fn ongamepaddisconnected(&self) -> Option<::js_sys::Function> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "Setter for the `ongamepaddisconnected` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/ongamepaddisconnected)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Window {
    #[js::import = "$0.ongamepaddisconnected = $1"]
pub fn set_ongamepaddisconnected(&self, value: Option<&::js_sys::Function>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Storage")]
    #[doc = "Getter for the `localStorage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Storage`, `Window`*"]
impl Window {
    #[js::import = "$0.localStorage"]
pub fn local_storage(&self) -> Result<Option<Storage>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdbFactory")]
    #[doc = "Getter for the `indexedDB` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/indexedDB)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdbFactory`, `Window`*"]
impl Window {
    #[js::import = "$0.indexedDB"]
pub fn indexed_db(&self) -> Result<Option<IdbFactory>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Performance")]
    #[doc = "Getter for the `performance` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/performance)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Performance`, `Window`*"]
impl Window {
    #[js::import = "$0.performance"]
pub fn performance(&self) -> Option<Performance> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `origin` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/origin)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.origin"]
pub fn origin(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `isSecureContext` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/isSecureContext)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.isSecureContext"]
pub fn is_secure_context(&self) -> bool { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "CacheStorage")]
    #[doc = "Getter for the `caches` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/caches)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CacheStorage`, `Window`*"]
impl Window {
    #[js::import = "$0.caches"]
pub fn caches(&self) -> Result<CacheStorage, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "Scheduler")]
    #[doc = "Getter for the `scheduler` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scheduler)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Scheduler`, `Window`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Window {
    #[js::import = "$0.scheduler"]
pub fn scheduler(&self) -> Scheduler { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Storage")]
    #[doc = "Getter for the `sessionStorage` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/sessionStorage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Storage`, `Window`*"]
impl Window {
    #[js::import = "$0.sessionStorage"]
pub fn session_storage(&self) -> Result<Option<Storage>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `alert()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/alert)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.alert()"]
pub fn alert(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `alert()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/alert)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.alert($1)"]
pub fn alert_with_message(&self, message: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `blur()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/blur)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.blur()"]
pub fn blur(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `cancelIdleCallback()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/cancelIdleCallback)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.cancelIdleCallback($1)"]
pub fn cancel_idle_callback(&self, handle: u32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `captureEvents()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/captureEvents)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.captureEvents()"]
pub fn capture_events(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `close()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/close)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.close()"]
pub fn close(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `confirm()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/confirm)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.confirm()"]
pub fn confirm(&self) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `confirm()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/confirm)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.confirm($1)"]
pub fn confirm_with_message(&self, message: &str) -> Result<bool, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `focus()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/focus)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.focus()"]
pub fn focus(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "CssStyleDeclaration", feature = "Element",))]
    #[doc = "The `getComputedStyle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/getComputedStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleDeclaration`, `Element`, `Window`*"]
impl Window {
    #[js::import = "$0.getComputedStyle($1)"]
pub fn get_computed_style(&self, elt: &Element) -> Result<Option<CssStyleDeclaration>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "CssStyleDeclaration", feature = "Element",))]
    #[doc = "The `getComputedStyle()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/getComputedStyle)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `CssStyleDeclaration`, `Element`, `Window`*"]
impl Window {
    #[js::import = "$0.getComputedStyle($1, $2)"]
pub fn get_computed_style_with_pseudo_elt(&self, elt: &Element, pseudo_elt: &str) -> Result<Option<CssStyleDeclaration>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Selection")]
    #[doc = "The `getSelection()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/getSelection)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Selection`, `Window`*"]
impl Window {
    #[js::import = "$0.getSelection()"]
pub fn get_selection(&self) -> Result<Option<Selection>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "MediaQueryList")]
    #[doc = "The `matchMedia()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/matchMedia)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MediaQueryList`, `Window`*"]
impl Window {
    #[js::import = "$0.matchMedia($1)"]
pub fn match_media(&self, query: &str) -> Result<Option<MediaQueryList>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `moveBy()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/moveBy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.moveBy($1, $2)"]
pub fn move_by(&self, x: i32, y: i32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `moveTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/moveTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.moveTo($1, $2)"]
pub fn move_to(&self, x: i32, y: i32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.open()"]
pub fn open(&self) -> Result<Option<Window>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.open($1)"]
pub fn open_with_url(&self, url: &str) -> Result<Option<Window>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.open($1, $2)"]
pub fn open_with_url_and_target(&self, url: &str, target: &str) -> Result<Option<Window>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `open()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/open)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.open($1, $2, $3)"]
pub fn open_with_url_and_target_and_features(&self, url: &str, target: &str, features: &str) -> Result<Option<Window>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `postMessage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/postMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.postMessage($1, $2)"]
pub fn post_message(&self, message: &::wasm_bindgen::JsValue, target_origin: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `postMessage()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/postMessage)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.postMessage($1, $2, $3)"]
pub fn post_message_with_transfer(&self, message: &::wasm_bindgen::JsValue, target_origin: &str, transfer: &::wasm_bindgen::JsValue) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `print()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/print)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.print()"]
pub fn print(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prompt()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/prompt)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.prompt()"]
pub fn prompt(&self) -> Result<Option<::alloc::string::String>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prompt()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/prompt)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.prompt($1)"]
pub fn prompt_with_message(&self, message: &str) -> Result<Option<::alloc::string::String>, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `prompt()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/prompt)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.prompt($1, $2)"]
pub fn prompt_with_message_and_default(&self, message: &str, default: &str) -> Result<Option<::alloc::string::String>, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `queryLocalFonts()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/queryLocalFonts)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Window {
    #[js::import = "$0.queryLocalFonts()"]
pub fn query_local_fonts(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "QueryOptions")]
    #[doc = "The `queryLocalFonts()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/queryLocalFonts)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `QueryOptions`, `Window`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Window {
    #[js::import = "$0.queryLocalFonts($1)"]
pub fn query_local_fonts_with_options(&self, options: &QueryOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `releaseEvents()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/releaseEvents)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.releaseEvents()"]
pub fn release_events(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `requestIdleCallback()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/requestIdleCallback)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.requestIdleCallback($1)"]
pub fn request_idle_callback(&self, callback: &::js_sys::Function) -> Result<u32, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "IdleRequestOptions")]
    #[doc = "The `requestIdleCallback()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/requestIdleCallback)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `IdleRequestOptions`, `Window`*"]
impl Window {
    #[js::import = "$0.requestIdleCallback($1, $2)"]
pub fn request_idle_callback_with_options(&self, callback: &::js_sys::Function, options: &IdleRequestOptions) -> Result<u32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `resizeBy()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/resizeBy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.resizeBy($1, $2)"]
pub fn resize_by(&self, x: i32, y: i32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `resizeTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/resizeTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.resizeTo($1, $2)"]
pub fn resize_to(&self, x: i32, y: i32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `scroll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scroll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.scroll($1, $2)"]
pub fn scroll_with_x_and_y(&self, x: f64, y: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "The `scroll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scroll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.scroll()"]
pub fn scroll(&self) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ScrollToOptions")]
    #[doc = "The `scroll()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scroll)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollToOptions`, `Window`*"]
impl Window {
    #[js::import = "$0.scroll($1)"]
pub fn scroll_with_scroll_to_options(&self, options: &ScrollToOptions) { panic!("web-sys intrinsic") }
}
    #[doc = "The `scrollBy()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollBy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.scrollBy($1, $2)"]
pub fn scroll_by_with_x_and_y(&self, x: f64, y: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "The `scrollBy()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollBy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.scrollBy()"]
pub fn scroll_by(&self) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ScrollToOptions")]
    #[doc = "The `scrollBy()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollBy)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollToOptions`, `Window`*"]
impl Window {
    #[js::import = "$0.scrollBy($1)"]
pub fn scroll_by_with_scroll_to_options(&self, options: &ScrollToOptions) { panic!("web-sys intrinsic") }
}
    #[doc = "The `scrollTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.scrollTo($1, $2)"]
pub fn scroll_to_with_x_and_y(&self, x: f64, y: f64) { panic!("web-sys intrinsic") }
}
    #[doc = "The `scrollTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.scrollTo()"]
pub fn scroll_to(&self) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ScrollToOptions")]
    #[doc = "The `scrollTo()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollTo)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ScrollToOptions`, `Window`*"]
impl Window {
    #[js::import = "$0.scrollTo($1)"]
pub fn scroll_to_with_scroll_to_options(&self, options: &ScrollToOptions) { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `showDirectoryPicker()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/showDirectoryPicker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Window {
    #[js::import = "$0.showDirectoryPicker()"]
pub fn show_directory_picker(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "DirectoryPickerOptions")]
    #[doc = "The `showDirectoryPicker()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/showDirectoryPicker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `DirectoryPickerOptions`, `Window`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Window {
    #[js::import = "$0.showDirectoryPicker($1)"]
pub fn show_directory_picker_with_options(&self, options: &DirectoryPickerOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `showOpenFilePicker()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/showOpenFilePicker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Window {
    #[js::import = "$0.showOpenFilePicker()"]
pub fn show_open_file_picker(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "OpenFilePickerOptions")]
    #[doc = "The `showOpenFilePicker()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/showOpenFilePicker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OpenFilePickerOptions`, `Window`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Window {
    #[js::import = "$0.showOpenFilePicker($1)"]
pub fn show_open_file_picker_with_options(&self, options: &OpenFilePickerOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[doc = "The `showSaveFilePicker()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/showSaveFilePicker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Window {
    #[js::import = "$0.showSaveFilePicker()"]
pub fn show_save_file_picker(&self) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "SaveFilePickerOptions")]
    #[doc = "The `showSaveFilePicker()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/showSaveFilePicker)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SaveFilePickerOptions`, `Window`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Window {
    #[js::import = "$0.showSaveFilePicker($1)"]
pub fn show_save_file_picker_with_options(&self, options: &SaveFilePickerOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `stop()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/stop)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.stop()"]
pub fn stop(&self) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "Indexing getter. As in the literal Javascript `this[key]`."]
    #[doc = ""]
    #[doc = ""]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.None($1)"]
pub fn get(&self, name: &str) -> Option<::js_sys::Object> { panic!("web-sys intrinsic") }
}
    #[doc = "The `cancelAnimationFrame()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/cancelAnimationFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.cancelAnimationFrame($1)"]
pub fn cancel_animation_frame(&self, handle: i32) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `requestAnimationFrame()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/requestAnimationFrame)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.requestAnimationFrame($1)"]
pub fn request_animation_frame(&self, callback: &::js_sys::Function) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `atob()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/atob)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.atob($1)"]
pub fn atob(&self, atob: &str) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `btoa()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/btoa)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.btoa($1)"]
pub fn btoa(&self, btoa: &str) -> Result<::alloc::string::String, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `clearInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/clearInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.clearInterval()"]
pub fn clear_interval(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `clearInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/clearInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.clearInterval($1)"]
pub fn clear_interval_with_handle(&self, handle: i32) { panic!("web-sys intrinsic") }
}
    #[doc = "The `clearTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/clearTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.clearTimeout()"]
pub fn clear_timeout(&self) { panic!("web-sys intrinsic") }
}
    #[doc = "The `clearTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/clearTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.clearTimeout($1)"]
pub fn clear_timeout_with_handle(&self, handle: i32) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlImageElement")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlImageElement`, `Window`*"]
impl Window {
    #[js::import = "$0.createImageBitmap($1)"]
pub fn create_image_bitmap_with_html_image_element(&self, a_image: &HtmlImageElement) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgImageElement")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgImageElement`, `Window`*"]
impl Window {
    #[js::import = "$0.createImageBitmap($1)"]
pub fn create_image_bitmap_with_svg_image_element(&self, a_image: &SvgImageElement) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlCanvasElement")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `Window`*"]
impl Window {
    #[js::import = "$0.createImageBitmap($1)"]
pub fn create_image_bitmap_with_html_canvas_element(&self, a_image: &HtmlCanvasElement) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlVideoElement")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`, `Window`*"]
impl Window {
    #[js::import = "$0.createImageBitmap($1)"]
pub fn create_image_bitmap_with_html_video_element(&self, a_image: &HtmlVideoElement) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`, `Window`*"]
impl Window {
    #[js::import = "$0.createImageBitmap($1)"]
pub fn create_image_bitmap_with_image_bitmap(&self, a_image: &ImageBitmap) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "OffscreenCanvas")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvas`, `Window`*"]
impl Window {
    #[js::import = "$0.createImageBitmap($1)"]
pub fn create_image_bitmap_with_offscreen_canvas(&self, a_image: &OffscreenCanvas) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoFrame")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`, `Window`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Window {
    #[js::import = "$0.createImageBitmap($1)"]
pub fn create_image_bitmap_with_video_frame(&self, a_image: &VideoFrame) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Blob")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `Window`*"]
impl Window {
    #[js::import = "$0.createImageBitmap($1)"]
pub fn create_image_bitmap_with_blob(&self, a_image: &Blob) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ImageData")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`, `Window`*"]
impl Window {
    #[js::import = "$0.createImageBitmap($1)"]
pub fn create_image_bitmap_with_image_data(&self, a_image: &ImageData) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "HtmlImageElement", feature = "ImageBitmapOptions",))]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlImageElement`, `ImageBitmapOptions`, `Window`*"]
impl Window {
    #[js::import = "$0.createImageBitmap($1, $2)"]
pub fn create_image_bitmap_with_html_image_element_and_image_bitmap_options(&self, a_image: &HtmlImageElement, a_options: &ImageBitmapOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "ImageBitmapOptions", feature = "SvgImageElement",))]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `SvgImageElement`, `Window`*"]
impl Window {
    #[js::import = "$0.createImageBitmap($1, $2)"]
pub fn create_image_bitmap_with_svg_image_element_and_image_bitmap_options(&self, a_image: &SvgImageElement, a_options: &ImageBitmapOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "HtmlCanvasElement", feature = "ImageBitmapOptions",))]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `ImageBitmapOptions`, `Window`*"]
impl Window {
    #[js::import = "$0.createImageBitmap($1, $2)"]
pub fn create_image_bitmap_with_html_canvas_element_and_image_bitmap_options(&self, a_image: &HtmlCanvasElement, a_options: &ImageBitmapOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "HtmlVideoElement", feature = "ImageBitmapOptions",))]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`, `ImageBitmapOptions`, `Window`*"]
impl Window {
    #[js::import = "$0.createImageBitmap($1, $2)"]
pub fn create_image_bitmap_with_html_video_element_and_image_bitmap_options(&self, a_image: &HtmlVideoElement, a_options: &ImageBitmapOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "ImageBitmap", feature = "ImageBitmapOptions",))]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`, `ImageBitmapOptions`, `Window`*"]
impl Window {
    #[js::import = "$0.createImageBitmap($1, $2)"]
pub fn create_image_bitmap_with_image_bitmap_and_image_bitmap_options(&self, a_image: &ImageBitmap, a_options: &ImageBitmapOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "ImageBitmapOptions", feature = "OffscreenCanvas",))]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `OffscreenCanvas`, `Window`*"]
impl Window {
    #[js::import = "$0.createImageBitmap($1, $2)"]
pub fn create_image_bitmap_with_offscreen_canvas_and_image_bitmap_options(&self, a_image: &OffscreenCanvas, a_options: &ImageBitmapOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "ImageBitmapOptions", feature = "VideoFrame",))]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `VideoFrame`, `Window`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Window {
    #[js::import = "$0.createImageBitmap($1, $2)"]
pub fn create_image_bitmap_with_video_frame_and_image_bitmap_options(&self, a_image: &VideoFrame, a_options: &ImageBitmapOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "Blob", feature = "ImageBitmapOptions",))]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `ImageBitmapOptions`, `Window`*"]
impl Window {
    #[js::import = "$0.createImageBitmap($1, $2)"]
pub fn create_image_bitmap_with_blob_and_image_bitmap_options(&self, a_image: &Blob, a_options: &ImageBitmapOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "ImageBitmapOptions", feature = "ImageData",))]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `ImageData`, `Window`*"]
impl Window {
    #[js::import = "$0.createImageBitmap($1, $2)"]
pub fn create_image_bitmap_with_image_data_and_image_bitmap_options(&self, a_image: &ImageData, a_options: &ImageBitmapOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlImageElement")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlImageElement`, `Window`*"]
impl Window {
    #[js::import = "$0.createImageBitmap($1, $2, $3, $4, $5)"]
pub fn create_image_bitmap_with_html_image_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh(&self, a_image: &HtmlImageElement, a_sx: i32, a_sy: i32, a_sw: i32, a_sh: i32) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "SvgImageElement")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `SvgImageElement`, `Window`*"]
impl Window {
    #[js::import = "$0.createImageBitmap($1, $2, $3, $4, $5)"]
pub fn create_image_bitmap_with_svg_image_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh(&self, a_image: &SvgImageElement, a_sx: i32, a_sy: i32, a_sw: i32, a_sh: i32) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlCanvasElement")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `Window`*"]
impl Window {
    #[js::import = "$0.createImageBitmap($1, $2, $3, $4, $5)"]
pub fn create_image_bitmap_with_html_canvas_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh(&self, a_image: &HtmlCanvasElement, a_sx: i32, a_sy: i32, a_sw: i32, a_sh: i32) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "HtmlVideoElement")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`, `Window`*"]
impl Window {
    #[js::import = "$0.createImageBitmap($1, $2, $3, $4, $5)"]
pub fn create_image_bitmap_with_html_video_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh(&self, a_image: &HtmlVideoElement, a_sx: i32, a_sy: i32, a_sw: i32, a_sh: i32) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ImageBitmap")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`, `Window`*"]
impl Window {
    #[js::import = "$0.createImageBitmap($1, $2, $3, $4, $5)"]
pub fn create_image_bitmap_with_image_bitmap_and_a_sx_and_a_sy_and_a_sw_and_a_sh(&self, a_image: &ImageBitmap, a_sx: i32, a_sy: i32, a_sw: i32, a_sh: i32) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "OffscreenCanvas")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `OffscreenCanvas`, `Window`*"]
impl Window {
    #[js::import = "$0.createImageBitmap($1, $2, $3, $4, $5)"]
pub fn create_image_bitmap_with_offscreen_canvas_and_a_sx_and_a_sy_and_a_sw_and_a_sh(&self, a_image: &OffscreenCanvas, a_sx: i32, a_sy: i32, a_sw: i32, a_sh: i32) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(feature = "VideoFrame")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `VideoFrame`, `Window`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Window {
    #[js::import = "$0.createImageBitmap($1, $2, $3, $4, $5)"]
pub fn create_image_bitmap_with_video_frame_and_a_sx_and_a_sy_and_a_sw_and_a_sh(&self, a_image: &VideoFrame, a_sx: i32, a_sy: i32, a_sw: i32, a_sh: i32) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Blob")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `Window`*"]
impl Window {
    #[js::import = "$0.createImageBitmap($1, $2, $3, $4, $5)"]
pub fn create_image_bitmap_with_blob_and_a_sx_and_a_sy_and_a_sw_and_a_sh(&self, a_image: &Blob, a_sx: i32, a_sy: i32, a_sw: i32, a_sh: i32) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "ImageData")]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageData`, `Window`*"]
impl Window {
    #[js::import = "$0.createImageBitmap($1, $2, $3, $4, $5)"]
pub fn create_image_bitmap_with_image_data_and_a_sx_and_a_sy_and_a_sw_and_a_sh(&self, a_image: &ImageData, a_sx: i32, a_sy: i32, a_sw: i32, a_sh: i32) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "HtmlImageElement", feature = "ImageBitmapOptions",))]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlImageElement`, `ImageBitmapOptions`, `Window`*"]
impl Window {
    #[js::import = "$0.createImageBitmap($1, $2, $3, $4, $5, $6)"]
pub fn create_image_bitmap_with_html_image_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh_and_a_options(&self, a_image: &HtmlImageElement, a_sx: i32, a_sy: i32, a_sw: i32, a_sh: i32, a_options: &ImageBitmapOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "ImageBitmapOptions", feature = "SvgImageElement",))]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `SvgImageElement`, `Window`*"]
impl Window {
    #[js::import = "$0.createImageBitmap($1, $2, $3, $4, $5, $6)"]
pub fn create_image_bitmap_with_svg_image_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh_and_a_options(&self, a_image: &SvgImageElement, a_sx: i32, a_sy: i32, a_sw: i32, a_sh: i32, a_options: &ImageBitmapOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "HtmlCanvasElement", feature = "ImageBitmapOptions",))]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlCanvasElement`, `ImageBitmapOptions`, `Window`*"]
impl Window {
    #[js::import = "$0.createImageBitmap($1, $2, $3, $4, $5, $6)"]
pub fn create_image_bitmap_with_html_canvas_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh_and_a_options(&self, a_image: &HtmlCanvasElement, a_sx: i32, a_sy: i32, a_sw: i32, a_sh: i32, a_options: &ImageBitmapOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "HtmlVideoElement", feature = "ImageBitmapOptions",))]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `HtmlVideoElement`, `ImageBitmapOptions`, `Window`*"]
impl Window {
    #[js::import = "$0.createImageBitmap($1, $2, $3, $4, $5, $6)"]
pub fn create_image_bitmap_with_html_video_element_and_a_sx_and_a_sy_and_a_sw_and_a_sh_and_a_options(&self, a_image: &HtmlVideoElement, a_sx: i32, a_sy: i32, a_sw: i32, a_sh: i32, a_options: &ImageBitmapOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "ImageBitmap", feature = "ImageBitmapOptions",))]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmap`, `ImageBitmapOptions`, `Window`*"]
impl Window {
    #[js::import = "$0.createImageBitmap($1, $2, $3, $4, $5, $6)"]
pub fn create_image_bitmap_with_image_bitmap_and_a_sx_and_a_sy_and_a_sw_and_a_sh_and_a_options(&self, a_image: &ImageBitmap, a_sx: i32, a_sy: i32, a_sw: i32, a_sh: i32, a_options: &ImageBitmapOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "ImageBitmapOptions", feature = "OffscreenCanvas",))]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `OffscreenCanvas`, `Window`*"]
impl Window {
    #[js::import = "$0.createImageBitmap($1, $2, $3, $4, $5, $6)"]
pub fn create_image_bitmap_with_offscreen_canvas_and_a_sx_and_a_sy_and_a_sw_and_a_sh_and_a_options(&self, a_image: &OffscreenCanvas, a_sx: i32, a_sy: i32, a_sw: i32, a_sh: i32, a_options: &ImageBitmapOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(web_sys_unstable_apis)]
    #[cfg(all(feature = "ImageBitmapOptions", feature = "VideoFrame",))]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `VideoFrame`, `Window`*"]
    #[doc = ""]
    #[doc = "*This API is unstable and requires `--cfg=web_sys_unstable_apis` to be activated, as"]
    #[doc = "[described in the `wasm-bindgen` guide](https://wasm-bindgen.github.io/wasm-bindgen/web-sys/unstable-apis.html)*"]
impl Window {
    #[js::import = "$0.createImageBitmap($1, $2, $3, $4, $5, $6)"]
pub fn create_image_bitmap_with_video_frame_and_a_sx_and_a_sy_and_a_sw_and_a_sh_and_a_options(&self, a_image: &VideoFrame, a_sx: i32, a_sy: i32, a_sw: i32, a_sh: i32, a_options: &ImageBitmapOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "Blob", feature = "ImageBitmapOptions",))]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Blob`, `ImageBitmapOptions`, `Window`*"]
impl Window {
    #[js::import = "$0.createImageBitmap($1, $2, $3, $4, $5, $6)"]
pub fn create_image_bitmap_with_blob_and_a_sx_and_a_sy_and_a_sw_and_a_sh_and_a_options(&self, a_image: &Blob, a_sx: i32, a_sy: i32, a_sw: i32, a_sh: i32, a_options: &ImageBitmapOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "ImageBitmapOptions", feature = "ImageData",))]
    #[doc = "The `createImageBitmap()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/createImageBitmap)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `ImageBitmapOptions`, `ImageData`, `Window`*"]
impl Window {
    #[js::import = "$0.createImageBitmap($1, $2, $3, $4, $5, $6)"]
pub fn create_image_bitmap_with_image_data_and_a_sx_and_a_sy_and_a_sw_and_a_sh_and_a_options(&self, a_image: &ImageData, a_sx: i32, a_sy: i32, a_sw: i32, a_sh: i32, a_options: &ImageBitmapOptions) -> Result<::js_sys::Promise, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Request")]
    #[doc = "The `fetch()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/fetch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`, `Window`*"]
impl Window {
    #[js::import = "$0.fetch($1)"]
pub fn fetch_with_request(&self, input: &Request) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `fetch()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/fetch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.fetch($1)"]
pub fn fetch_with_str(&self, input: &str) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(all(feature = "Request", feature = "RequestInit",))]
    #[doc = "The `fetch()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/fetch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Request`, `RequestInit`, `Window`*"]
impl Window {
    #[js::import = "$0.fetch($1, $2)"]
pub fn fetch_with_request_and_init(&self, input: &Request, init: &RequestInit) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "RequestInit")]
    #[doc = "The `fetch()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/fetch)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RequestInit`, `Window`*"]
impl Window {
    #[js::import = "$0.fetch($1, $2)"]
pub fn fetch_with_str_and_init(&self, input: &str, init: &RequestInit) -> ::js_sys::Promise { panic!("web-sys intrinsic") }
}
    #[doc = "The `queueMicrotask()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/queueMicrotask)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.queueMicrotask($1)"]
pub fn queue_microtask(&self, callback: &::js_sys::Function) { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setInterval($1)"]
pub fn set_interval_with_callback(&self, handler: &::js_sys::Function) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setInterval($1, $2, $3)"]
pub fn set_interval_with_callback_and_timeout_and_arguments(&self, handler: &::js_sys::Function, timeout: i32, arguments: &::js_sys::Array) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setInterval($1, $2)"]
pub fn set_interval_with_callback_and_timeout_and_arguments_0(&self, handler: &::js_sys::Function, timeout: i32) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setInterval($1, $2, $3)"]
pub fn set_interval_with_callback_and_timeout_and_arguments_1(&self, handler: &::js_sys::Function, timeout: i32, arguments_1: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setInterval($1, $2, $3, $4)"]
pub fn set_interval_with_callback_and_timeout_and_arguments_2(&self, handler: &::js_sys::Function, timeout: i32, arguments_1: &::wasm_bindgen::JsValue, arguments_2: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setInterval($1, $2, $3, $4, $5)"]
pub fn set_interval_with_callback_and_timeout_and_arguments_3(&self, handler: &::js_sys::Function, timeout: i32, arguments_1: &::wasm_bindgen::JsValue, arguments_2: &::wasm_bindgen::JsValue, arguments_3: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setInterval($1, $2, $3, $4, $5, $6)"]
pub fn set_interval_with_callback_and_timeout_and_arguments_4(&self, handler: &::js_sys::Function, timeout: i32, arguments_1: &::wasm_bindgen::JsValue, arguments_2: &::wasm_bindgen::JsValue, arguments_3: &::wasm_bindgen::JsValue, arguments_4: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setInterval($1, $2, $3, $4, $5, $6, $7)"]
pub fn set_interval_with_callback_and_timeout_and_arguments_5(&self, handler: &::js_sys::Function, timeout: i32, arguments_1: &::wasm_bindgen::JsValue, arguments_2: &::wasm_bindgen::JsValue, arguments_3: &::wasm_bindgen::JsValue, arguments_4: &::wasm_bindgen::JsValue, arguments_5: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setInterval($1, $2, $3, $4, $5, $6, $7, $8)"]
pub fn set_interval_with_callback_and_timeout_and_arguments_6(&self, handler: &::js_sys::Function, timeout: i32, arguments_1: &::wasm_bindgen::JsValue, arguments_2: &::wasm_bindgen::JsValue, arguments_3: &::wasm_bindgen::JsValue, arguments_4: &::wasm_bindgen::JsValue, arguments_5: &::wasm_bindgen::JsValue, arguments_6: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setInterval($1, $2, $3, $4, $5, $6, $7, $8, $9)"]
pub fn set_interval_with_callback_and_timeout_and_arguments_7(&self, handler: &::js_sys::Function, timeout: i32, arguments_1: &::wasm_bindgen::JsValue, arguments_2: &::wasm_bindgen::JsValue, arguments_3: &::wasm_bindgen::JsValue, arguments_4: &::wasm_bindgen::JsValue, arguments_5: &::wasm_bindgen::JsValue, arguments_6: &::wasm_bindgen::JsValue, arguments_7: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setInterval($1)"]
pub fn set_interval_with_str(&self, handler: &str) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setInterval($1, $2, $3)"]
pub fn set_interval_with_str_and_timeout_and_unused(&self, handler: &str, timeout: i32, unused: &::js_sys::Array) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setInterval($1, $2)"]
pub fn set_interval_with_str_and_timeout_and_unused_0(&self, handler: &str, timeout: i32) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setInterval($1, $2, $3)"]
pub fn set_interval_with_str_and_timeout_and_unused_1(&self, handler: &str, timeout: i32, unused_1: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setInterval($1, $2, $3, $4)"]
pub fn set_interval_with_str_and_timeout_and_unused_2(&self, handler: &str, timeout: i32, unused_1: &::wasm_bindgen::JsValue, unused_2: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setInterval($1, $2, $3, $4, $5)"]
pub fn set_interval_with_str_and_timeout_and_unused_3(&self, handler: &str, timeout: i32, unused_1: &::wasm_bindgen::JsValue, unused_2: &::wasm_bindgen::JsValue, unused_3: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setInterval($1, $2, $3, $4, $5, $6)"]
pub fn set_interval_with_str_and_timeout_and_unused_4(&self, handler: &str, timeout: i32, unused_1: &::wasm_bindgen::JsValue, unused_2: &::wasm_bindgen::JsValue, unused_3: &::wasm_bindgen::JsValue, unused_4: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setInterval($1, $2, $3, $4, $5, $6, $7)"]
pub fn set_interval_with_str_and_timeout_and_unused_5(&self, handler: &str, timeout: i32, unused_1: &::wasm_bindgen::JsValue, unused_2: &::wasm_bindgen::JsValue, unused_3: &::wasm_bindgen::JsValue, unused_4: &::wasm_bindgen::JsValue, unused_5: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setInterval($1, $2, $3, $4, $5, $6, $7, $8)"]
pub fn set_interval_with_str_and_timeout_and_unused_6(&self, handler: &str, timeout: i32, unused_1: &::wasm_bindgen::JsValue, unused_2: &::wasm_bindgen::JsValue, unused_3: &::wasm_bindgen::JsValue, unused_4: &::wasm_bindgen::JsValue, unused_5: &::wasm_bindgen::JsValue, unused_6: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setInterval()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setInterval)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setInterval($1, $2, $3, $4, $5, $6, $7, $8, $9)"]
pub fn set_interval_with_str_and_timeout_and_unused_7(&self, handler: &str, timeout: i32, unused_1: &::wasm_bindgen::JsValue, unused_2: &::wasm_bindgen::JsValue, unused_3: &::wasm_bindgen::JsValue, unused_4: &::wasm_bindgen::JsValue, unused_5: &::wasm_bindgen::JsValue, unused_6: &::wasm_bindgen::JsValue, unused_7: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setTimeout($1)"]
pub fn set_timeout_with_callback(&self, handler: &::js_sys::Function) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setTimeout($1, $2, $3)"]
pub fn set_timeout_with_callback_and_timeout_and_arguments(&self, handler: &::js_sys::Function, timeout: i32, arguments: &::js_sys::Array) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setTimeout($1, $2)"]
pub fn set_timeout_with_callback_and_timeout_and_arguments_0(&self, handler: &::js_sys::Function, timeout: i32) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setTimeout($1, $2, $3)"]
pub fn set_timeout_with_callback_and_timeout_and_arguments_1(&self, handler: &::js_sys::Function, timeout: i32, arguments_1: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setTimeout($1, $2, $3, $4)"]
pub fn set_timeout_with_callback_and_timeout_and_arguments_2(&self, handler: &::js_sys::Function, timeout: i32, arguments_1: &::wasm_bindgen::JsValue, arguments_2: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setTimeout($1, $2, $3, $4, $5)"]
pub fn set_timeout_with_callback_and_timeout_and_arguments_3(&self, handler: &::js_sys::Function, timeout: i32, arguments_1: &::wasm_bindgen::JsValue, arguments_2: &::wasm_bindgen::JsValue, arguments_3: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setTimeout($1, $2, $3, $4, $5, $6)"]
pub fn set_timeout_with_callback_and_timeout_and_arguments_4(&self, handler: &::js_sys::Function, timeout: i32, arguments_1: &::wasm_bindgen::JsValue, arguments_2: &::wasm_bindgen::JsValue, arguments_3: &::wasm_bindgen::JsValue, arguments_4: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setTimeout($1, $2, $3, $4, $5, $6, $7)"]
pub fn set_timeout_with_callback_and_timeout_and_arguments_5(&self, handler: &::js_sys::Function, timeout: i32, arguments_1: &::wasm_bindgen::JsValue, arguments_2: &::wasm_bindgen::JsValue, arguments_3: &::wasm_bindgen::JsValue, arguments_4: &::wasm_bindgen::JsValue, arguments_5: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setTimeout($1, $2, $3, $4, $5, $6, $7, $8)"]
pub fn set_timeout_with_callback_and_timeout_and_arguments_6(&self, handler: &::js_sys::Function, timeout: i32, arguments_1: &::wasm_bindgen::JsValue, arguments_2: &::wasm_bindgen::JsValue, arguments_3: &::wasm_bindgen::JsValue, arguments_4: &::wasm_bindgen::JsValue, arguments_5: &::wasm_bindgen::JsValue, arguments_6: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setTimeout($1, $2, $3, $4, $5, $6, $7, $8, $9)"]
pub fn set_timeout_with_callback_and_timeout_and_arguments_7(&self, handler: &::js_sys::Function, timeout: i32, arguments_1: &::wasm_bindgen::JsValue, arguments_2: &::wasm_bindgen::JsValue, arguments_3: &::wasm_bindgen::JsValue, arguments_4: &::wasm_bindgen::JsValue, arguments_5: &::wasm_bindgen::JsValue, arguments_6: &::wasm_bindgen::JsValue, arguments_7: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setTimeout($1)"]
pub fn set_timeout_with_str(&self, handler: &str) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setTimeout($1, $2, $3)"]
pub fn set_timeout_with_str_and_timeout_and_unused(&self, handler: &str, timeout: i32, unused: &::js_sys::Array) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setTimeout($1, $2)"]
pub fn set_timeout_with_str_and_timeout_and_unused_0(&self, handler: &str, timeout: i32) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setTimeout($1, $2, $3)"]
pub fn set_timeout_with_str_and_timeout_and_unused_1(&self, handler: &str, timeout: i32, unused_1: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setTimeout($1, $2, $3, $4)"]
pub fn set_timeout_with_str_and_timeout_and_unused_2(&self, handler: &str, timeout: i32, unused_1: &::wasm_bindgen::JsValue, unused_2: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setTimeout($1, $2, $3, $4, $5)"]
pub fn set_timeout_with_str_and_timeout_and_unused_3(&self, handler: &str, timeout: i32, unused_1: &::wasm_bindgen::JsValue, unused_2: &::wasm_bindgen::JsValue, unused_3: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setTimeout($1, $2, $3, $4, $5, $6)"]
pub fn set_timeout_with_str_and_timeout_and_unused_4(&self, handler: &str, timeout: i32, unused_1: &::wasm_bindgen::JsValue, unused_2: &::wasm_bindgen::JsValue, unused_3: &::wasm_bindgen::JsValue, unused_4: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setTimeout($1, $2, $3, $4, $5, $6, $7)"]
pub fn set_timeout_with_str_and_timeout_and_unused_5(&self, handler: &str, timeout: i32, unused_1: &::wasm_bindgen::JsValue, unused_2: &::wasm_bindgen::JsValue, unused_3: &::wasm_bindgen::JsValue, unused_4: &::wasm_bindgen::JsValue, unused_5: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setTimeout($1, $2, $3, $4, $5, $6, $7, $8)"]
pub fn set_timeout_with_str_and_timeout_and_unused_6(&self, handler: &str, timeout: i32, unused_1: &::wasm_bindgen::JsValue, unused_2: &::wasm_bindgen::JsValue, unused_3: &::wasm_bindgen::JsValue, unused_4: &::wasm_bindgen::JsValue, unused_5: &::wasm_bindgen::JsValue, unused_6: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `setTimeout()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Window/setTimeout)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Window`*"]
impl Window {
    #[js::import = "$0.setTimeout($1, $2, $3, $4, $5, $6, $7, $8, $9)"]
pub fn set_timeout_with_str_and_timeout_and_unused_7(&self, handler: &str, timeout: i32, unused_1: &::wasm_bindgen::JsValue, unused_2: &::wasm_bindgen::JsValue, unused_3: &::wasm_bindgen::JsValue, unused_4: &::wasm_bindgen::JsValue, unused_5: &::wasm_bindgen::JsValue, unused_6: &::wasm_bindgen::JsValue, unused_7: &::wasm_bindgen::JsValue) -> Result<i32, JsValue> { panic!("web-sys intrinsic") }
}
