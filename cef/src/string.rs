use cef_sys::cef_string_utf16_t;
use std::ptr::{null, null_mut};
use widestring::U16CString;

pub(crate) type CefString = CefStringUTF16;
pub(crate) struct CefStringUTF16 {
    str: U16CString,
}
impl CefStringUTF16 {
    pub fn from_str(s: &str) -> Self {
        Self {
            str: U16CString::from_str(s).unwrap(), // TODO error safety
        }
    }
    pub fn from_cef(ptr: *const cef_string_utf16_t) -> CefStringUTF16 {
        if ptr == null() {
            CefStringUTF16 {
                str: U16CString::from_str("").unwrap(),
            }
        } else {
            // It's a pointer, so CEF retains ownership and will call the dtor

            unsafe {
                CefStringUTF16 {
                    str: U16CString::from_ptr((*ptr).str, (*ptr).length).unwrap(), // TODO error safety
                }
            }
        }
    }
    pub fn into_cef(self) -> cef_string_utf16_t {
        extern "C" fn free_str(ptr: *mut u16) {
            if ptr == null_mut() {
                return;
            }
            // TODO - what about the cef_string_utf16_t wrapper?
            unsafe {
                // Restore and drop
                U16CString::from_raw(ptr);
            }
        }

        cef_string_utf16_t {
            length: self.str.len(),
            str: self.str.into_raw(),
            dtor: Some(free_str),
        }
    }
}
impl ToString for CefStringUTF16 {
    fn to_string(&self) -> String {
        self.str.to_string_lossy()
    }
}
