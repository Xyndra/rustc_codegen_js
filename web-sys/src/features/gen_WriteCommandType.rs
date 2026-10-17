#![allow(unused_imports)]
#![allow(clippy::all)]
#[doc = "The `WriteCommandType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `WriteCommandType`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteCommandType {
    Write = "write",
    Seek = "seek",
    Truncate = "truncate",
}
