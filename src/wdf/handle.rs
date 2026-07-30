mod private {
    use crate::rt::wdk_sys::HANDLE;

    /// Encapsulates a kernel object or resource [`HANDLE`] of type `H`.
    ///
    /// # About [`Handle`]
    /// [`HANDLE`] is one of the core concepts in the Windows Driver Framework (WDF) and represents
    /// a handle to a kernel resource or object.
    /// Handles are used extensively in WDF to manage various system resources,
    /// such as device objects, file objects, and other kernel objects. This is the Rust version.
    /// There are 3 common forms of `Handle<T>`, exposed through 3 main aliases of [`Handle`]:
    /// - [`Handle`]: Simply a `Handle<T>`. Typically a raw pointer to a kernel object.
    /// - [`HandleRef`]: `Handle<&'a T>`, typically a reference to a raw pointer that points to a kernel object.
    /// - [`HandleMut`]: `Handle<&'a mut T>`, typically a mutable reference to a raw pointer that points to a kernel object.
    ///
    /// # Type Parameters
    /// - `H`: The handle type being wrapped. Defaults to `HANDLE` if not explicitly specified.
    ///
    /// # Traits
    /// - `Clone`: Allows `Handle` to be cloned, creating a duplicate with the same underlying handle.
    /// - `Debug`: Enables formatting of `Handle` for debugging purposes.
    ///
    /// # Example
    /// `Handle` is the basic building block of `swdk`.
    /// You can use it to implement your own functions for raw WDF kernel types.
    /// For example, the `swdk` library implements [`Handle::read_status()`] for [`WDFIOTARGET`]
    /// exactly in this way:
    /// ```rust
    /// impl swdk::Handle<swdk::rt::wdk_sys::WDFIOTARGET> {
    ///     pub fn read_status(&self) -> swdk::val::WdfIoTargetState {
    ///         swdk::val::WdfIoTargetState::from(unsafe {
    ///             ...
    /// ```
    ///
    /// `swdk` is still in development, anyway, you can already declare a full Rust driver
    /// in just a few lines using `Handle`:
    /// ```rust
    /// use swdk::rt::wdk_sys::{WDFDEVICE, WDFDRIVER, PWDFDEVICE_INIT, STATUS_SUCCESS};
    /// use swdk::Handle;
    /// use swdk::if_nterror_return_ntstatus;
    /// use swdk::bd::{WdfDriverConf, WdfDriverSetup, WdfObjAttrs};
    /// use swdk::println;
    ///
    /// type HandleDevice = Handle<WDFDEVICE>;
    ///
    ///#[unsafe(export_name = "DriverEntry")]
    /// pub unsafe extern "system" fn driver_entry(
    ///     driver_obj: PDRIVER_OBJECT,
    ///     registry_path: PCUNICODE_STRING,
    /// ) -> NTSTATUS {
    ///     debug!("DriverEntry launched from WDF");
    ///     if_nterror_return_ntstatus!(
    ///         Handle::<WDFDRIVER>::allocate_from_owned(
    ///             driver_obj,
    ///             WdfDriverConf {
    ///                 setup: WdfDriverSetup {
    ///                     on_driver_unload: Some(
    ///                         on_driver_unload
    ///                     ),
    ///                     on_device_add: Some(
    ///                         on_driver_device_add
    ///                     ),
    ///                     ..WdfDriverSetup::default()
    ///                 },
    ///                 registry_path,
    ///             },
    ///             Some(WdfObjAttrs::<WdfCtxNoneDesc>::default())
    ///         )
    ///     );
    ///     STATUS_SUCCESS
    /// }
    ///
    /// #[unsafe(link_section = "PAGE")]
    /// unsafe extern "C" fn on_driver_device_add(
    ///     _driver: WDFDRIVER,
    ///     device_init: PWDFDEVICE_INIT,
    /// ) -> NTSTATUS {
    ///     println!("Hello world")
    /// }
    ///
    /// #[unsafe(link_section = "PAGE")]
    /// unsafe extern "C" fn on_driver_unload(
    ///     _driver: WDFDRIVER,
    ///     device_init: PWDFDEVICE_INIT,
    /// ) -> NTSTATUS {
    ///     println!("Goodbye")
    /// }
    /// ```
    /// Above, an example of a potential driver that writes "Hello world" to WinDbg buffer
    /// when its device is added, and "Goodbye" when it unloads.
    /// # See Also
    /// - [swdk official repo](https://github.com/rez23/swdk)
    /// - [WDF get started](https://learn.microsoft.com/windows-hardware/drivers/wdf/)
    /// - [API reference documentation for Windows Driver Kit](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/)
    #[derive(Clone, Debug)]
    pub struct Handle<H = HANDLE>(H);

    /// A reference to a kernel object or resource
    ///
    /// # See Also
    /// This is just an alias to [`Handle<T>`] where `T` is `&T`
    pub type HandleRef<'a, T = HANDLE> = Handle<&'a T>;

    /// A mutable reference to a kernel object or resource
    ///
    ///# See Also
    /// This is just an alias to [`Handle<T>`] where `T` is `&mut T`
    pub type HandleMut<'a, T = HANDLE> = Handle<&'a mut T>;

    mod _impls {
        use core::borrow::Borrow;
        use core::ops::Deref;

        use crate::op::{
            AsPtr, AsRaw, AsRawWithBorrow,
            AsWdfType, IntoInner, IntoRaw,
        };
        use crate::{Handle, HandleRef};

        impl<H> Handle<H> {
            pub fn new(raw: H) -> Self {
                Self(raw)
            }
        }

        impl<H: Copy> AsWdfType<H> for Handle<H> {}
        impl<H> IntoInner<H> for Handle<H> {
            fn into_inner(self) -> H {
                self.0
            }
        }
        impl<H> Deref for Handle<H> {
            type Target = H;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl<H> AsRef<H> for Handle<H> {
            fn as_ref(&self) -> &H {
                &self.0
            }
        }
        impl<H> AsPtr<H> for Handle<H> {}
        unsafe impl<H> Send for Handle<H> {}
        unsafe impl<H> Sync for Handle<H> {}
        impl<H: Copy> IntoRaw<H> for Handle<H> {}
        impl<H: Copy> AsRaw<H> for Handle<H> {}

        impl<'a, H> Borrow<H> for HandleRef<'a, H> {
            fn borrow(&self) -> &H {
                self.0
            }
        }
        impl<'a, H: Copy> AsRawWithBorrow<H> for HandleRef<'a, H> {}

        impl<'a, H: Copy> HandleRef<'a, H> {
            pub fn raw(&self) -> H {
                *self.0
            }
        }

        #[cfg(feature = "minimal-runtime")]
        mod _kmdf {
            mod _wdfhandle {
                use crate::rt::wdk_sys::HANDLE;

                use crate::Handle;
                use crate::op::{AsRaw, AsWdfHandle};

                unsafe impl AsWdfHandle<HANDLE> for Handle {
                    fn as_wdf_handle(&self) -> HANDLE {
                        self.raw()
                    }
                }
            }
            mod _wdfobject {
                use core::ptr;

                use crate::Handle;
                use crate::bd::WdfObjAttrs;
                use crate::op::{
                    AsBuilder, AsCtxDescriptor, AsWdfObject, AsWdfOwner,
                    AsWdfWithCtx, NtResult,
                };
                #[cfg(feature = "kmdf-runtime")]
                use crate::rt::__cb;
                use crate::rt::wdk_sys::{WDFOBJECT,
                                         WDF_NO_HANDLE,
                };

                #[cfg(feature = "kmdf-runtime")]
                unsafe impl AsWdfObject<WDFOBJECT> for Handle<WDFOBJECT> {}

                impl Handle<WDFOBJECT> {
                    pub fn allocate(attrs: Option<WdfObjAttrs>) -> NtResult<Self> {
                        Self::allocate_from_owned(
                            (),
                            None,
                            attrs,
                        )
                    }
                }

                impl AsWdfOwner<WDFOBJECT> for Handle<WDFOBJECT> {
                    type Conf = ();
                    type Owned = ();

                    fn allocate_from_owned<D>(
                        _: Self::Owned,
                        _: Option<Self::Conf>,
                        attrs: Option<WdfObjAttrs<D>>,
                    ) -> NtResult<Self> where
                            D: AsCtxDescriptor,
                    {
                        let mut obj = WDF_NO_HANDLE.cast();
                        let mut attrs = attrs.map(|a| a.build());
                        let attrs_ptr = attrs.as_mut().map_or(
                            ptr::null_mut(),
                            ptr::from_mut,
                        );

                        #[cfg(feature = "kmdf-runtime")]
                        unsafe {
                            __cb::wdf_object_create(
                                attrs_ptr, &mut obj,
                            )
                        }?;

                        Ok(Handle::new(obj))
                    }
                }
                
                #[cfg(feature = "kmdf-runtime")]
                unsafe impl AsWdfWithCtx<WDFOBJECT> for Handle<WDFOBJECT> {}
            }
            mod _wdfdevice {
                use core::ptr;

                use crate::Handle;
                use crate::bd::WdfObjAttrs;
                use crate::op::{
                    AsBuilder, AsCtxDescriptor, AsWdfOwner,
                    NtResult,
                };
                #[cfg(feature = "kmdf-runtime")]
                use crate::rt::__cb;
                use crate::rt::wdk_sys::{
                    PWDFDEVICE_INIT, WDF_NO_HANDLE,
                    WDFDEVICE,
                };

                impl AsWdfOwner<WDFDEVICE> for Handle<WDFDEVICE> {
                    type Conf = ();
                    type Owned = PWDFDEVICE_INIT;

                    fn allocate_from_owned<D>(
                        owned: Self::Owned,
                        _: Option<Self::Conf>,
                        attrs: Option<WdfObjAttrs<D>>,
                    ) -> NtResult<Self> where
                            D: AsCtxDescriptor,
                    {
                        let mut device: WDFDEVICE = WDF_NO_HANDLE.cast();
                        let mut attrs = attrs.map(|a| a.build());

                        let mut pdev_init = owned;

                        let attrs_ptr = attrs.as_mut().map_or(
                            ptr::null_mut(),
                            ptr::from_mut,
                        );

                        let dev_init_ptr = &raw mut pdev_init;
                        let device_ptr = &raw mut device;

                        #[cfg(feature = "kmdf-runtime")]
                        // SAFETY: `driver_init` is safe because is passed by WDF
                        unsafe {
                            __cb::wdf_device_create(
                                dev_init_ptr,
                                attrs_ptr,
                                device_ptr,
                            )
                        }?;

                        Ok(Self::new(device))
                    }
                }
                impl Handle<WDFDEVICE> {
                    /// Creates a new instance of the object from an owned `PWDFDEVICE_INIT` with optional attributes.
                    ///
                    /// This method initializes the object using a provided `PWDFDEVICE_INIT` that is owned by the caller
                    /// and applies optional object attributes if specified. It delegates the actual creation to the method
                    /// `from_owned_with_attrs`.
                    ///
                    /// # Type Parameters
                    ///
                    /// - [`D`]: A type that implements `AsCtxDescriptor`. This is used to describe the context associated
                    ///   with the object being created.
                    ///
                    /// # Parameters
                    ///
                    /// - [`owner`]: A `PWDFDEVICE_INIT` structure that must be owned by the caller. This structure is utilized
                    ///   for the initialization process of the object.
                    /// - [`attrs`]: An optional `WdfObjAttrs` descriptor that configures the attributes of the object. Pass
                    ///   `None` if no additional attributes are required.
                    ///
                    /// # Returns
                    ///
                    /// Returns an [`NtResult`] containing the newly created object wrapped in `Self` on success. If an
                    /// error occurs during initialization, the result contains the corresponding [`NTSTATUS`] failure code.
                    ///
                    /// # Examples
                    ///
                    /// ```rust
                    /// let device_init: PWDFDEVICE_INIT = ...;
                    /// let attrs = Some(WdfObjAttrs::new(...));
                    ///
                    /// let result = MyObject::from_owned(device_init, attrs);
                    /// match result {
                    ///     Ok(obj) => {
                    ///         // Successfully created the object
                    ///     }
                    ///     Err(status) => {
                    ///         // Handle the error
                    ///     }
                    /// }
                    /// ```
                    ///
                    /// # Notes
                    ///
                    /// - This method is typically used when the caller explicitly owns the [`PWDFDEVICE_INIT`] handle,
                    ///   ensuring its proper management during the creation process.
                    /// - If additional context or attributes need to be applied beyond the basics, use the [`attrs`] parameter
                    ///   to specify them.
                    pub fn allocate<D>(
                        owner: PWDFDEVICE_INIT,
                        attrs: Option<WdfObjAttrs<D>>,
                    ) -> NtResult<Self> where
                            D: AsCtxDescriptor,
                    {
                        Self::allocate_from_owned::<D>(
                            owner,
                            None,
                            attrs,
                        )
                    }
                }
            }
            mod _pdevice_init {
                use core::ptr;

                use wdk_sys::PWDFDEVICE_INIT;

                use crate::HandleRef;
                use crate::bd::WdfDevicePnpPowerSetup;
                use crate::op::AsBuilder;
                #[cfg(feature = "kmdf-runtime")]
                use crate::rt::__cb;

                impl<'a> HandleRef<'a, PWDFDEVICE_INIT> {
                    #[inline]
                    #[cfg(feature = "kmdf-runtime")]
                    pub fn with_filter(self) -> Self {
                        unsafe {
                            __cb::wdf_f_do_init_set_filter(
                                self.0,
                            )
                        };
                        self
                    }

                    #[inline]
                    #[cfg(feature = "kmdf-runtime")]
                    pub fn with_pnp_setup(
                        self,
                        setup: WdfDevicePnpPowerSetup,
                    ) -> Self {
                        let pnp_setup = setup.build();
                        unsafe {
                            __cb::wdf_device_init_set_pnp_power_event_callbacks(
                                self.raw(),
                                ptr::from_ref(&pnp_setup).cast_mut(),
                            )
                        };
                        self
                    }
                }
            }

            mod _wdfdriver {
                use core::ptr;
                use wdk_sys::STATUS_INVALID_PARAMETER;
                use crate::Handle;
                use crate::bd::{
                    WdfDriverConf, WdfObjAttrs,
                };
                use crate::op::{
                    AsBuilder, AsCtxDescriptor, AsWdfOwner,
                    NtResult,
                };
                #[cfg(feature = "kmdf-runtime")]
                use crate::rt::__cb;
                use crate::rt::wdk_sys::{
                    PDRIVER_OBJECT, WDF_NO_HANDLE,
                    WDFDRIVER,
                };

                impl AsWdfOwner<WDFDRIVER> for Handle<WDFDRIVER> {
                    type Conf = WdfDriverConf;
                    type Owned = PDRIVER_OBJECT;

                    fn allocate_from_owned<D>(
                        owner: Self::Owned,
                        conf: Option<Self::Conf>,
                        attrs: Option<WdfObjAttrs<D>>,
                    ) -> NtResult<Self> where
                            D: AsCtxDescriptor,
                    {
                        let mut driver: WDFDRIVER = WDF_NO_HANDLE.cast();
                        let mut attrs = attrs.map(|a| a.build());
                        let conf = conf.ok_or({
                            STATUS_INVALID_PARAMETER
                        })?;
                        let mut config = conf.build();

                        let driver_obj_ptr = ptr::NonNull::new(owner)
                            .ok_or(STATUS_INVALID_PARAMETER)?
                            .as_ptr();

                        let attrs_ptr = attrs.as_mut().map_or(
                            ptr::null_mut(),
                            ptr::from_mut,
                        );

                        let registry_path = conf.registry_path;
                        let config_ptr = &raw mut config;
                        let driver_ptr = &raw mut driver;

                        #[cfg(feature = "kmdf-runtime")]
                        // SAFETY: `driver_init` is safe because is passed by WDF
                        unsafe {
                            __cb::wdf_driver_create(
                                driver_obj_ptr,
                                registry_path,
                                attrs_ptr,
                                config_ptr,
                                driver_ptr,
                            )
                        }?;

                        Ok(Self::new(driver))
                    }
                }
                impl Handle<WDFDRIVER> {
                    pub fn allocate(
                        p_driver_obj: PDRIVER_OBJECT,
                        conf: WdfDriverConf,
                        attrs: Option<WdfObjAttrs>
                    ) -> NtResult<Self> {
                        Self::allocate_from_owned(p_driver_obj, Some(conf), attrs)
                    }
                }
            }
            mod _wio_target {
                #[cfg(feature = "kmdf-runtime")]
                use core::ptr;

                use wdk_sys::{PWDF_IO_TARGET_OPEN_PARAMS, STATUS_INVALID_PARAMETER, WDF_NO_HANDLE};

                use crate::Handle;
                use crate::bd::WdfObjAttrs;
                use crate::ioctl::{
                    IoCtlRequest, IoCtlResponse,
                };
                use crate::op::{AsBuilder, AsBuilderMut, AsWdfFromOwner, AsWdfOwned, NtResult};
                #[cfg(feature = "kmdf-runtime")]
                use crate::op::{AsOptionalBuff, AsRaw};
                #[cfg(feature = "kmdf-runtime")]
                use crate::rt::__cb;
                #[cfg(feature = "kmdf-runtime")]
                use crate::rt::wdk_sys::STATUS_UNSUCCESSFUL;
                use crate::rt::wdk_sys::{
                    ULONG_PTR, WDFDEVICE, WDFIOTARGET,
                };
                #[cfg(feature = "kmdf-runtime")]
                use crate::runtime::kmdf::{
                    wdf_request_send_async,
                    wdf_target_io_get_state,
                };
                use crate::runtime::utils::from_option_to_ptr;
                use crate::vals::WdfIoTargetError::IllegalState;
                #[cfg(feature = "kmdf-runtime")]
                use crate::vals::WdfIoTargetError::IoCtlTargetSendError;
                use crate::vals::{
                    IoCtlTargetSendInfo, WdfIoTargetError,
                    WdfIoTargetState,
                };

                impl Handle<WDFIOTARGET> {
                    pub fn allocate_default(
                        owner: &WDFDEVICE,
                    ) -> NtResult<Self> {
                        #[cfg(feature = "kmdf-runtime")]
                        {
                            let device = ptr::NonNull::new(*owner)
                                .ok_or(STATUS_INVALID_PARAMETER)?
                                .as_ptr();

                            let io_target = unsafe {
                                __cb::wdf_target_io_get(
                                    device,
                                )
                            };

                            if io_target.is_null() {
                                return Err(
                                    STATUS_UNSUCCESSFUL,
                                );
                            }
                            Ok(Self::new(io_target))
                        }
                        #[cfg(feature = "test-runtime")]
                        {
                            unimplemented!(
                                "Cannot create WDFIOTARGET handle in test-runtime"
                            )
                        }
                    }
                }

                #[cfg(feature = "kmdf-runtime")]
                impl AsWdfFromOwner<WDFIOTARGET> for Handle<WDFIOTARGET> {}

                #[cfg(feature = "kmdf-runtime")]
                impl AsWdfOwned<WDFIOTARGET> for Handle<WDFIOTARGET> {
                    type Owner = WDFDEVICE;
                    type Conf = ();

                    #[cfg_attr(feature = "test-runtime",
                        expect(unused_variables,
                            reason = "Unused because of test-runtime"))]
                    fn allocate_from_owner(
                        owner: &Self::Owner,
                        _: Option<Self::Conf>,
                        attrs: Option<WdfObjAttrs>,
                    ) -> NtResult<Self> {
                        let mut io_target: WDFIOTARGET = WDF_NO_HANDLE.cast();
                        let mut attrs = attrs.map(|a| a.build());

                        let device_ptr = ptr::NonNull::new(*owner)
                            .ok_or(STATUS_INVALID_PARAMETER)?
                            .as_ptr();

                        let attrs_ptr = attrs.as_mut().map_or(
                            ptr::null_mut(),
                            ptr::from_mut,
                        );

                        #[cfg(feature = "kmdf-runtime")]
                        // SAFETY: `device_ptr` is safe because is passed by WDF
                        unsafe {
                            __cb::wdf_target_io_create(
                                device_ptr,
                                attrs_ptr,
                                &raw mut io_target,
                            )
                        }?;

                        Ok(Self::new(io_target))
                    }
                }
                impl Handle<WDFIOTARGET> {
                    /// Reads and returns the status of the I/O target.
                    ///
                    /// This method determines the current state of the I/O target and returns
                    /// it as a `WdfIoTargetState` enum. The returned state depends on the
                    /// active runtime feature.
                    ///
                    /// # Runtime Features
                    /// - **kmdf-runtime**:
                    ///   - If this feature is enabled, the state is retrieved by calling the
                    ///     underlying KMDF-specific function `wdf_get_targetio_state` on the
                    ///     raw handle returned by the `self.raw()` method.
                    ///   - Note: The call to `wdf_get_targetio_state` is marked `unsafe`
                    ///     because it interacts directly with a lower-level API.
                    /// - **test-runtime**:
                    ///   - If this feature is enabled, the method will always return
                    ///     `WdfIoTargetState::Started`, simulating a consistent state for
                    ///     testing purposes.
                    ///
                    /// # Returns
                    /// A `WdfIoTargetState` indicating the current status of the I/O target.
                    ///
                    /// # Safety
                    /// When the `kmdf-runtime` feature is enabled, this method uses an
                    /// `unsafe` block to call `wdf_get_targetio_state`. Ensure that the
                    /// environment and underlying abstractions are correctly initialized and
                    /// managed to avoid undefined behavior.
                    pub fn read_status(
                        &self,
                    ) -> WdfIoTargetState {
                        #[cfg(feature = "kmdf-runtime")]
                        {
                            WdfIoTargetState::from(unsafe {
                                wdf_target_io_get_state(
                                    self.raw(),
                                )
                            })
                        }
                        #[cfg(feature = "test-runtime")]
                        {
                            WdfIoTargetState::Started
                        }
                    }

                    /// Sends an IOCTL (Input/Output Control) request to the device and waits for a response.
                    ///
                    /// This function is designed to send IOCTL requests when the device is in a
                    /// "Started" state. It constructs the input and output buffers for the IOCTL operation
                    /// and interacts with the underlying KMDF runtime (if enabled) using the appropriate
                    /// APIs. It returns the response obtained from the device if the operation is successful
                    /// or an error if the operation fails.
                    ///
                    /// # Type Parameters
                    /// * [`R`]: The type of the expected response from the IOCTL operation. Must implement [`Default`].
                    ///
                    /// # Parameters
                    /// * [`request`]: An [`IoCtlRequest`] containing the IOCTL command and optional data to send to the device.
                    ///
                    /// # Returns
                    /// * [`Ok(IoCtlResponse<R>)`]: On success, returns a populated [`IoCtlResponse`] of type [`R`] which holds
                    ///   the response to the request.
                    /// * [`Err(WdfIoTargetError)`]: On failure, returns a [`WdfIoTargetError`] describing the nature of the error.
                    ///
                    /// # Errors
                    /// * This function will return:
                    ///   * [`WdfIoTargetState::IllegalState`]: If the device is not in a "Started" state.
                    ///   * [`WdfIoTargetState::IoCtlTargetSendError`]: If sending the IOCTL request failed, providing detailed information
                    ///     such as the command, NTSTATUS code, any optional input buffer, and bytes returned.
                    ///
                    /// # Key Operations
                    /// 1. Validates that the device is in the "Started" state.
                    /// 2. Builds the input and output buffers for the IOCTL operation.
                    /// 3. Interacts with the device driver stack (e.g., via [`WdfIoTargetSendIoctlSynchronously`] in KMDF mode).
                    /// 4. Processes the response or handles any errors arising from the operation.
                    ///
                    /// # Safety
                    /// * Unsafe blocks are used for interacting with the KMDF runtime. These blocks ensure that the
                    ///   required pointers for buffers are valid and the KMDF callback for sending IOCTL requests correctly
                    ///   handles the data. Developer attention is required when working in "kmdf-runtime" mode.
                    ///
                    /// # Features
                    /// This function includes conditional compilation specifically for the `test-runtime` feature:
                    /// * Suppresses warnings for unused variables where certain parameters are not required during
                    ///   testing scenarios.
                    ///
                    /// # Example
                    /// ```rust
                    /// // Example usage:
                    /// let ioctl_request = IoCtlRequest::new(REQUEST_CODE, Some(input_data));
                    /// let ioctl_response: Result<IoCtlResponse<ResponseData>, WdfIoTargetError> = device.send_ioctl(ioctl_request);
                    ///
                    /// match ioctl_response {
                    ///     Ok(response) => println!("Response received: {:?}", response.data()),
                    ///     Err(e) => eprintln!("Failed to send IOCTL request: {:?}", e),
                    /// }
                    /// ```
                    ///
                    /// # Dependencies
                    /// This function relies on the following external components:
                    /// * [`IoCtlRequest`]: Represents the IOCTL request, including the command and optional data.
                    /// * [`IoCtlResponse`]: Represents the response from the IOCTL request.
                    /// * [`WdfIoTargetError`]: Error types related to WDF IO targets.
                    /// * KMDF APIs (if the ` kmdf-runtime ` feature is enabled).
                    ///
                    /// # Notes
                    /// * Ensure you manage the lifetime and validity of `self` and buffers when interacting
                    ///   with the KMDF runtime, as improper handling can lead to undefined behavior in unsafe blocks.
                    pub fn send_ioctl_sync<R: Default>(
                        &self,
                        request: IoCtlRequest<Option<R>>,
                    ) -> Result<
                        IoCtlResponse<R>,
                        WdfIoTargetError,
                    > {
                        // if device is not started, return error
                        let device_status = self.read_status();
                        let WdfIoTargetState::Started = device_status
                        else {
                            return Err(IllegalState(
                                device_status,
                            ));
                        };

                        // build input ioctl output buffer only if request has data to send
                        let request_desc = request.build();

                        #[cfg_attr(
                            feature = "test-runtime",
                            expect(unused_variables,
                                reason = "Unused because of test-runtime")
                        )] let request_desc_ptr = from_option_to_ptr(
                            request_desc.as_ref(),
                        );

                        // build Ioctl output buffer ptr
                        let mut response = IoCtlResponse::<R>::default();

                        #[cfg_attr(
                            feature = "test-runtime",
                            expect(unused_mut,
                                unused_variables,
                                reason = "Unused because of test-runtime")
                        )] let mut response_desc = response.build_mut();

                        #[cfg_attr(
                            feature = "test-runtime",
                            expect(unused_mut,
                                unused_variables,
                                reason = "Unused because of test-runtime")
                        )] let mut bytes_returned: ULONG_PTR = 0;

                        #[cfg(feature = "kmdf-runtime")]
                        // SAFETY: Is iot safe since output_desc and bytes_returned are valid pointers
                        //         and `WdfIoTargetSendIoctlSynchronously` KMDF callback handles it correctly
                        unsafe {
                            // Retrieve the general collection info (including the required preparsed descriptor size)
                            wdf_request_send_async(
                                self.raw(),
                                request.command(),
                                ptr::null_mut(),
                                request_desc_ptr.cast_mut(),
                                &raw mut response_desc,
                                ptr::null_mut(),
                                &raw mut bytes_returned,
                            )
                        }.map_err(|ntstatus| {
                            IoCtlTargetSendError(IoCtlTargetSendInfo {
                                command: request.command(),
                                ntstatus,
                                request: request.as_buff().map(|buff| buff.to_vec()).unwrap_or_default(),
                                byte_returned: bytes_returned as usize,
                            })
                        })?;

                        Ok(response)
                    }

                    pub fn open(
                        params: PWDF_IO_TARGET_OPEN_PARAMS,
                    ) {}
                }
            }

            mod _wdf_queue {
                use core::ptr;
                use wdk_sys::{STATUS_INVALID_PARAMETER, WDFDEVICE, WDFQUEUE, WDF_NO_HANDLE};

                use crate::bd::{
                    WdfIoQueueConfig, WdfObjAttrs,
                };
                use crate::op::{
                    AsBuilder,
                    AsWdfFromOwnerWithConfAndAttrs,
                    AsWdfOwned, NtResult,
                };

                #[cfg(feature = "kmdf-runtime")]
                use crate::rt::__cb;
                use crate::wdf::handle::private::Handle;

                impl AsWdfOwned<WDFQUEUE> for Handle<WDFQUEUE> {
                    type Owner = WDFDEVICE;
                    type Conf = WdfIoQueueConfig;

                    fn allocate_from_owner(owner: &Self::Owner, conf: Option<Self::Conf>, attrs: Option<WdfObjAttrs>) -> NtResult<Self> {
                        let mut queue = WDF_NO_HANDLE.cast();
                        let device = ptr::NonNull::new(*owner)
                            .ok_or(STATUS_INVALID_PARAMETER)?
                            .as_ptr();

                        let mut config = conf.map(|c| c.build()).ok_or(STATUS_INVALID_PARAMETER)?;
                        let mut attrs = attrs.map(|a| a.build());

                        let attrs_ptr = attrs.as_mut().map_or(
                            ptr::null_mut(),
                            ptr::from_mut,
                        );

                        #[cfg(feature = "kmdf-runtime")]
                        // SAFETY: `device` is safe because cannot be null
                        //          and is passed directly from wdf
                        unsafe {
                            __cb::wdf_io_queue_create(
                                device,
                                &raw mut config,
                                attrs_ptr,
                                &raw mut queue,
                            )
                        }?;


                        Ok(Self::new(queue))
                    }
                }
                impl AsWdfFromOwnerWithConfAndAttrs<WDFQUEUE> for Handle<WDFQUEUE> {}
            }
        }
    }
}

#[allow(unused)]
pub use private::{Handle, HandleMut, HandleRef};
