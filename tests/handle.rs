use swdk::Handle;
use swdk::op::{AsRaw, AsRawWithBorrow, AsWdfObject};
use swdk::rt::wdk_sys::WDFDEVICE;

#[test]
fn can_create_handle() {
    let h = Handle::new(123u32);

    assert_eq!(*h, 123);
}

#[test]
fn as_ref_returns_inner() {
    let h = Handle::new(55u32);

    assert_eq!(*h.as_ref(), 55);
}

#[test]
fn raw_returns_copy() {
    let h = Handle::new(88u32);

    assert_eq!(h.raw(), 88);
}

#[test]
fn raw_with_borrow_returns_inner() {
    let value = 99u32;

    let handle = Handle::new(&value);

    assert_eq!(handle.raw_with_borrow(), 99);
}

#[test]
fn device_handle_can_be_seen_as_wdf_object() {
    let raw: WDFDEVICE = core::ptr::dangling_mut();

    let handle = Handle::<WDFDEVICE>::new(raw);
    let object = handle.as_wdf_object();

    assert_eq!(object.cast::<()>(), raw.cast::<()>());
}
