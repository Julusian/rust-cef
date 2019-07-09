use crate::ptr::{wrap_ptr, BaseRefCountedExt, WrapperFor};
use crate::{CommandLine, ToCef};
use cef_sys::{
    cef_browser_process_handler_t, cef_command_line_t, cef_list_value_t, cef_print_handler_t,
};
use std::ptr::null_mut;
use std::sync::Arc;

pub trait BrowserProcessHandler {
    fn on_context_initialized(&self) {}
    fn on_before_child_process_launch(&self, _command_line: &CommandLine) {}
    fn on_render_process_thread_created(&self, _extra_info: *mut cef_list_value_t) {}
    //    fn get_print_handler(&self, extra_info) -> *mut cef_print_handler_t {}
    fn on_schedule_message_pump_work(&self, _delay_ms: i64) {}
}
impl BrowserProcessHandler for () {}

struct BrowserProcessHandlerWrapper<T: BrowserProcessHandler> {
    _base: cef_browser_process_handler_t,
    internal: Arc<T>,
}
unsafe impl<T: BrowserProcessHandler> WrapperFor<cef_browser_process_handler_t>
    for BrowserProcessHandlerWrapper<T>
{
}
impl<T: BrowserProcessHandler> BrowserProcessHandlerWrapper<T> {
    fn from_ptr<'a>(
        ptr: *mut cef_browser_process_handler_t,
    ) -> &'a mut BaseRefCountedExt<cef_browser_process_handler_t, BrowserProcessHandlerWrapper<T>>
    {
        unsafe { &mut *(ptr as *mut _) }
    }

    unsafe extern "C" fn on_context_initialized(handler: *mut cef_browser_process_handler_t) {
        let handler = Self::from_ptr(handler);

        handler.internal.on_context_initialized()
    }

    unsafe extern "C" fn on_before_child_process_launch(
        handler: *mut cef_browser_process_handler_t,
        command_line: *mut cef_command_line_t,
    ) {
        let handler = Self::from_ptr(handler);
        let command_line = CommandLine::from(command_line, false);

        handler
            .internal
            .on_before_child_process_launch(&command_line);
    }
    unsafe extern "C" fn on_render_process_thread_created(
        handler: *mut cef_browser_process_handler_t,
        extra_info: *mut cef_list_value_t,
    ) {
        let handler = Self::from_ptr(handler);

        handler
            .internal
            .on_render_process_thread_created(extra_info)
    }

    unsafe extern "C" fn get_print_handler(
        handler: *mut cef_browser_process_handler_t,
    ) -> *mut cef_print_handler_t {
        let handler = Self::from_ptr(handler);

        //        handler.internal.get_print_handler()
        null_mut()
    }

    unsafe extern "C" fn on_schedule_message_pump_work(
        handler: *mut cef_browser_process_handler_t,
        delay_ms: i64,
    ) {
        let handler = Self::from_ptr(handler);

        handler.internal.on_schedule_message_pump_work(delay_ms)
    }
}
impl<T: BrowserProcessHandler> ToCef<cef_browser_process_handler_t> for Arc<T> {
    fn to_cef(&self) -> *mut cef_browser_process_handler_t {
        wrap_ptr(|base| BrowserProcessHandlerWrapper {
            _base: cef_browser_process_handler_t {
                base,
                on_context_initialized: Some(
                    BrowserProcessHandlerWrapper::<T>::on_context_initialized,
                ),
                on_before_child_process_launch: Some(
                    BrowserProcessHandlerWrapper::<T>::on_before_child_process_launch,
                ),
                on_render_process_thread_created: Some(
                    BrowserProcessHandlerWrapper::<T>::on_render_process_thread_created,
                ),
                get_print_handler: Some(BrowserProcessHandlerWrapper::<T>::get_print_handler),
                on_schedule_message_pump_work: Some(
                    BrowserProcessHandlerWrapper::<T>::on_schedule_message_pump_work,
                ),
            },
            internal: self.clone(),
        })
    }
}
