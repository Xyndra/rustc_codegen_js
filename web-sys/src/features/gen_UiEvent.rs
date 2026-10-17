#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `UiEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UiEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct UiEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "Window")]
    #[doc = "Getter for the `view` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/view)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UiEvent`, `Window`*"]
impl UiEvent {
    #[js::import = "$0.view"]
pub fn view(&self) -> Option<Window> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `detail` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/detail)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UiEvent`*"]
impl UiEvent {
    #[js::import = "$0.detail"]
pub fn detail(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `layerX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/layerX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UiEvent`*"]
impl UiEvent {
    #[js::import = "$0.layerX"]
pub fn layer_x(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `layerY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/layerY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UiEvent`*"]
impl UiEvent {
    #[js::import = "$0.layerY"]
pub fn layer_y(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `pageX` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/pageX)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UiEvent`*"]
impl UiEvent {
    #[js::import = "$0.pageX"]
pub fn page_x(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `pageY` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/pageY)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UiEvent`*"]
impl UiEvent {
    #[js::import = "$0.pageY"]
pub fn page_y(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `which` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/which)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UiEvent`*"]
impl UiEvent {
    #[js::import = "$0.which"]
pub fn which(&self) -> u32 { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "Getter for the `rangeParent` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/rangeParent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `Node`, `UiEvent`*"]
impl UiEvent {
    #[js::import = "$0.rangeParent"]
pub fn range_parent(&self) -> Option<Node> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `rangeOffset` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/rangeOffset)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UiEvent`*"]
impl UiEvent {
    #[js::import = "$0.rangeOffset"]
pub fn range_offset(&self) -> i32 { panic!("web-sys intrinsic") }
}
    #[doc = "The `new UiEvent(..)` constructor, creating a new instance of `UiEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/UIEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UiEvent`*"]
impl UiEvent {
    #[js::import = "new UIEvent($0)"]
pub fn new(type_: &str) -> Result<UiEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "UiEventInit")]
    #[doc = "The `new UiEvent(..)` constructor, creating a new instance of `UiEvent`."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/UIEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UiEvent`, `UiEventInit`*"]
impl UiEvent {
    #[js::import = "new UIEvent($0, $1)"]
pub fn new_with_event_init_dict(type_: &str, event_init_dict: &UiEventInit,) -> Result<UiEvent, JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `initUIEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/initUIEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UiEvent`*"]
impl UiEvent {
    #[js::import = "$0.initUIEvent($1)"]
pub fn init_ui_event(&self, a_type: &str) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initUIEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/initUIEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UiEvent`*"]
impl UiEvent {
    #[js::import = "$0.initUIEvent($1, $2)"]
pub fn init_ui_event_with_a_can_bubble(&self, a_type: &str, a_can_bubble: bool) { panic!("web-sys intrinsic") }
}
    #[doc = "The `initUIEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/initUIEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UiEvent`*"]
impl UiEvent {
    #[js::import = "$0.initUIEvent($1, $2, $3)"]
pub fn init_ui_event_with_a_can_bubble_and_a_cancelable(&self, a_type: &str, a_can_bubble: bool, a_cancelable: bool) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initUIEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/initUIEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UiEvent`, `Window`*"]
impl UiEvent {
    #[js::import = "$0.initUIEvent($1, $2, $3, $4)"]
pub fn init_ui_event_with_a_can_bubble_and_a_cancelable_and_a_view(&self, a_type: &str, a_can_bubble: bool, a_cancelable: bool, a_view: Option<&Window>) { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Window")]
    #[doc = "The `initUIEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/UIEvent/initUIEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UiEvent`, `Window`*"]
impl UiEvent {
    #[js::import = "$0.initUIEvent($1, $2, $3, $4, $5)"]
pub fn init_ui_event_with_a_can_bubble_and_a_cancelable_and_a_view_and_a_detail(&self, a_type: &str, a_can_bubble: bool, a_cancelable: bool, a_view: Option<&Window>, a_detail: i32) { panic!("web-sys intrinsic") }
}
impl UiEvent {
    #[doc = "The `UIEvent.SCROLL_PAGE_UP` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UiEvent`*"]
    pub const SCROLL_PAGE_UP: i32 = -32768i64 as i32;
    #[doc = "The `UIEvent.SCROLL_PAGE_DOWN` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `UiEvent`*"]
    pub const SCROLL_PAGE_DOWN: i32 = 32768u64 as i32;
}
