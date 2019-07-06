mod display_handler;
mod life_span_handler;
mod render_handler;

pub use display_handler::*;
pub use life_span_handler::*;
pub use render_handler::*;

use crate::ptr::{wrap_ptr, BaseRefCountedExt, WrapperFor};
use crate::ToCef;
use cef_sys::{
    cef_audio_handler_t, cef_browser_t, cef_client_t, cef_context_menu_handler_t,
    cef_dialog_handler_t, cef_display_handler_t, cef_download_handler_t, cef_drag_handler_t,
    cef_find_handler_t, cef_focus_handler_t, cef_frame_t, cef_jsdialog_handler_t,
    cef_keyboard_handler_t, cef_life_span_handler_t, cef_load_handler_t, cef_process_id_t,
    cef_process_message_t, cef_render_handler_t, cef_request_handler_t,
};
use std::ptr::null_mut;
use std::sync::Arc;

pub trait AudioHandler {
    //
}
impl AudioHandler for () {}

pub trait Client {
    type OutAudioHandler: AudioHandler;
    type OutDisplayHandler: DisplayHandler;
    type OutLifeSpanHandler: LifeSpanHandler;
    type OutRenderHandler: RenderHandler;

    // TODO - fill out
    fn get_audio_handler(&self) -> Option<Arc<Self::OutAudioHandler>> {
        None
    }
    fn get_display_handler(&self) -> Option<Arc<Self::OutDisplayHandler>> {
        None
    }
    fn get_life_span_handler(&self) -> Option<Arc<Self::OutLifeSpanHandler>> {
        None
    }
    fn get_render_handler(&self) -> Option<Arc<Self::OutRenderHandler>> {
        None
    }
}

struct ClientWrapper<T: Client> {
    _base: cef_client_t,
    internal: Arc<T>,
}
unsafe impl<T: Client> WrapperFor<cef_client_t> for ClientWrapper<T> {}
impl<T: Client> ClientWrapper<T> {
    fn from_ptr<'a>(
        ptr: *mut cef_client_t,
    ) -> &'a mut BaseRefCountedExt<cef_client_t, ClientWrapper<T>> {
        unsafe { &mut *(ptr as *mut _) }
    }

    extern "C" fn get_audio_handler(_client: *mut cef_client_t) -> *mut cef_audio_handler_t {
        null_mut()
    }

    extern "C" fn get_context_menu_handler(
        _client: *mut cef_client_t,
    ) -> *mut cef_context_menu_handler_t {
        null_mut()
    }

    extern "C" fn get_dialog_handler(_client: *mut cef_client_t) -> *mut cef_dialog_handler_t {
        null_mut()
    }

    extern "C" fn get_display_handler(client: *mut cef_client_t) -> *mut cef_display_handler_t {
        let client = Self::from_ptr(client);
        if let Some(handler) = client.internal.get_display_handler() {
            handler.to_cef()
        } else {
            null_mut()
        }
    }

    extern "C" fn get_download_handler(_client: *mut cef_client_t) -> *mut cef_download_handler_t {
        null_mut()
    }

    extern "C" fn get_drag_handler(_client: *mut cef_client_t) -> *mut cef_drag_handler_t {
        null_mut()
    }

    extern "C" fn get_find_handler(_client: *mut cef_client_t) -> *mut cef_find_handler_t {
        null_mut()
    }

    extern "C" fn get_focus_handler(_client: *mut cef_client_t) -> *mut cef_focus_handler_t {
        null_mut()
    }

    extern "C" fn get_jsdialog_handler(_client: *mut cef_client_t) -> *mut cef_jsdialog_handler_t {
        null_mut()
    }

    extern "C" fn get_keyboard_handler(_client: *mut cef_client_t) -> *mut cef_keyboard_handler_t {
        null_mut()
    }

    extern "C" fn get_life_span_handler(client: *mut cef_client_t) -> *mut cef_life_span_handler_t {
        let client = Self::from_ptr(client);
        if let Some(handler) = client.internal.get_life_span_handler() {
            handler.to_cef()
        } else {
            null_mut()
        }
    }

    extern "C" fn get_load_handler(_client: *mut cef_client_t) -> *mut cef_load_handler_t {
        null_mut()
    }

    extern "C" fn get_render_handler(client: *mut cef_client_t) -> *mut cef_render_handler_t {
        let client = Self::from_ptr(client);
        if let Some(handler) = client.internal.get_render_handler() {
            handler.to_cef()
        } else {
            null_mut()
        }
    }

    extern "C" fn get_request_handler(_client: *mut cef_client_t) -> *mut cef_request_handler_t {
        null_mut()
    }

    extern "C" fn on_process_message_received(
        _client: *mut cef_client_t,
        _browser: *mut cef_browser_t,
        _frame: *mut cef_frame_t,
        _source_process: cef_process_id_t,
        _message: *mut cef_process_message_t,
    ) -> ::std::os::raw::c_int {
        0
    }
}
impl<T: Client> ToCef<cef_client_t> for Arc<T> {
    fn to_cef(&self) -> *mut cef_client_t {
        wrap_ptr(|base| ClientWrapper {
            _base: cef_client_t {
                base,
                get_audio_handler: Some(ClientWrapper::<T>::get_audio_handler),
                get_context_menu_handler: Some(ClientWrapper::<T>::get_context_menu_handler),
                get_dialog_handler: Some(ClientWrapper::<T>::get_dialog_handler),
                get_display_handler: Some(ClientWrapper::<T>::get_display_handler),
                get_download_handler: Some(ClientWrapper::<T>::get_download_handler),
                get_drag_handler: Some(ClientWrapper::<T>::get_drag_handler),
                get_find_handler: Some(ClientWrapper::<T>::get_find_handler),
                get_focus_handler: Some(ClientWrapper::<T>::get_focus_handler),
                get_jsdialog_handler: Some(ClientWrapper::<T>::get_jsdialog_handler),
                get_keyboard_handler: Some(ClientWrapper::<T>::get_keyboard_handler),
                get_life_span_handler: Some(ClientWrapper::<T>::get_life_span_handler),
                get_load_handler: Some(ClientWrapper::<T>::get_load_handler),
                get_render_handler: Some(ClientWrapper::<T>::get_render_handler),
                get_request_handler: Some(ClientWrapper::<T>::get_request_handler),
                on_process_message_received: Some(ClientWrapper::<T>::on_process_message_received),
            },
            internal: self.clone(),
        })
    }
}
