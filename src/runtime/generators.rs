#[cfg(feature = "test-runtime")]
pub struct FakeResponse {
    Size: u32,
    CollectionType: u32,
    Usage: u32,
    UsagePage: u32,
    ParentId: u32,
    NumberOfChildren: u32,
    NumberOfDescendants: u32,
}
#[cfg(feature = "test-runtime")]
macro_rules! userapi_ioctl_response {
    () => {{
        //use wdk_sys::HID_COLLECTION_INFORMATION;
        $crate::generators:MY_RESPONSEE {
            Size: 10,
            CollectionType: 20,
            Usage: 33,
            UsagePage: 44,
            ParentId: 1,
            NumberOfChildren: 1000,
            NumberOfDescendants: 9,
        }
    }};
}
