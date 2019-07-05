extern crate cef_sys;

mod app;
mod ptr;
mod settings;
mod string;

pub use app::*;
pub use settings::*;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr::null_mut;
use std::sync::Arc;

pub(crate) trait ToCef<T> {
    fn to_cef(&self) -> *mut T;
}

//pub struct CefRc<T> {
//    ptr: *const T, // TODO - type?
//}
//impl<T> Deref for CefRc<T> {
//    type Target = T;
//
//    fn deref(&self) -> &Self::Target {
//        unsafe { &*self.ptr }
//    }
//}
//impl<T> CefRc<T> {
//    pub fn wrap(ptr: Arc<T>) {}
//    //    pub fn new(ptr: *T) {
//    //        if ptr != null() {
//    //            ptr->AddRef();
//    //        }
//    //    }
//}

pub fn execute_process<TApp: App>(
    args: &Vec<String>,
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
    args: &Vec<String>,
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
