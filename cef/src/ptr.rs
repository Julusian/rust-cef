use cef_sys::{_cef_base_ref_counted_t, cef_base_ref_counted_t};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicUsize, Ordering};

pub(crate) unsafe trait WrapperFor<T> {}

// This relies on the c storage structure to allow casting a *_cef_base_ref_counted_t into a pointer of this type which starts with _cef_base_ref_counted_t
#[repr(C)]
pub(crate) struct BaseRefCountedExt<TCef, TWrapper> {
    v: TWrapper,
    count: AtomicUsize,
    phantom: PhantomData<TCef>,
}
impl<TCef, TWrapper> Deref for BaseRefCountedExt<TCef, TWrapper> {
    type Target = TWrapper;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}
impl<TCef, TWrapper> DerefMut for BaseRefCountedExt<TCef, TWrapper> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.v
    }
}
impl<TCef, TWrapper: WrapperFor<TCef>> BaseRefCountedExt<TCef, TWrapper> {
    fn wrap_ptr<F>(wrapper: F) -> *mut TCef
    where
        F: FnOnce(cef_base_ref_counted_t) -> TWrapper,
    {
        let base = BaseRefCountedExt::<TCef, TWrapper> {
            v: wrapper(_cef_base_ref_counted_t {
                size: std::mem::size_of::<Self>(),
                add_ref: Some(Self::add_ref),
                release: Some(Self::release),
                has_one_ref: Some(Self::has_one_ref),
                has_at_least_one_ref: Some(Self::has_at_least_one_ref),
            }),
            count: AtomicUsize::new(1),
            phantom: PhantomData,
        };
        unsafe { std::mem::transmute(Box::into_raw(Box::new(base))) }
    }

    fn from_ptr<'a>(ptr: *mut cef_base_ref_counted_t) -> &'a mut BaseRefCountedExt<TCef, TWrapper> {
        unsafe { std::mem::transmute(ptr) }
    }
    extern "C" fn add_ref(ptr: *mut cef_base_ref_counted_t) {
        let base = Self::from_ptr(ptr);
        base.count.fetch_add(1, Ordering::Relaxed);
    }
    extern "C" fn release(ptr: *mut cef_base_ref_counted_t) -> i32 {
        let base = Self::from_ptr(ptr);
        let old_count = base.count.fetch_sub(1, Ordering::Release);
        if old_count == 1 {
            // reclaim and release
            unsafe { Box::from_raw(base) };

            1 // true
        } else {
            0 // false
        }
    }
    extern "C" fn has_one_ref(ptr: *mut cef_base_ref_counted_t) -> i32 {
        let base = Self::from_ptr(ptr);
        if base.count.load(Ordering::SeqCst) == 1 {
            1 // true
        } else {
            0 // false
        }
    }
    extern "C" fn has_at_least_one_ref(ptr: *mut cef_base_ref_counted_t) -> i32 {
        let base = Self::from_ptr(ptr);
        if base.count.load(Ordering::SeqCst) >= 1 {
            1 // true
        } else {
            0 // false
        }
    }
}

pub(crate) fn wrap_ptr<TCef, TWrapper, F>(wrapper: F) -> *mut TCef
where
    F: FnOnce(cef_base_ref_counted_t) -> TWrapper,
    TWrapper: WrapperFor<TCef>,
{
    BaseRefCountedExt::wrap_ptr(wrapper)
}
