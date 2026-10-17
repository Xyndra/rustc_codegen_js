#![allow(unused_imports)]
#![allow(clippy::all)]
#[doc = "The `RtcDataChannelState` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcDataChannelState`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RtcDataChannelState {
    Connecting = "connecting",
    Open = "open",
    Closing = "closing",
    Closed = "closed",
}
