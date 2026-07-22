mod private {
    /// Map `$crate::rt::wdk_sys::call_unsafe_wdf_function_binding!` to a `Result`
    /// This permits to use the macro in a `?` context
    /// # Example
    /// ```
    /// unsafe {
    ///     call_ntstatus_wdf_binding!(
    ///         WdfDeviceCreate,
    ///         &raw mut device_init,
    ///         &raw mut attrs,
    ///         &raw mut device_handle)
    /// }?;
    /// ```
    #[cfg(feature = "wdk-runtime")]
    #[macro_export]
    macro_rules! call_ntstatus_wdf_unsafe_binding {
    ($func:ident $(, $args:expr )* $(,)?) => {{
        $crate::rt::logging::ntstatus_to_result(
            $crate::rt::wdk_sys::call_unsafe_wdf_function_binding!(
                $func,
                $($args),*
            )
        )}};
    }
}

/// Macro to declare a WDF (Windows Driver Framework) context descriptor for a specific type.
///
/// This macro defines static descriptors and handles required to associate a custom context type
/// (`$context_type`) with Windows Driver Framework (WDF) objects.
///
/// # Arguments
/// - `$context_type`: The user-defined struct type that represents the context to be associated
/// with a WDF object.
///
/// # Details
/// This macro generates:
/// - A static struct, internally used by WDF to instruct the Windows kernel on how to create
///   and manage contexts of type `$context_type`.
/// - A type alias for handles related to the `$context_type`.
/// - Implementation of the `AsCtxStaticDesc` trait for `$context_type`.
///
/// The generated code ensures seamless access to kernel-allocated instances of `$context_type`,
/// enabling retrieval of their unique descriptors and associated context objects.
///
/// # Example Usage
/// ```rust
/// use some_crate::declare_ctx_descriptor;
///
/// #[repr(C)]
/// struct MyContext {
///     value: u32,
/// }
///
/// declare_ctx_descriptor!(MyContext);
///
/// // Now, you can use `MyContext` with WDF seamlessly.
/// ```
///
/// # Generated Artifacts
/// 1. **Static Descriptor (`[<WDF_$context_type:snake:upper_TYPE_INFO>]`)**:
///    - A static variable with unique configuration belonging to `$context_type`.
///    - Contains metadata required by the framework, like size, unique type identifier, and context name.
///
/// 2. **Type Alias (`[<$context_type CtxHandle>]`)**:
///    - A scoped alias to handle WDF objects with the context type `$context_type`.
///
/// 3. **Trait Implementation**:
///    - Implements the `AsCtxStaticDesc` for `$context_type`, which provides:
///      - Access to the unique descriptor (`unique` method).
///      - `wdf_get` for acquiring a `const` pointer linked to WDF context.
///      - `wdf_get_mut` for acquiring a `mut` pointer linked to WDF context.
///      - Retrieval of the context type's name using `wdf_type_name`.
///
/// # Usage in different runtimes
/// The generated code supports both `test-runtime` and `wdk-runtime` configurations:
/// - `test-runtime`: Marks relevant functions as unimplemented.
/// - `wdk-runtime`: Leverages WDF APIs to perform runtime kernel-level operations.
///
/// # Safety
/// All unsafe operations are encapsulated and designed to comply with WDF and Rust's ownership
/// guarantees. However, improper usage may lead to undefined behavior.
///
/// # Notes
/// - This macro under the hood requires `wdk_sys` to be in your current crate or will not work
/// - This macro requires the `paste` crate to handle name concatenation for generated artifacts.
/// - The macro makes heavy use of unsafe code and should only be used when you are familiar with
///   WDF and kernel programming concepts.
///
/// # Warning
/// This macro needs at least `wdk_sys` to be in your current crate or will not work as expected!
/// This is needed because the internal call to wdk `unsafe_wdf_function_bindings!`
/// Because of a lak inside the original wdk macro declaration, this macro will call wdk directly,
/// making it impossible using reexported wdk from swdk.
#[cfg(any(feature = "test-runtime", feature = "wdk-runtime"))]
#[macro_export]
macro_rules! declare_ctx_descriptor {
    ($context_type:ty) => {
        use $crate::*;
        use $crate::operators::*;

        #[doc = concat!("A static struct internally used by WDF to instruct the kernel on how to create contexts of type: `",stringify!($context_type),"`")]
        #[unsafe(link_section = ".data")]
        $crate::__swdk_paste! {
            pub static [<WDF_ $context_type:snake:upper _TYPE_INFO>]: $crate::context::WdfObjCtxTypeInfo = $crate::context::WdfObjCtxTypeInfo::new(
                $crate::rt::wdk_sys::WDF_OBJECT_CONTEXT_TYPE_INFO {
                    Size: $crate::const_size_to_ulong!($crate::rt::wdk_sys::WDF_OBJECT_CONTEXT_TYPE_INFO),
                    ContextName: concat!(stringify!($context_type),'\0').as_bytes().as_ptr().cast(),
                    ContextSize: core::mem::size_of::<$context_type>(),
                    UniqueType: core::ptr::addr_of!(
                        [<WDF_ $context_type:snake:upper _TYPE_INFO>]
                    ).cast(),
                    EvtDriverGetUniqueContextType: None,
                }
            );
        }

        impl $crate::operators::AsCtxDesc for $context_type {
            #[doc = concat!("Get a ptr to the WDF unique descriptor of`", stringify!($context_type), "`")]
            fn unique() -> core::option::Option<$crate::rt::wdk_sys::PCWDF_OBJECT_CONTEXT_TYPE_INFO> {
                let unique = unsafe {
                    $crate::__swdk_paste!{[<WDF_ $context_type:snake:upper _TYPE_INFO>].unique()}
                };
                (!unique.is_null()).then(||{unique})
            }

            #[doc = concat!("Get a `const` ptr view on kernel allocated `", stringify!($context_type), "` associated with [`obj`]")]
            fn wdf_get<O>(obj: $crate::HandleRef<O>) -> core::option::Option<$crate::HandleRef<Self>> {
                 let ptr: *const $context_type = unsafe {
                    use $crate::rt::wdk_sys;
                    let unique = Self::unique()?;
                    $crate::rt::wdk_sys::call_unsafe_wdf_function_binding!(
                        WdfObjectGetTypedContextWorker,
                        core::ptr::from_ref(obj.as_ref()) as $crate::rt::wdk_sys::WDFOBJECT,
                        unique,
                    ).cast()
                 };

                 (!ptr.is_null()).then(|| unsafe {
                    $crate::HandleRef::new(&*ptr)
                 })
            }

            #[doc = concat!("Get a `mut` ptr view on kernel allocated instance of `", stringify!($context_type), "` associated with [`obj`]")]
            fn wdf_get_mut<O>(obj: $crate::HandleRef<O>) -> core::option::Option<$crate::HandleMut<Self>> {
                let ptr: *mut $context_type = unsafe {
                    use $crate::rt::wdk_sys;
                    let unique = Self::unique()?;
                    $crate::rt::wdk_sys::call_unsafe_wdf_function_binding!(
                        WdfObjectGetTypedContextWorker,
                        core::ptr::from_ref(obj.as_ref()) as $crate::rt::wdk_sys::WDFOBJECT,
                        unique,
                    ).cast()
                };

                (!ptr.is_null()).then(|| unsafe {
                    $crate::HandleMut::new(&mut *ptr)
                })
            }

            #[doc = concat!("Get the type name associated with `", stringify!($context_type), "` in WDF")]
            #[inline]
            fn wdf_type_name() -> core::option::Option<&'static str> {
                core::option::Option::Some(concat!(stringify!($context_type)))
            }
        }
    }
}
#[cfg(any(
    all(not(feature = "test-runtime"), not(feature = "wdk-runtime")),
    all(feature = "test-runtime", feature = "wdk-runtime"),
))]
#[macro_export]
macro_rules! declare_ctx_descriptor {
    ($context_type:ty) => {{
        compile_error!(
         "You need to one and only once valid runtime to declare context objects!
         you can use `test-runtime` or `wdk-runtime` feature to enable one")
    }}
}

#[macro_export]
macro_rules! impl_trait_for_wdf_handle {
    ($handler_name:ident,[$($safety:ident $trait_name:path),* $(,)?]) => {
        $(
            impl_trait_for_wdf_handle!(
                @trait
                $handler_name
                $safety
                $trait_name
            );
        )*
    };

    (@trait $handler_name:ident unsafe $trait_name:path) => {
        unsafe impl<'a> $trait_name for $handler_name<'a> {}
    };

    (@trait $handler_name:ident safe $trait_name:path) => {
        impl<'a> $trait_name for $handler_name<'a> {}
    };
}

#[macro_export]
macro_rules! create_handle_with_name {
    (
        $typename:ident,
        $handler_typename:ty,
    ) => {
        /// represent a container for type_name
        pub struct $handler_typename<'a>($handler_typename, PhantomData<&'a ()>)

        impl<'a> $handler_typename<'a> {
            $crate::__swdk_paste! {
                pub fn new([<$handler_typename:snake>]: $handler_typename) -> Self {
                    Self([<$handler_typename:snake>], PhantomData)
                }

                pub fn into_inner(self) -> $handler_typename {
                    self.0
                }
            }
        }

        impl<'a> OwnWdfObject<'a> for $handler_typename<'a> {}
    }
}

#[macro_export]
macro_rules! create_handle {
    (
        $handle_type:ident,
    ) => {
        $crate:wdf_declare_handle_with_namee!(
            $handle_type,
            $raw_type,
            $owner_type
        );
    };
}

#[macro_export]
macro_rules! create_handles {
    (
        $(
            $handle_type:ident,
            $raw_type:ty,
            $owner_type:ty
        );+ $(;)?
    ) => {
        $(
            $crate:wdf_declare_handlee!(
                $handle_type,
                $raw_type,
                $owner_type
            );
        )+
    };
}

#[macro_export]
macro_rules! impl_handle_as_builder {
    ($handle:ident, $descriptor:ty, $builder:block) => {
        impl<'a, T> AsRef<T> for $handle<'a, T> {
            #[inline]
            fn as_ref(&self) -> &T {
                &self.0
            }
        }

        impl<'a, T> AsBuilder<T> for $handle<'a, T> {
            type Descriptor<'a> = $descriptor
            where
                Self: 'a;

            fn build(&self) -> Self::Descriptor<'_> $builder
        }
    };
}

#[macro_export]
macro_rules! impl_handle_as_builder_mut {
    ($handle:ident, $descriptor:ty, $builder:block) => {
           impl<'a, T> AsMut<T> for $handle<'a, T> {
            #[inline]
            fn as_mut(&mut self) -> &mut T {
                &mut self.0
            }
        }

        impl<'a, T> AsBuilderMut<T> for $handle<'a, T> {
            type Descriptor<'a> = $descriptor
            where
                Self: 'a;

            fn build_mut(&mut self) -> Self::Descriptor<'_> $builder
        }
    };
}