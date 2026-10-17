#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use ::wasm_bindgen::JsValue;
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `MutationEvent` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationEvent`*"]
#[js::opaque]
#[derive(Copy)]
pub struct MutationEvent(pub ::wasm_bindgen::JsValue);
    #[cfg(feature = "Node")]
    #[doc = "Getter for the `relatedNode` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/relatedNode)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationEvent`, `Node`*"]
impl MutationEvent {
    #[js::import = "$0.relatedNode"]
pub fn related_node(&self) -> Option<Node> { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `prevValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/prevValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationEvent`*"]
impl MutationEvent {
    #[js::import = "$0.prevValue"]
pub fn prev_value(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `newValue` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/newValue)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationEvent`*"]
impl MutationEvent {
    #[js::import = "$0.newValue"]
pub fn new_value(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `attrName` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/attrName)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationEvent`*"]
impl MutationEvent {
    #[js::import = "$0.attrName"]
pub fn attr_name(&self) -> ::alloc::string::String { panic!("web-sys intrinsic") }
}
    #[doc = "Getter for the `attrChange` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/attrChange)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationEvent`*"]
impl MutationEvent {
    #[js::import = "$0.attrChange"]
pub fn attr_change(&self) -> u16 { panic!("web-sys intrinsic") }
}
    #[doc = "The `initMutationEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/initMutationEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationEvent`*"]
impl MutationEvent {
    #[js::import = "$0.initMutationEvent($1)"]
pub fn init_mutation_event(&self, type_: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `initMutationEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/initMutationEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationEvent`*"]
impl MutationEvent {
    #[js::import = "$0.initMutationEvent($1, $2)"]
pub fn init_mutation_event_with_can_bubble(&self, type_: &str, can_bubble: bool) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[doc = "The `initMutationEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/initMutationEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationEvent`*"]
impl MutationEvent {
    #[js::import = "$0.initMutationEvent($1, $2, $3)"]
pub fn init_mutation_event_with_can_bubble_and_cancelable(&self, type_: &str, can_bubble: bool, cancelable: bool) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `initMutationEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/initMutationEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationEvent`, `Node`*"]
impl MutationEvent {
    #[js::import = "$0.initMutationEvent($1, $2, $3, $4)"]
pub fn init_mutation_event_with_can_bubble_and_cancelable_and_related_node(&self, type_: &str, can_bubble: bool, cancelable: bool, related_node: Option<&Node>) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `initMutationEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/initMutationEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationEvent`, `Node`*"]
impl MutationEvent {
    #[js::import = "$0.initMutationEvent($1, $2, $3, $4, $5)"]
pub fn init_mutation_event_with_can_bubble_and_cancelable_and_related_node_and_prev_value(&self, type_: &str, can_bubble: bool, cancelable: bool, related_node: Option<&Node>, prev_value: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `initMutationEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/initMutationEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationEvent`, `Node`*"]
impl MutationEvent {
    #[js::import = "$0.initMutationEvent($1, $2, $3, $4, $5, $6)"]
pub fn init_mutation_event_with_can_bubble_and_cancelable_and_related_node_and_prev_value_and_new_value(&self, type_: &str, can_bubble: bool, cancelable: bool, related_node: Option<&Node>, prev_value: &str, new_value: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `initMutationEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/initMutationEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationEvent`, `Node`*"]
impl MutationEvent {
    #[js::import = "$0.initMutationEvent($1, $2, $3, $4, $5, $6, $7)"]
pub fn init_mutation_event_with_can_bubble_and_cancelable_and_related_node_and_prev_value_and_new_value_and_attr_name(&self, type_: &str, can_bubble: bool, cancelable: bool, related_node: Option<&Node>, prev_value: &str, new_value: &str, attr_name: &str) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
    #[cfg(feature = "Node")]
    #[doc = "The `initMutationEvent()` method."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MutationEvent/initMutationEvent)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationEvent`, `Node`*"]
impl MutationEvent {
    #[js::import = "$0.initMutationEvent($1, $2, $3, $4, $5, $6, $7, $8)"]
pub fn init_mutation_event_with_can_bubble_and_cancelable_and_related_node_and_prev_value_and_new_value_and_attr_name_and_attr_change(&self, type_: &str, can_bubble: bool, cancelable: bool, related_node: Option<&Node>, prev_value: &str, new_value: &str, attr_name: &str, attr_change: u16) -> Result<(), JsValue> { panic!("web-sys intrinsic") }
}
impl MutationEvent {
    #[doc = "The `MutationEvent.MODIFICATION` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationEvent`*"]
    pub const MODIFICATION: u16 = 1u64 as u16;
    #[doc = "The `MutationEvent.ADDITION` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationEvent`*"]
    pub const ADDITION: u16 = 2u64 as u16;
    #[doc = "The `MutationEvent.REMOVAL` const."]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `MutationEvent`*"]
    pub const REMOVAL: u16 = 3u64 as u16;
}
