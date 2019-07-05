extern crate cef_sys;

mod settings;
mod string;

use cef_sys::{cef_app_t, cef_command_line_t, cef_settings_t, cef_string_t};
pub use settings::*;
use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::ptr::null_mut;
use std::sync::Arc;
use string::CefString;

pub trait App {
    fn on_before_command_line_processing(&self); // TODO - proper type
}

//fn create_cef_app<TApp: App>(application: Option<TApp>) {
//    extern "C" fn on_before_command_line_processing(
//        self_: *mut cef_app_t,
//        process_type: *const cef_string_t,
//        command_line: *mut cef_command_line_t,
//    ) {
//        //
//    }
//    let app = cef_app_t {
//        base: _cef_base_ref_counted_t {},
//        on_before_command_line_processing: Some(on_before_command_line_processing),
//        on_register_custom_schemes: None,
//        get_resource_bundle_handler: None,
//        get_browser_process_handler: None,
//        get_render_process_handler: None,
//    };
//}

pub fn execute_process(
    args: &Vec<String>,
    //    application: *mut cef_app_t,
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

    let mut app = null_mut();

    unsafe { cef_sys::cef_execute_process(&args3, app, null_mut()) }
}

pub fn initialize(
    args: &Vec<String>,
    settings: Settings,
    //    application: Option<TApp>,
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

    let settings2 = settings.to_cef();
    //
    //    let app = cef_app_t {
    //        base: _cef_base_ref_counted_t {},
    //        on_before_command_line_processing: None,
    //        on_register_custom_schemes: None,
    //        get_resource_bundle_handler: None,
    //        get_browser_process_handler: None,
    //        get_render_process_handler: None,
    //    };
    let mut app = null_mut();

    unsafe { cef_sys::cef_initialize(&args3, &settings2, app, null_mut()) }
}

pub fn shutdown() {
    unsafe { cef_sys::cef_shutdown() }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
