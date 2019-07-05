use crate::ptr::{wrap_ptr, BaseRefCountedExt, WrapperFor};
use crate::string::CefString;
use crate::ToCef;
use cef_sys::{
    cef_app_t, cef_browser_process_handler_t, cef_command_line_t, cef_render_process_handler_t,
    cef_resource_bundle_handler_t, cef_scheme_registrar_t, cef_string_t,
};
use std::ptr::null_mut;
use std::sync::Arc;

pub trait App {
    fn on_before_command_line_processing(&self, _process_type: &str, _command_line: &CommandLine) {}
    // TODO - finish implementing
    //    fn on_register_custom_schemes(&self) {}
    //    fn get_resource_bundle_handler(&self) {}
    //    fn get_browser_process_handler(&self) {}
    //    fn get_render_process_handler(&self) {}
}

#[repr(C)]
struct AppWrapper<TApp: App> {
    base: cef_app_t,
    internal: Arc<TApp>,
}
unsafe impl<TApp: App> WrapperFor<cef_app_t> for AppWrapper<TApp> {}
impl<TApp: App> AppWrapper<TApp> {
    fn from_ptr<'a>(ptr: *mut cef_app_t) -> &'a mut BaseRefCountedExt<cef_app_t, AppWrapper<TApp>> {
        unsafe { std::mem::transmute(ptr) }
    }
    extern "C" fn on_before_command_line_processing(
        app: *mut cef_app_t,
        process_type: *const cef_string_t,
        command_line: *mut cef_command_line_t,
    ) {
        let app = Self::from_ptr(app);
        let process_type = CefString::from_cef(process_type);
        let command_line = CommandLine::from(command_line, false);
        app.internal
            .on_before_command_line_processing(&process_type.to_string(), &command_line);
    }

    extern "C" fn on_register_custom_schemes(
        _app: *mut cef_app_t,
        _registrar: *mut cef_scheme_registrar_t,
    ) {
        //        let app = Self::from_ptr(app);
        // TODO registrar
        //        app.internal.on_register_custom_schemes();
    }
    extern "C" fn get_resource_bundle_handler(
        _app: *mut cef_app_t,
    ) -> *mut cef_resource_bundle_handler_t {
        //        let app = Self::from_ptr(app);

        //        app.internal.get_resource_bundle_handler();
        null_mut()
    }
    extern "C" fn get_browser_process_handler(
        _app: *mut cef_app_t,
    ) -> *mut cef_browser_process_handler_t {
        //        let app = Self::from_ptr(app);

        //        app.internal.get_browser_process_handler();
        null_mut()
    }
    extern "C" fn get_render_process_handler(
        _app: *mut cef_app_t,
    ) -> *mut cef_render_process_handler_t {
        //        let app = Self::from_ptr(app);

        //        app.internal.get_render_process_handler();
        null_mut()
    }
}
impl<TApp: App> ToCef<cef_app_t> for Arc<TApp> {
    fn to_cef(&self) -> *mut cef_app_t {
        wrap_ptr(|base| AppWrapper {
            base: cef_app_t {
                base,
                on_before_command_line_processing: Some(
                    AppWrapper::<TApp>::on_before_command_line_processing,
                ),
                on_register_custom_schemes: Some(AppWrapper::<TApp>::on_register_custom_schemes),
                get_resource_bundle_handler: Some(AppWrapper::<TApp>::get_resource_bundle_handler),
                get_browser_process_handler: Some(AppWrapper::<TApp>::get_browser_process_handler),
                get_render_process_handler: Some(AppWrapper::<TApp>::get_render_process_handler),
            },
            internal: self.clone(),
        })
    }
}

pub struct CommandLine {
    ptr: *mut cef_command_line_t,
    track_ref: bool,
}
impl Drop for CommandLine {
    fn drop(&mut self) {
        if self.track_ref {
            unsafe {
                if let Some(release) = (*self.ptr).base.release {
                    release(&mut (*self.ptr).base);
                }
            }
        }
    }
}
impl CommandLine {
    pub(crate) fn from(ptr: *mut cef_command_line_t, track_ref: bool) -> CommandLine {
        if track_ref {
            unsafe {
                // Let CEF know we are passing it around
                if let Some(add_ref) = (*ptr).base.add_ref {
                    add_ref(&mut (*ptr).base);
                }
            }
        }
        CommandLine { ptr, track_ref }
    }

    pub fn is_valid(&self) -> bool {
        unsafe {
            if let Some(is_valid) = (*self.ptr).is_valid {
                is_valid(&mut (*self.ptr)) > 0
            } else {
                false
            }
        }
    }

    pub fn is_read_only(&self) -> bool {
        unsafe {
            if let Some(is_read_only) = (*self.ptr).is_read_only {
                is_read_only(&mut (*self.ptr)) > 0
            } else {
                false
            }
        }
    }

    // TODO - fill out remaining
}
