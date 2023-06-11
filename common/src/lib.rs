// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

pub mod aggregate;
pub mod event_store;
pub mod command;
pub mod event;