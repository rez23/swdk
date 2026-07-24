use swdk::ctx::operations;

#[test]
fn context_descriptor_has_valid_size() {
    let value = 123u32;

    let desc =
        unsafe { operations::build_for_data_type(&value, "u32") };

    assert!(desc.Size > 0);
    assert_eq!(desc.ContextSize, core::mem::size_of::<u32>());
}

#[test]
fn context_name_ptr_is_not_null() {
    let value = 42u32;

    let desc =
        unsafe { operations::build_for_data_type(&value, "u32") };

    assert!(!desc.ContextName.is_null());
}

#[test]
fn unique_type_ptr_is_not_null() {
    let value = 42u32;

    let desc =
        unsafe { operations::build_for_data_type(&value, "u32") };

    assert!(!desc.UniqueType.is_null());
}