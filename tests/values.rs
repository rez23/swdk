use swdk::vals::WdfIoTargetState;

#[test]
fn started_roundtrip() {
    let raw = i32::from(WdfIoTargetState::Started);

    let state = WdfIoTargetState::from(raw);

    assert_eq!(state, WdfIoTargetState::Started);
}

#[test]
fn stopped_roundtrip() {
    let raw = i32::from(WdfIoTargetState::Stopped);

    let state = WdfIoTargetState::from(raw);

    assert_eq!(state, WdfIoTargetState::Stopped);
}

#[test]
fn unknown_is_preserved() {
    let value = 9999i32;

    let state = WdfIoTargetState::from(value);

    assert_eq!(state, WdfIoTargetState::Unknown(9999));
}

#[test]
fn state_conversion_stopped() {
    let raw = i32::from(WdfIoTargetState::Stopped);

    assert_eq!(
        WdfIoTargetState::from(raw),
        WdfIoTargetState::Stopped
    );
}

#[test]
fn state_conversion_deleted() {
    let raw = i32::from(WdfIoTargetState::Deleted);

    assert_eq!(
        WdfIoTargetState::from(raw),
        WdfIoTargetState::Deleted
    );
}

#[test]
fn state_conversion_purged() {
    let raw = i32::from(WdfIoTargetState::Purged);

    assert_eq!(
        WdfIoTargetState::from(raw),
        WdfIoTargetState::Purged
    );
}

#[test]
fn state_conversion_closed_for_query_remove() {
    let raw = i32::from(WdfIoTargetState::ClosedForQueryRemove);

    assert_eq!(
        WdfIoTargetState::from(raw),
        WdfIoTargetState::ClosedForQueryRemove
    );
}