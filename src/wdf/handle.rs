mod private {
    #[cfg(feature = "test-runtime")]
    use crate::rt::test_rt::*;

    use wdk_sys::HANDLE;

    /// A simple lifetime-safe handle for WDF kernel objects
    #[derive(Clone, Debug)]
    pub struct Handle<H = HANDLE>(H);
    pub type HandleRef<'a, T = HANDLE> = Handle<&'a T>;
    pub type HandleMut<'a, T = HANDLE> = Handle<&'a mut T>;

    mod _impls {
        use crate::{Handle, HandleRef};
        use crate::op::{AsPtr, AsRaw, AsRawWithBorrow};
        use core::borrow::Borrow;
        use core::ops::Deref;

        impl<H> Handle<H> {
            pub fn new(raw: H) -> Self {
                Self(raw)
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
        impl<H: Copy> AsRaw<H> for Handle<H> {}

        impl<'a, H> Borrow<H> for HandleRef<'a, H> {
            fn borrow(&self) -> &H {
                self.0
            }
        }
        impl<'a, H: Copy> AsRawWithBorrow<H> for HandleRef<'a, H> {}

        #[cfg(feature = "minimal-runtime")]
        mod _kmdf {
            mod _wdfobject {
                #[cfg(feature = "test-runtime")]
                use crate::rt::test_rt::*;
                
                use crate::bd::WdfObjAttrs;
                use crate::Handle;
                use crate::op::{
                    AsBuilder, AsCtxDescriptor, AsWdfOwner, NtResult,
                };
                use core::ptr;
                use wdk_sys::{WDFOBJECT, WDF_NO_HANDLE};
                use crate::rt::__cb;
                use crate::wdf::handle::private::_impls::_kmdf;

                impl AsWdfOwner<WDFOBJECT> for Handle<WDFOBJECT> {
                    type Conf = ();
                    type Owned = ();

                    fn from_owned_with_attrs<D>(
                        _: Self::Conf,
                        _: Self::Owned,
                        attrs: Option<WdfObjAttrs<D>>,
                    ) -> NtResult<Self> where
                            D: AsCtxDescriptor,
                    {
                        let mut obj = WDF_NO_HANDLE.cast();
                        let mut attrs = attrs.map(|a| a.build());
                        let attrs_ptr = attrs.as_mut().map_or(ptr::null_mut(), ptr::from_mut);

                        #[cfg(feature = "kmdf-runtime")]
                        unsafe {
                            __cb::wdf_object_create(
                                attrs_ptr,
                                &mut obj,
                            )
                        }?;

                        Ok(Handle::new(obj))
                    }
                }
            }
            mod _wdfdevice {
                #[cfg(feature = "test-runtime")]
                use crate::rt::test_rt::*;
                
                use crate::bd::WdfObjAttrs;
                use crate::op::{
                    AsBuilder, AsCtxDescriptor, AsWdfOwner, NtResult,
                };
                use core::ptr;

                use wdk_sys::{PWDFDEVICE_INIT, WDFDEVICE, WDF_NO_HANDLE};
                use crate::Handle;
                use crate::rt::__cb;

                impl AsWdfOwner<WDFDEVICE> for Handle<WDFDEVICE> {
                    type Conf = ();
                    type Owned = PWDFDEVICE_INIT;

                    fn from_owned_with_attrs<D>(
                        owner: Self::Owned,
                        _: Self::Conf,
                        attrs: Option<WdfObjAttrs<D>>,
                    ) -> NtResult<Self> where
                            D: AsCtxDescriptor,
                    {
                        let mut device: WDFDEVICE = WDF_NO_HANDLE.cast();
                        let mut attrs = attrs.map(|a| a.build());

                        let mut pdev_init = owner;

                        let attrs_ptr = attrs.as_mut().map_or(ptr::null_mut(), ptr::from_mut);

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
                    pub fn from_owned<D>(
                        owner: PWDFDEVICE_INIT,
                        attrs: Option<WdfObjAttrs<D>>,
                    ) -> NtResult<Self> where
                            D: AsCtxDescriptor,
                    {
                        Self::from_owned_with_attrs::<D>(owner, (), attrs)
                    }
                }
            }
            mod _wdfdriver {
                #[cfg(feature = "test-runtime")]
                use crate::rt::test_rt::*;
                
                use crate::bd::{WdfDriverConf, WdfObjAttrs};
                use crate::op::{
                    AsBuilder, AsCtxDescriptor, AsWdfOwner, NtResult,
                };
                use core::ptr;

                use wdk_sys::{PDRIVER_OBJECT, WDFDRIVER, WDF_NO_HANDLE};
                use crate::Handle;
                use crate::rt::__cb;

                impl AsWdfOwner<WDFDRIVER> for Handle<WDFDRIVER> {
                    type Conf = WdfDriverConf;
                    type Owned = PDRIVER_OBJECT;

                    fn from_owned_with_attrs<D>(
                        owner: Self::Owned,
                        conf: Self::Conf,
                        attrs: Option<WdfObjAttrs<D>>,
                    ) -> NtResult<Self> where
                            D: AsCtxDescriptor,
                    {
                        let mut driver: WDFDRIVER = WDF_NO_HANDLE.cast();
                        let mut attrs = attrs.map(|a| a.build());
                        let mut config = conf.build();

                        let driver_obj_ptr = owner;

                        let attrs_ptr = attrs.as_mut().map_or(ptr::null_mut(), ptr::from_mut);

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
            }
            mod _wio_target {
                #[cfg(feature = "kmdf-runtime")]
                use core::ptr;

                #[cfg(feature = "kmdf-runtime")]
                use wdk_sys::call_unsafe_wdf_function_binding;
                
                #[cfg(feature = "test-runtime")]
                use crate::rt::test_rt::*;

                use crate::ioctl::{IoCtlRequest, IoCtlResponse};
                use crate::op::{
                    AsBuilder, AsBuilderMut,
                    AsWdfOwned, NtResult,
                };
                use crate::runtime::utils::from_option_to_ptr;
                use crate::vals::WdfIoTargetError::{
                    IllegalState,
                };
                use crate::vals::{IoCtlTargetSendInfo, WdfIoTargetError, WdfIoTargetState};
                use wdk_sys::{ULONG_PTR, WDFDEVICE, WDFIOTARGET};

                #[cfg(feature = "kmdf-runtime")]
                use crate::vals::WdfIoTargetError::IoCtlTargetSendError;

                #[cfg(feature = "kmdf-runtime")]
                use crate::op::{AsOptionalBuff, AsRaw, AsRawWithBorrow};

                #[cfg(feature = "kmdf-runtime")]
                use wdk_sys::STATUS_UNSUCCESSFUL;
                use crate::Handle;
                use crate::rt::__cb;
                use crate::runtime::kmdf::{wdf_get_targetio_state, wdf_request_send_async};

                impl AsWdfOwned<WDFIOTARGET> for Handle<WDFIOTARGET> {
                    type Owner = WDFDEVICE;
                    #[cfg_attr(feature = "test-runtime",
                        expect(unused_variables, reason = "Unused because of test-runtime"))]
                    fn from_owner(owner: &Self::Owner) -> NtResult<Self> {
                        #[cfg(feature = "kmdf-runtime")]{
                            let device = owner;
                            let io_target = unsafe {
                                __cb::wdf_get_target_io(*device)
                            };

                            if io_target.is_null() {
                                return Err(STATUS_UNSUCCESSFUL);
                            }
                            Ok(Self::new(io_target))
                        }
                        #[cfg(feature = "test-runtime")] {
                            compile_error!("Cannot create WDFIOTARGET handle in test-runtime")
                        }
                    }
                }

                impl Handle<WDFIOTARGET> {
                    pub fn read_status(&self) -> WdfIoTargetState {
                        #[cfg(feature = "kmdf-runtime")]{
                            WdfIoTargetState::from(unsafe {
                                wdf_get_targetio_state(self.raw())
                            })
                        }
                        #[cfg(feature = "test-runtime")]{
                            WdfIoTargetState::Started
                        }
                    }

                    pub fn send_ioctl<R: Default>(
                        &self,
                        request: IoCtlRequest<Option<R>>,
                    ) -> Result<IoCtlResponse<R>, WdfIoTargetError> {
                        // if device is not started return error
                        let device_status = self.read_status();
                        let WdfIoTargetState::Started = device_status else {
                            return Err(IllegalState(device_status));
                        };

                        // build input ioctl output buffer only if request has data to send
                        let request_desc = request.build();

                        #[cfg_attr(
                            feature = "test-runtime",
                            expect(unused_variables, reason = "Unused because of test-runtime")
                        )] let request_desc_ptr = from_option_to_ptr(request_desc.as_ref());

                        // build Ioctl output buffer ptr
                        let mut response = IoCtlResponse::<R>::default();

                        #[cfg_attr(feature = "test-runtime",
                            expect(
                                unused_mut,
                                unused_variables,
                                reason = "Unused because of test-runtime"
                            ))] let mut response_desc = response.build_mut();

                        #[cfg_attr(feature = "test-runtime",
                            expect(
                                unused_mut,
                                unused_variables,
                                reason = "Unused because of test-runtime"
                            ))] let mut bytes_returned: ULONG_PTR = 0;

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
                }
            }
        }
    }
}

#[allow(unused)]
pub use private::{Handle, HandleRaw, HandleRawMut, HandleRef, HandleMut};
