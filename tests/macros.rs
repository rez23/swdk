use swdk::declare_ctx_descriptor;
use swdk::operators::{AsCtxDesc, AsUnique};

#[derive(Default)]
struct MyCtx {
    value: u32,
}
declare_ctx_descriptor!(MyCtx);

#[test]
fn generated_descriptor_has_unique_type() {
    let unique = MyCtx::unique();

    assert!(unique.is_some());
}

#[test]
fn generated_type_name_is_correct() {
    assert_eq!(
        MyCtx::wdf_type_name(),
        Some("MyCtx")
    );
}