use cef_sys::cef_string_utf16_t;
use std::ptr::null_mut;
use widestring::U16CString;

pub type CefString = CefStringUTF16;
pub struct CefStringUTF16 {
    str: U16CString,
}
impl CefStringUTF16 {
    pub fn from_str(str: &str) -> Self {
        Self {
            str: U16CString::from_str(str).unwrap(), // TODO error safety
        }
    }
    pub(crate) fn into_cef(self) -> cef_string_utf16_t {
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
