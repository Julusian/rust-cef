use crate::ptr::RefCounterGuard;
use crate::types::string::CefString;
use crate::Browser;
use cef_sys::{
    cef_domvisitor_t, cef_frame_t, cef_process_id_t, cef_process_message_t, cef_request_t,
    cef_string_visitor_t, cef_v8context_t,
};
use std::ptr::null_mut;

pub struct Frame {
    ptr: RefCounterGuard<cef_frame_t>,
}
impl Frame {
    pub(crate) fn from(ptr: *mut cef_frame_t, track_ref: bool) -> Self {
        unsafe {
            Self {
                ptr: RefCounterGuard::from(&mut (*ptr).base, ptr, track_ref),
            }
        }
    }

    pub fn is_valid(&self) -> bool {
        if let Some(func) = self.ptr.as_ref().is_valid {
            unsafe { func(self.ptr.get()) > 0 }
        } else {
            false
        }
    }

    pub fn undo(&self) {
        if let Some(func) = self.ptr.as_ref().undo {
            unsafe { func(self.ptr.get()) }
        }
    }

    pub fn redo(&self) {
        if let Some(func) = self.ptr.as_ref().redo {
            unsafe { func(self.ptr.get()) }
        }
    }

    pub fn cut(&self) {
        if let Some(func) = self.ptr.as_ref().cut {
            unsafe { func(self.ptr.get()) }
        }
    }

    pub fn copy(&self) {
        if let Some(func) = self.ptr.as_ref().copy {
            unsafe { func(self.ptr.get()) }
        }
    }

    pub fn paste(&self) {
        if let Some(func) = self.ptr.as_ref().paste {
            unsafe { func(self.ptr.get()) }
        }
    }

    pub fn del(&self) {
        if let Some(func) = self.ptr.as_ref().del {
            unsafe { func(self.ptr.get()) }
        }
    }

    pub fn select_all(&self) {
        if let Some(func) = self.ptr.as_ref().select_all {
            unsafe { func(self.ptr.get()) }
        }
    }

    pub fn view_source(&self) {
        if let Some(func) = self.ptr.as_ref().view_source {
            unsafe { func(self.ptr.get()) }
        }
    }

    pub fn get_source(&self, _visitor: *mut cef_string_visitor_t) {
        // TODO
    }

    pub fn get_text(&self, _visitor: *mut cef_string_visitor_t) {
        // TODO
    }

    pub fn load_request(&self, _request: *mut cef_request_t) {
        // TODO
    }

    pub fn load_url(&self, url: &str) {
        if let Some(func) = self.ptr.as_ref().load_url {
            let url = CefString::from_str(url);
            unsafe { func(self.ptr.get(), &url.into_cef()) }
        }
    }

    pub fn load_string(&self, string_val: &str, url: &str) {
        if let Some(func) = self.ptr.as_ref().load_string {
            let string_val = CefString::from_str(string_val);
            let url = CefString::from_str(url);
            unsafe { func(self.ptr.get(), &string_val.into_cef(), &url.into_cef()) }
        }
    }

    pub fn execute_javascript(&self, code: &str, script_url: &str, start_line: i32) {
        if let Some(func) = self.ptr.as_ref().execute_java_script {
            let code = CefString::from_str(code);
            let script_url = CefString::from_str(script_url);
            unsafe {
                func(
                    self.ptr.get(),
                    &code.into_cef(),
                    &script_url.into_cef(),
                    start_line,
                )
            }
        }
    }

    pub fn is_main(&self) -> bool {
        if let Some(func) = self.ptr.as_ref().is_main {
            unsafe { func(self.ptr.get()) > 0 }
        } else {
            false
        }
    }

    pub fn is_focused(&self) -> bool {
        if let Some(func) = self.ptr.as_ref().is_focused {
            unsafe { func(self.ptr.get()) > 0 }
        } else {
            false
        }
    }

    pub fn get_name(&self) -> String {
        if let Some(func) = self.ptr.as_ref().get_name {
            CefString::from_userfree_cef(unsafe { func(self.ptr.get()) }).to_string()
        } else {
            "".to_string()
        }
    }

    pub fn get_identifier(&self) -> i64 {
        if let Some(func) = self.ptr.as_ref().get_identifier {
            unsafe { func(self.ptr.get()) }
        } else {
            0
        }
    }

    pub fn get_parent(&self) -> Option<Self> {
        if let Some(func) = self.ptr.as_ref().get_parent {
            unsafe { Some(Self::from(func(self.ptr.get()), true)) }
        } else {
            None
        }
    }

    pub fn get_url(&self) -> String {
        if let Some(func) = self.ptr.as_ref().get_url {
            unsafe {
                let ptr = func(self.ptr.get());
                let res = CefString::from_cef(ptr);

                cef_sys::cef_string_userfree_utf16_free(ptr);

                res.to_string()
            }
        } else {
            "".to_string()
        }
    }

    pub fn get_browser(&self) -> Option<Browser> {
        if let Some(func) = self.ptr.as_ref().get_browser {
            unsafe { Some(Browser::from(func(self.ptr.get()), true)) }
        } else {
            None
        }
    }

    pub fn get_v8context(&self) -> *mut cef_v8context_t {
        if let Some(func) = self.ptr.as_ref().get_v8context {
            // TODO
            unsafe { func(self.ptr.get()) }
        } else {
            null_mut()
        }
    }

    pub fn visit_dom(&self, _visitor: *mut cef_domvisitor_t) {
        // TODO
    }

    pub fn send_process_message(
        &self,
        _target_process: cef_process_id_t,
        _message: *mut cef_process_message_t,
    ) {
        // TODO
    }
}
