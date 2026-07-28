# SWDK
###### A Safe Windows Driver Kit for the Rust programming language

**SWDK** is Rust wdf middleware for writing safe, idiomatic and rusty Kernel-Mode drivers (currently focused on KMDF) on Windows.

Unlike many wrappers, SWDK does not attempt to hide KMDF behind a completely different programming model.

Instead, it models native KMDF concepts such as:

- Handles
- Ownership
- Contexts
- Descriptors
- I/O Targets
- Object Attributes
- Driver and Device creation

through Rust traits and type-safe abstractions while preserving the original WDF mental model.

---

# Goals

SWDK try to provide to rust Windows kernel driver writers a safe and ergonomic middleware between native Rust and Windows Kenrel Framework runtime operations
The library was built following the next principals:

- Preserve the original KMDF architecture
- Introduce Rust ownership semantics over WDF resources
- Reduce the boilerplate required by native WDF descriptors
- Improve NTSTATUS diagnostics and debugging
- Provide a runtime abstraction suitable for testing
- Let the driver’s choice how to ménage raws WDF types

Probably teh primary objective is:
>*If you are already familiar with the Windows Kernel Framework, SWDK should feel familiar too.*

---

# Philosophy

### Resources are `Handle`

At the heart of *SWDK* lies a surprisingly simple idea:

```rust
Handle<T>
```
`Handle` provide a packer WDF kernel runtime resource and objects, providing a data type that is able to represent itself 
as a WDF data type allocated inside the kernel space.

## Available Handle Specializations
The main filopsofy of *SWDK* is define what you want implementing `Handle<T>`, infactl `Handle` is providing a convenient e idiomatic way 
to describe WDF runtimes allocated data resources behaviors, ownership and capability in a safe Rust environment.
To preserv that goal only the really basic `Handle` specializations are provided by default (and will be added in the future).
Maintaining a giant wrapper aroun ALL WDF capability would be an hell and, more important, like say early, is not the objective of this crate.

Some of the currently `Handle<T>` impls are:

| Handle Type                      | Exposed Functionality                                          |
|----------------------------------|----------------------------------------------------------------|
| `Handle<WDFDRIVER>`              | Driver creation through `from_owned_with_attrs()`              |
| `Handle<WDFDEVICE>`              | Device creation through `from_owned()`                         |
| `Handle<WDFIOTARGET>`            | Access to device I/O targets through `from_owner()`            |
| `Handle<WDFIOTARGET>`            | I/O target state inspection through `read_status()`            |
| `Handle<WDFIOTARGET>`            | Typed synchronous IOCTL support through `send_ioctl_sync()`    |
| `Handle<WDFOBJECT>`              | Generic WDF object creation with typed attributes and contexts |
| `HandleRef<'_, PWDFDEVICE_INIT>` | Device filter configuration through `with_filter()`            |
| `HandleRef<'_, PWDFDEVICE_INIT>` | PnP/Power callback registration through `with_pnp_setup()`     |

___

### Data-Oriented by Design

Many frameworks built around KMDF attempt to encapsulate devices and framework resources behind large object-oriented abstractions.

SWDK intentionally avoids this approach.

Instead of building an object hierarchy, SWDK focuses on describing:

- ownership relationships
- object contexts
- descriptors
- object attributes
- framework callbacks
- I/O targets
- device capabilities

as composable pieces of data.

Traits are then used to transform these descriptions into native KMDF structures and runtime objects.

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

In SWDK, the same intent can be expressed as a composition of behaviors, for example:
```rust
fn initialize_device_init(device_init: &mut PWDF_DEVICE_INIT) -> PWDF_DEVICE_INIT {
    Handle::new(&device_init).with_filter().with_pnp_setup(
        WdfDevicePnpPowerSetup {
            on_device_d0_entry: Some(on_device_d0_entry),
            ..Default::default()
        }
    ).raw()
}

fn on_device_d0_entry(...) {
    ...
}
```

The result remains recognizably KMDF, but shifts the focus from imperative descriptor manipulation to declarative composition.

---

### Zero-Cost Abstractions

SWDK is heavily based on Rust generics, trait implementations and static dispatch.  
As a result, most abstractions provided by the framework exist only at compile time.

Types such as:

```rust
Handle<WDFDEVICE>
Handle<WDFIOTARGET>

WdfObjAttrs<DeviceContext>

IoCtlRequest<MyRequest>
IoCtlResponse<MyResponse>
```

carry additional semantic information for the compiler while introducing little to no runtime overhead.

The framework intentionally tries to avoid any kinds of dynamic dispatch (`dyn Trait`) specialization.
Instead all the specializatio onere is demanding to compiel time rust type resolution system through generics and traits.

This allows APIs to remain expressive while generating code that is extremely close to what an equivalent hand-written KMDF driver would produce.

In practice, most abstractions compile down to:

- direct function calls
- direct structure initialization
- direct pointer manipulation
- direct KMDF callback invocations

without introducing additional runtime layers.

The goal is simple:

> Better semantics for humans.
>
> No additional work for the CPU.

# Example

The following example shows a minimal KMDF driver implemented using SWDK.

```rust
use swdk::{
    Handle,
    if_nterror_return_ntstatus,
};
use swdk::bd::{
    WdfDriverConf,
    WdfDriverSetup,
    WdfObjAttrs,
};
use swdk::ctx::WdfCtxNoneDesc;
use swdk::rt::wdk_sys::{
    NTSTATUS,
    PCUNICODE_STRING,
    PDRIVER_OBJECT,
    PWDFDEVICE_INIT,
    STATUS_SUCCESS,
    WDFDRIVER,
};

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
            Some(
                WdfObjAttrs::<WdfCtxNoneDesc>::default()
            ),
        )
    );

    STATUS_SUCCESS
}

unsafe extern "C" fn on_device_add(
    _driver: WDFDRIVER,
    _device_init: PWDFDEVICE_INIT,
) -> NTSTATUS {
    info("Device added and loaded!");
    STATUS_SUCCESS
}
```

Although minimal, this is a real KMDF driver.

The example demonstrates some of the framework's core concepts:

- `Handle<T>` as the primary abstraction
- typed WDF object creation
- driver configuration through builders
- automatic descriptor initialization
- NTSTATUS-oriented error handling
# Getting Started

SWDK is built on top of Microsoft's
**windows-drivers-rs** ecosystem.

For using SWDK you need to use bindgen to get WDF bindings from C offcial sources.
Fortunately Microsft official wdf crate camwe to hep us with its wdk-build crate and cargo wdk (look at MS [windows-driver-rs](https://github.com/microsoft/windows-drivers-rs) for more info)

A typical driver project uses:

```toml
[dependencies]
swdk = "..."
```

`swdk-macros` optioanlly provides custom derive and attributes for swdk (like `#[derive(CtxDescriptor)]`):
```toml
[dependencies]
swdk = "..."
swdk-macros = "..."
```
## Build you driver
Because swdk is built on top of WDF, internally uses the MS official `wdk` crate exposed bindgen-generated functions bindings.
This means that you need a custom `build.rs` and a `makefile.toml` in your crate and a working and operational installation of clang on your machine to build your driver or wdk:

Fortunately, MS provides two very ysefulle crates to help us with this process:
```toml
[build-dependencies]
wdk-build = "..."
cargo-wdk = "..."
```

than the process is quite simple:

declare your custom `build.rs`:
```rust
fn main() -> Result<(), wdk_build::ConfigError> {
    wdk_build::configure_wdf_binary_build();
    Ok(())
}
```

and build your driver by cargo-wdk:
```bash
cargo wdk build
```

# Additional Resources
You can get more information about WDF KMDF (The only available runtime for now) or official rust wdk on their official sites and projects:

### Resources:

- [windows-drivers-rs](https://github.com/microsoft/windows-drivers-rs)
- [swdk official repo](https://github.com/rez23/swdk)
- [WDF get started](https://learn.microsoft.com/windows-hardware/drivers/wdf/)
- [API reference documentation for Windows Driver Kit](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/)