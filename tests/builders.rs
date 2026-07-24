use swdk::bd::{
    WdfDriverConf,
    WdfDriverSetup,
    WdfObjAttrs
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