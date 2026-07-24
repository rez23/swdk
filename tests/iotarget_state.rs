use swdk::val::WdfIoTargetState;

#[test]
fn state_conversion_started() {
    let raw = u32::from(WdfIoTargetState::Started);

    assert_eq!(
        WdfIoTargetState::from(raw),
        WdfIoTargetState::Started
    );
}

#[test]
fn state_conversion_closed() {
    let raw = u32::from(WdfIoTargetState::Closed);

    assert_eq!(
        WdfIoTargetState::from(raw),
        WdfIoTargetState::Closed
    );
}

#[test]
fn unknown_state_is_preserved() {
    assert_eq!(
        WdfIoTargetState::from(999),
        WdfIoTargetState::Unknown(999)
    );
}