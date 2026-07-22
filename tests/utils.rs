use swdk::rt::utils::{
    from_option_to_ptr,
    parse_hex_u16,
};

#[test]
fn parse_hex_lowercase() {
    assert_eq!(parse_hex_u16("ff"), Some(255));
}

#[test]
fn parse_hex_uppercase() {
    assert_eq!(parse_hex_u16("FF"), Some(255));
}

#[test]
fn parse_invalid_hex() {
    assert_eq!(parse_hex_u16("XYZ"), None);
}

#[test]
fn option_to_ptr_null() {
    let ptr: *const u32 =
        from_option_to_ptr::<u32>(None);

    assert!(ptr.is_null());
}