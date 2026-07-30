use swdk::rt::utils::{from_option_to_mut_ptr, from_option_to_ptr, parse_hex_u16};

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
#[test]
fn parse_hex_overflow_returns_none() {
    assert_eq!(parse_hex_u16("10000"), None);
}

#[test]
fn parse_empty_hex_returns_zero() {
    assert_eq!(parse_hex_u16(""), Some(0));
}

#[test]
fn option_to_ptr_some_is_non_null() {
    let value = 123u32;

    let ptr = from_option_to_ptr(Some(&value));

    assert!(!ptr.is_null());
    assert_eq!(unsafe { *ptr }, 123);
}

#[test]
fn option_to_mut_ptr_some_is_non_null() {
    let mut value = 123u32;

    let ptr = from_option_to_mut_ptr(Some(&mut value));

    assert!(!ptr.is_null());

    unsafe {
        *ptr = 456;
    }

    assert_eq!(value, 456);
}