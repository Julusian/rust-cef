use crate::ptr::wrap_ptr;
use crate::ToCef;
use cef_sys::cef_app_t;
use std::sync::Arc;

pub trait App {
    // TODO - proper types
    fn on_before_command_line_processing(&self) {}
    fn on_register_custom_schemes(&self) {}
    fn get_resource_bundle_handler(&self) {}
    fn get_browser_process_handler(&self) {}
    fn get_render_process_handler(&self) {}
}

//struct SomeWrapper<TApp: App> {
//    raw: Arc<TApp>,
//} // TODO

impl<TApp: App> ToCef<cef_app_t> for Arc<TApp> {
    fn to_cef(&self) -> *mut cef_app_t {
        wrap_ptr(|a| cef_app_t {
            base: a,
            on_before_command_line_processing: None,
            on_register_custom_schemes: None,
            get_resource_bundle_handler: None,
            get_browser_process_handler: None,
            get_render_process_handler: None,
        })

        //        let wrapper = Box::new(Arc::new(SomeWrapper { raw: self.clone() }));
        //
        //        let bas_ref = cef_base_ref_counted_t {
        //            size: 0,
        //            add_ref: None,
        //            release: None,
        //            has_one_ref: None,
        //            has_at_least_one_ref: None,
        //        };
        //
        //        unimplemented!()
    }
}
