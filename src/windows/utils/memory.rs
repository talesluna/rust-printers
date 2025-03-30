use libc::c_ulong;
use std::alloc::{alloc, dealloc, Layout};

fn _ptr_layout<T>(size: usize) -> Layout {
    return unsafe { Layout::from_size_align_unchecked(size, align_of::<T>()) };
}

pub fn alloc_s<T>(size: c_ulong) -> *mut T {
    return unsafe { alloc(_ptr_layout::<T>(size as usize)) as *mut T };
}

pub fn dealloc_s<T>(ptr: *const T) {
    let layout = _ptr_layout::<T>(size_of::<T>());
    unsafe { dealloc(ptr as *mut u8, layout) };
}