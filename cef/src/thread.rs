use crate::ptr::{wrap_ptr, BaseRefCountedExt, WrapperFor};
use cef_sys::{cef_currently_on, cef_post_task, cef_task_t};

#[derive(Copy, Clone, Debug)]
pub enum ThreadId {
    UI = cef_sys::cef_thread_id_t_TID_UI as isize,
    FileBackground = cef_sys::cef_thread_id_t_TID_FILE_BACKGROUND as isize,
    // File = cef_sys::cef_thread_id_t_TID_FILE as isize,
    FileUserVisible = cef_sys::cef_thread_id_t_TID_FILE_USER_VISIBLE as isize,
    FileUserBlocking = cef_sys::cef_thread_id_t_TID_FILE_USER_BLOCKING as isize,
    ProcessLauncher = cef_sys::cef_thread_id_t_TID_PROCESS_LAUNCHER as isize,
    IO = cef_sys::cef_thread_id_t_TID_IO as isize,
    Renderer = cef_sys::cef_thread_id_t_TID_RENDERER as isize,
}

pub fn currently_on(id: ThreadId) -> bool {
    unsafe { cef_currently_on(id as u32) > 0 }
}

pub fn post_task<F: FnOnce() -> ()>(id: ThreadId, func: F) -> Result<(), bool> {
    if currently_on(id) {
        // TODO - execute now

    }

    let task = wrap_ptr(move |base| TaskWrapper {
        base: cef_task_t {
            base,
            execute: Some(TaskWrapper::<F>::execute),
        },
        func: Some(func),
    });

    // TODO
    let ok = unsafe { cef_post_task(id as u32, task) };
    if ok > 0 {
        Ok(())
    } else {
        Err(false)
    }
}

pub struct TaskWrapper<F: FnOnce() -> ()> {
    base: cef_task_t,
    func: Option<F>,
}
unsafe impl<F: FnOnce() -> ()> WrapperFor<cef_task_t> for TaskWrapper<F> {}
impl<F: FnOnce() -> ()> TaskWrapper<F> {
    fn from_ptr<'a>(ptr: *mut cef_task_t) -> &'a mut BaseRefCountedExt<cef_task_t, TaskWrapper<F>> {
        unsafe { std::mem::transmute(ptr) }
    }

    extern "C" fn execute(task: *mut cef_task_t) {
        let task = Self::from_ptr(task);

        if let Some(func) = task.func.take() {
            (func)();
        }
    }
}
