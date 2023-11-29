//! Client library for [polygon.io](https://www.polygon.io).
#[cfg(feature = "rest")]
pub mod rest;
pub mod types;
#[cfg(feature = "websocket")]
pub mod websocket;
