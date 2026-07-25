use swdk::vals::WdfIoTargetState;

#[test]
fn started_roundtrip() {
    let raw = u32::from(WdfIoTargetState::Started);

    let state = WdfIoTargetState::from(raw);

    assert_eq!(state, WdfIoTargetState::Started);
}

#[test]
fn stopped_roundtrip() {
    let raw = u32::from(WdfIoTargetState::Stopped);

    let state = WdfIoTargetState::from(raw);

    assert_eq!(state, WdfIoTargetState::Stopped);
}

#[test]
fn unknown_is_preserved() {
    let value = 9999u32;

    let state = WdfIoTargetState::from(value);

    assert_eq!(state, WdfIoTargetState::Unknown(9999));
}