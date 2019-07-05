#[allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    clippy::all
)]
#[link(name = "cef", kind = "dynamic")]
mod bindings;

pub use bindings::*;
