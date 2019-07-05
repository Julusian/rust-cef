#![feature(non_exhaustive)]

extern crate cef_sys;

mod app;
mod browser;
mod client;
mod ptr;
mod settings;
mod thread;
mod types;
mod window;

use crate::types::string::CefString;
pub use app::*;
pub use browser::*;
pub use client::*;
pub use settings::*;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr::null_mut;
use std::sync::Arc;
pub use thread::*;
pub use types::*;
pub use window::*;

pub(crate) trait ToCef<T> {
    fn to_cef(&self) -> *mut T;
}

pub fn execute_process<TApp: App>(
    args: &[String],
    application: &Arc<TApp>,
    //    windows_sandbox_info: *mut ::std::os::raw::c_void,
) -> i32 {
    let args: Vec<CString> = args
        .iter()
        .map(|x| CString::new(x.as_str()).unwrap())
        .collect();
    let mut args2: Vec<*mut c_char> = args.iter().map(|x| x.as_ptr() as *mut _).collect();
    let args3 = cef_sys::_cef_main_args_t {
        argc: args2.len() as i32,
        argv: args2.as_mut_ptr(),
    };

    unsafe { cef_sys::cef_execute_process(&args3, application.to_cef(), null_mut()) }
}

pub fn initialize<TApp: App>(
    args: &[String],
    settings: Settings,
    application: &Arc<TApp>,
    //    application: *mut cef_sys::cef_app_t,
    //    windows_sandbox_info: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    let args: Vec<CString> = args
        .iter()
        .map(|x| CString::new(x.as_str()).unwrap())
        .collect();
    let mut args2: Vec<*mut c_char> = args.iter().map(|x| x.as_ptr() as *mut _).collect();
    let args3 = cef_sys::_cef_main_args_t {
        argc: args2.len() as i32,
        argv: args2.as_mut_ptr(),
    };

    unsafe { cef_sys::cef_initialize(&args3, &settings.to_cef(), application.to_cef(), null_mut()) }
}

pub fn create_browser_sync<TClient: Client>(
    info: WindowInfo,
    client: &Arc<TClient>,
    url: &str,
    settings: BrowserSettings,
) {
    let _res = unsafe {
        cef_sys::cef_browser_host_create_browser_sync(
            &info.to_cef(),
            client.to_cef(),
            &CefString::from_str(url).into_cef(),
            &settings.to_cef(),
            null_mut(),
            null_mut(),
        )
    };
}

pub fn shutdown() {
    unsafe { cef_sys::cef_shutdown() }
}

pub fn do_message_loop_work() {
    unsafe { cef_sys::cef_do_message_loop_work() }
}

pub fn run_message_loop() {
    unsafe { cef_sys::cef_run_message_loop() }
}

pub fn quit_message_loop() {
    unsafe { cef_sys::cef_quit_message_loop() }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
