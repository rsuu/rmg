#[cfg(target_arch = "x86_64")]
pub mod desktop;

#[cfg(target_arch = "wasm32")]
pub mod web;
