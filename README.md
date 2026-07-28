# Rust Safe Windows Driver Kit
> ###### This is a highly experimental project in a very early stage of development.
> ###### Contributions are welcome, but if you submit PR may be some time for the approval

**swdk** is middleware for the Windows Driver Framework (WDF) designed to provide a safe, idiomatic, and expressive 
way to write Windows kernel drivers in Rust, with a current focus on KMDF.

**swdk** is not a wrapper around a specific WDF runtime or some sorta of utility crate for WDF.  
Rather than hiding WDF callbacks behind a higher-level, object-oriented abstraction, swdk defines a compact set of Rust 
rules that describe how the Rust compiler should handle WDF operations, data types, resources, etc.

In this sense, swdk does not aim to cover every WDF feature. Instead, it provides a Rust-to-WDF meta-binding that, 
as a result, permits you to write Windows drivers as a set of rules applied directly to the raw data types exposed by WDF.


# A brief example


```rust
use swdk::bd::{WdfDriverConf, WdfDriverSetup, WdfObjAttrs};
use swdk::ctx::WdfCtxNoneDesc;
use swdk::op::{AsRaw, AsWdfOwned, AsWdfOwner};
use swdk::rt::wdk_sys::{
    NTSTATUS, PCUNICODE_STRING, PDRIVER_OBJECT, PWDFDEVICE_INIT,
    STATUS_SUCCESS, WDFDEVICE, WDFDRIVER,
};
use swdk::{Handle, if_nterror_return_ntstatus, info};

#[unsafe(export_name = "DriverEntry")]
pub unsafe extern "system" fn driver_entry(
    driver_obj: PDRIVER_OBJECT,
    registry_path: PCUNICODE_STRING,
) -> NTSTATUS {
    if_nterror_return_ntstatus!(
        Handle::<WDFDRIVER>::from_owned_with_attrs(
            driver_obj,
            WdfDriverConf {
                setup: WdfDriverSetup {
                    on_device_add: Some(on_device_add),
                    ..Default::default()
                },
                registry_path,
            },
            Some(WdfObjAttrs::<WdfCtxNoneDesc>::default()),
        )
    );

    STATUS_SUCCESS
}

unsafe extern "C" fn on_device_add(
    _driver: WDFDRIVER,
    device_init: PWDFDEVICE_INIT,
) -> NTSTATUS {
    if_nterror_return_ntstatus!(
        Handle::<WDFDEVICE>::from_owned(
            Handle::new(&device_init).raw(),
            Some(WdfObjAttrs::<WdfCtxNoneDesc>::default()),
        )
    );

    info!("Device added and loaded!");
    STATUS_SUCCESS
}
```

Although minimal, this example creates both the KMDF driver object and a device object. It demonstrates several of the framework's core concepts:

- `Handle<T>` as the primary abstraction
- typed WDF object creation
- driver configuration through data structures
- automatic descriptor initialization
- `NTSTATUS`-oriented error handling

For a more complete example check the [gamepad-info](https://github.com/rez23/gamepad-info) official driver example crate
for *swdk*
# Getting started

swdk is built on Microsoft's **windows-drivers-rs** ecosystem.

Building a driver requires WDF bindings generated from the official C headers. Microsoft's `wdk-build` crate configures this process, while the `cargo-wdk` tool provides the `cargo wdk` command. See [windows-drivers-rs](https://github.com/microsoft/windows-drivers-rs) for installation requirements and further details.

A typical driver project uses:

```toml
[dependencies]
swdk = "..."
```

The optional `swdk-macros` crate provides custom derives and attributes, such as `#[derive(CtxDescriptor)]`:

```toml
[dependencies]
swdk = "..."
swdk-macros = "..."
```

## Build your driver

Because swdk is built on WDF, it uses the bindings exposed by Microsoft's `wdk` crates. Your driver crate therefore needs a custom `build.rs`, driver-model metadata in `Cargo.toml`, and a working LLVM/Clang installation.

Add `wdk-build` as a build dependency:

```toml
[build-dependencies]
wdk-build = "..."
```

Create `build.rs`:

```rust
fn main() -> Result<(), wdk_build::ConfigError> {
    wdk_build::configure_wdf_binary_build();
    Ok(())
}
```

After installing the `cargo-wdk` command-line tool, build the driver with:

```bash
cargo wdk build
```



# Goals

This crate aims to give to Rust Windows kernel-driver developers a safe and ergonomic bridge between Rust 
and the Windows Driver Framework runtime. 

SWDK was designed trying to follow the next principals:
- Preserve the original KMDF architecture
- Apply Rust ownership semantics to WDF resources
- Reduce the boilerplate required for native WDF descriptors
- Improve `NTSTATUS` diagnostics and debugging
- Provide a runtime abstraction suitable for testing
- Let driver authors choose when to work with raw WDF types

Probably the primary objective of *swdk* is
> If you are already familiar with the Windows Driver Framework, swdk should feel familiar too.



# Philosophy

Two key ideas lie at the heart of swdk:

- Every WDF resource can be represented by a `Handle`.
- A `Handle` knows how to present its underlying object or resource to WDF for operations such as allocation, deallocation, and initialization.

swdk implements these ideas through a deliberately simple type:

```rust
Handle<T>
```

`Handle<T>` packages a WDF kernel object or resource in a Rust type that can expose the corresponding raw WDF value when required.



### Data-oriented

Many frameworks or wrappers built around KMDF attempt to encapsulate devices and framework resources behind large object-oriented abstractions. swdk intentionally avoids this approach.

Instead of building an object hierarchy, swdk describes the following as composable pieces of data:

- ownership relationships
- object contexts
- descriptors
- object attributes
- framework callbacks
- I/O targets
- device capabilities

Traits then transform these descriptions into native KMDF structures and runtime objects.

For example, the following C code configures device initialization:

```c
WdfFdoInitSetFilter(DeviceInit);

WDF_PNPPOWER_EVENT_CALLBACKS callbacks;
WDF_PNPPOWER_EVENT_CALLBACKS_INIT(&callbacks);

callbacks.EvtDeviceD0Entry = OnD0Entry;

WdfDeviceInitSetPnpPowerEventCallbacks(
    DeviceInit,
    &callbacks
);
```

With swdk, the same intent can be expressed as a composition of behaviors:

```rust
fn initialize_device_init(device_init: PWDFDEVICE_INIT) -> PWDFDEVICE_INIT {
    Handle::new(&device_init)
        .with_filter()
        .with_pnp_setup(WdfDevicePnpPowerSetup {
            on_device_d0_entry: Some(on_device_d0_entry),
            ..Default::default()
        })
        .raw()
}

fn on_device_d0_entry(...) {
    ...
}
```

The result remains recognizably KMDF, while shifting the focus from imperative descriptor manipulation to declarative composition.



### Zero-cost

swdk relies heavily on Rust generics, trait implementations, and static dispatch. As a result, most abstractions provided by the framework exist only at compile time.

Types such as:

```rust
Handle<WDFDEVICE>
Handle<WDFIOTARGET>

WdfObjAttrs<DeviceContext>

IoCtlRequest<MyRequest>
IoCtlResponse<MyResponse>
```

carry additional semantic information for the compiler while introducing little to no runtime overhead.

The framework avoids dynamic dispatch (`dyn Trait`) and delegates specialization to Rust's compile-time type-resolution system through generics and traits. This keeps the APIs expressive while producing code close to an equivalent hand-written KMDF driver.

In practice, most abstractions compile down to:

- direct function calls
- direct structure initialization
- direct pointer manipulation
- direct KMDF callback invocations

without introducing additional runtime layers.
# Additional resources

For more information about the Windows Driver Framework, KMDF, and the official Rust WDK project, see:

- [windows-drivers-rs](https://github.com/microsoft/windows-drivers-rs)
- [swdk official repository](https://github.com/rez23/swdk)
- [Get started with WDF](https://learn.microsoft.com/windows-hardware/drivers/wdf/)
- [Windows Driver Kit API reference](https://learn.microsoft.com/windows-hardware/drivers/ddi/)
