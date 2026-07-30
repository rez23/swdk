use swdk::bd::{
    WdfDriverConf, WdfDriverSetup, WdfIoQueueConfig,
    WdfObjAttrs,
};
use swdk::ctx::WdfCtxNoneDesc;
use swdk::op::AsBuilder;

#[test]
fn obj_attrs_default_is_valid() {
    let attrs = WdfObjAttrs::<WdfCtxNoneDesc>::default();

    let desc = attrs.build();

    assert!(desc.Size > 0);
}

#[test]
fn driver_conf_build_is_valid() {
    let conf = WdfDriverConf {
        setup: WdfDriverSetup::default(),
        registry_path: core::ptr::null(),
    };

    let desc = conf.build();

    assert!(desc.Size > 0);
}

#[test]
fn obj_attrs_without_context_has_null_context_type_info() {
    let attrs = WdfObjAttrs::<WdfCtxNoneDesc>::default();
    let desc = attrs.build();

    assert!(desc.ContextTypeInfo.is_null());
}

unsafe extern "C" fn cleanup(_: WDFOBJECT) {}
unsafe extern "C" fn destroy(_: WDFOBJECT) {}

#[test]
fn obj_attrs_propagates_cleanup_and_destroy_callbacks() {
    let attrs = WdfObjAttrs::<WdfCtxNoneDesc>::default()
        .with_on_cleanup(Some(cleanup))
        .with_on_destroy(Some(destroy));

    let desc = attrs.build();

    assert!(desc.EvtCleanupCallback.is_some());
    assert!(desc.EvtDestroyCallback.is_some());
}

#[test]
fn io_queue_config_sets_default_queue_flag() {
    let config = WdfIoQueueConfig {
        default_queue: true,
        ..Default::default()
    };

    let desc = config.build();

    assert_eq!(desc.DefaultQueue, 1);
}

#[test]
fn io_queue_config_sets_allow_zero_length_requests_flag() {
    let config = WdfIoQueueConfig {
        allow_zero_length_requests: true,
        ..Default::default()
    };

    let desc = config.build();

    assert_eq!(desc.AllowZeroLengthRequests, 1);
}
use swdk::rt::wdk_sys::{WDFOBJECT, WDFQUEUE,
                        WDFREQUEST,
};

unsafe extern "C" fn on_io_device_control(
    _: WDFQUEUE,
    _: WDFREQUEST,
    _: usize,
    _: usize,
    _: u32,
) {
}

#[test]
fn io_queue_config_sets_device_control_callback() {
    let config = WdfIoQueueConfig {
        on_io_device_control: Some(on_io_device_control),
        ..Default::default()
    };

    let desc = config.build();

    assert!(desc.EvtIoDeviceControl.is_some());
}

unsafe extern "C" fn on_io_internal_device_control(
    _: WDFQUEUE,
    _: WDFREQUEST,
    _: usize,
    _: usize,
    _: u32,
) {
}

#[test]
fn io_queue_config_sets_internal_device_control_callback() {
    let config = WdfIoQueueConfig {
        on_io_internal_device_control: Some(
            on_io_internal_device_control,
        ),
        ..Default::default()
    };

    let desc = config.build();

    assert!(desc.EvtIoInternalDeviceControl.is_some());
}

#[test]
fn io_queue_config_propagates_dispatch_type() {
    use swdk::rt::wdk_sys::_WDF_IO_QUEUE_DISPATCH_TYPE::WdfIoQueueDispatchParallel;

    let config = WdfIoQueueConfig {
        dispatch_type: WdfIoQueueDispatchParallel,
        ..Default::default()
    };

    let desc = config.build();

    assert_eq!(
        desc.DispatchType,
        WdfIoQueueDispatchParallel
    );
}

#[derive(Default)]
struct TestContext {
    value: u32,
}
#[test]
fn device_ctx_none_is_none() {
    let attrs = WdfObjAttrs::<WdfCtxNoneDesc>::default();
    let desc = attrs.build();

    assert!(desc.ContextTypeInfo.is_null());
}
