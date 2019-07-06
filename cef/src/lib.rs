#![feature(non_exhaustive)]
#![feature(type_alias_enum_variants)]

extern crate cef_sys;

mod app;
mod browser;
mod client;
mod frame;
mod ptr;
mod settings;
mod thread;
mod types;
mod window;

use crate::types::string::CefString;
pub use app::*;
pub use browser::*;
pub use client::*;
pub use frame::*;
pub use settings::*;
use std::ptr::null_mut;
use std::sync::Arc;
pub use thread::*;
pub use types::*;
pub use window::*;

pub(crate) trait ToCef<T> {
    fn to_cef(&self) -> *mut T;
}

#[cfg(target_os = "windows")]
mod platform {
    use std::ptr::null_mut;

    pub type CefArgs<'a> = cef_sys::HINSTANCE;

    pub(crate) struct CefMainArgsWrapper {
        pub cef: cef_sys::_cef_main_args_t,
    }

    pub(crate) fn args_to_cef(raw: CefArgs) -> CefMainArgsWrapper {
        CefMainArgsWrapper {
            cef: cef_sys::_cef_main_args_t { instance: raw },
        }
    }

    pub(crate) fn default_args() -> CefMainArgsWrapper {
        args_to_cef(null_mut())
    }
}

#[cfg(not(target_os = "windows"))]
mod platform {
    use std::ffi::CString;
    use std::os::raw::c_char;
    use std::ptr::null_mut;

    pub type CefArgs<'a> = &'a [String];

    pub(crate) struct CefMainArgsWrapper {
        pub cef: cef_sys::_cef_main_args_t,
        pub keepalive: Vec<CString>,
        pub keepalive2: Vec<*mut c_char>,
    }

    pub(crate) fn args_to_cef(raw: CefArgs) -> CefMainArgsWrapper {
        // TODO - won't this cause the types to be freed before the pointers?
        let args = raw
            .iter()
            .map(|x| CString::new(x.as_str()).unwrap())
            .collect::<Vec<CString>>();
        let mut res = CefMainArgsWrapper {
            cef: cef_sys::_cef_main_args_t {
                argc: 0,
                argv: null_mut(),
            },
            keepalive: args,
            keepalive2: Vec::new(),
        };

        res.keepalive2 = res.keepalive.iter().map(|x| x.as_ptr() as *mut _).collect();
        res.cef.argc = res.keepalive2.len() as i32;
        res.cef.argv = res.keepalive2.as_mut_ptr();

        res
    }

    pub(crate) fn default_args() -> CefMainArgsWrapper {
        args_to_cef(&std::env::args().collect::<Vec<String>>())
    }
}

pub use platform::*;

pub fn execute_process_with_args<TApp: App>(
    args: CefArgs,
    application: &Arc<TApp>,
    //    windows_sandbox_info: *mut ::std::os::raw::c_void,
) -> i32 {
    let args = platform::args_to_cef(args);
    unsafe { cef_sys::cef_execute_process(&args.cef, application.to_cef(), null_mut()) }
}
pub fn execute_process<TApp: App>(
    application: &Arc<TApp>,
    //    windows_sandbox_info: *mut ::std::os::raw::c_void,
) -> i32 {
    let args = platform::default_args();
    unsafe { cef_sys::cef_execute_process(&args.cef, application.to_cef(), null_mut()) }
}

pub fn initialize_with_args<TApp: App>(
    args: CefArgs,
    settings: Settings,
    application: &Arc<TApp>,
    //    application: *mut cef_sys::cef_app_t,
    //    windows_sandbox_info: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    let args = platform::args_to_cef(args);
    unsafe {
        cef_sys::cef_initialize(
            &args.cef,
            &settings.to_cef(),
            application.to_cef(),
            null_mut(),
        )
    }
}

pub fn initialize<TApp: App>(
    settings: Settings,
    application: &Arc<TApp>,
    //    application: *mut cef_sys::cef_app_t,
    //    windows_sandbox_info: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    let args = platform::default_args();
    unsafe {
        cef_sys::cef_initialize(
            &args.cef,
            &settings.to_cef(),
            application.to_cef(),
            null_mut(),
        )
    }
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
