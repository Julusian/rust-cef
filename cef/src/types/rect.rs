use cef_sys::cef_rect_t;

#[derive(Clone, Debug)]
pub struct CefRect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}
impl CefRect {
    pub(crate) fn from(raw: &cef_rect_t) -> CefRect {
        CefRect {
            x: raw.x,
            y: raw.y,
            width: raw.width,
            height: raw.height,
        }
    }
}
impl Default for CefRect {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }
}
