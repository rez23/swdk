use std::ptr;
use std::ptr::NonNull;
use swdk::Handle;
use swdk::op::{AsRaw, AsRawWithBorrow, AsWdfHandle};
use swdk::rt::wdk_sys::WDFDEVICE;

#[test]
fn can_create_handle() {
    let elem = 123u32;
    let h = Handle::new(unsafe { NonNull::new_unchecked(ptr::from_ref(&elem).cast_mut()) });

    assert_eq!(*h, 123);
}

#[test]
fn as_ref_returns_inner() {
    let elem = 55u32;
    let h = Handle::new(unsafe { NonNull::new_unchecked(ptr::from_ref(&elem).cast_mut()) });

    assert_eq!(*h.as_ref(), 55);
}

#[test]
fn raw_returns_copy() {
    let elem = 88u32;
    let h = Handle::new(unsafe { NonNull::new_unchecked(ptr::from_ref(&elem).cast_mut()) });

    assert_eq!(h.raw(), 88);
}