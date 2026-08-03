mod private {
    pub mod operations {
        use core::ptr;
        use crate::rt::wdk_sys::{
            _WDF_MEMORY_DESCRIPTOR__bindgen_ty_1,
            _WDF_MEMORY_DESCRIPTOR__bindgen_ty_1__bindgen_ty_1,
            WDF_MEMORY_DESCRIPTOR, _WDF_MEMORY_DESCRIPTOR_TYPE,
        };
        use crate::size_to_ulong;

        #[allow(dead_code)]
        pub fn build_for_data_type<T>(elem: &T) -> WDF_MEMORY_DESCRIPTOR {
            WDF_MEMORY_DESCRIPTOR {
                Type:
                    _WDF_MEMORY_DESCRIPTOR_TYPE::WdfMemoryDescriptorTypeBuffer,
                u: _WDF_MEMORY_DESCRIPTOR__bindgen_ty_1 {
                    BufferType:
                        _WDF_MEMORY_DESCRIPTOR__bindgen_ty_1__bindgen_ty_1 {
                            Buffer: ptr::from_ref(elem).cast_mut().cast(),
                            Length: size_to_ulong!(size_of::<T>()),
                        },
                },
            }
        }

        pub fn build_for_data_type_mut<T>(
            elem: &mut T,
        ) -> WDF_MEMORY_DESCRIPTOR {
            WDF_MEMORY_DESCRIPTOR {
                Type:
                    _WDF_MEMORY_DESCRIPTOR_TYPE::WdfMemoryDescriptorTypeBuffer,
                u: _WDF_MEMORY_DESCRIPTOR__bindgen_ty_1 {
                    BufferType:
                        _WDF_MEMORY_DESCRIPTOR__bindgen_ty_1__bindgen_ty_1 {
                            Buffer: ptr::from_mut(elem).cast(),
                            Length: size_to_ulong!(size_of::<T>()),
                        },
                },
            }
        }
    }

    /// A data type that is able to represent a fillable IOCTL WDF Response
    ///
    /// # Type Parameters
    /// - [`T`]: The type of the response payload, which must implement the [`Default`] trait.
    ///
    /// # Usage
    /// This struct is typically used to encapsulate the response data returned from ioctl operations
    /// in a structured and type-safe manner. The wrapped type `T` must have a `Default` implementation
    /// to ensure it can be initialized with default values.
    #[derive(Debug, Default)]
    pub struct Describe<T>(T);

    mod buff_input_io {
        use crate::descriptors::private::{operations, Describe};
        use crate::op::{AsBuff, AsBuilder, AsBuilderMut, AsNonNull, AsNonNullBuff, AsRawBuff, IsWdfType, ToNonNull};
        use core::ops::Deref;
        use core::ptr::NonNull;
        use crate::rt::wdk_sys::WDF_MEMORY_DESCRIPTOR;

        impl IsWdfType for WDF_MEMORY_DESCRIPTOR {}
        impl<T> Describe<T> {
            #[allow(dead_code)]
            pub fn new(data: T) -> Self {
                Self(data)
            }

            pub fn from_descriptor(desc: NonNull<WDF_MEMORY_DESCRIPTOR>) -> Option<Self> {
                let buff_len = unsafe { desc.as_ref().u.BufferType.Length };
                let expected_len = size_of::<T>() as u32;
                if buff_len != expected_len {
                    return None;
                }

                Some(Self(unsafe { core::ptr::read(desc.as_ref().u.BufferType.Buffer as *const T) }))
            }

            #[allow(dead_code)]
            pub fn into_inner(self) -> T {
                self.0
            }
        }

        impl<T> ToNonNull<T> for Describe<T> {}

        impl<T: Copy> AsNonNull<T> for Describe<T> {}

        impl<T:Copy> AsNonNullBuff<T> for Describe<T> {}

        impl<T> AsRef<T> for Describe<T> {
            fn as_ref(&self) -> &T {
                &self.0
            }
        }

        impl<T> AsRawBuff<T> for Describe<T> {}
        impl<T> AsBuilder for Describe<T> {
            type Descriptor<'a>
                = WDF_MEMORY_DESCRIPTOR
            where
                Self: 'a;

            #[inline]
            fn build(&self) -> Self::Descriptor<'_> {
                operations::build_for_data_type(self.as_ref())
            }
        }

        impl<T> AsMut<T> for Describe<T> {
            fn as_mut(&mut self) -> &mut T {
                &mut self.0
            }
        }
        impl<T> AsBuilderMut for Describe<T> {
            #[inline]
            fn build_mut(&mut self) -> Self::Descriptor<'_> {
                operations::build_for_data_type(self.as_mut())
            }
        }
        impl<T> Deref for Describe<T> {
            type Target = T;

            fn deref(&self) -> &Self::Target {
                self.as_ref()
            }
        }

    }
}
pub use private::{Describe};
