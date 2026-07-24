mod _concepts {
    #[cfg(feature = "test-runtime")]
    use crate::rt::test_rt::*;

    use wdk_sys::NTSTATUS;

    pub type NtResult<T = ()> = Result<T, NTSTATUS>;
}
mod _operators {
    #[cfg(feature = "test-runtime")]
    use crate::rt::test_rt::*;

    use crate::bd::WdfObjAttrs;
    use crate::ctx::WdfCtxNoneDesc;
    use crate::{Handle, HandleMut, HandleRef};
    use crate::op::_concepts::NtResult;
    use alloc::vec::Vec;
    use core::borrow::Borrow;
    use core::ops::Deref;
    use core::ptr;
    use wdk_sys::PCWDF_OBJECT_CONTEXT_TYPE_INFO;

    pub trait AsPtr<T>: AsRef<T> {
        #[inline]
        unsafe fn as_ptr(&self) -> *const T {
            ptr::from_ref(self.as_ref())
        }

        #[inline]
        fn with_ptr<F>(&self, f: F)
                       where
                           F: FnOnce(*const T),
        {
            f(unsafe { self.as_ptr() })
        }
    }

    pub trait AsPtrMut<T>: AsPtr<T> + AsMut<T> {
        /// Returns a mutable raw pointer to `T`.
        ///
        /// # Safety
        ///
        /// The borrow checker does not track the returned raw pointer.
        ///
        /// The caller must ensure that:
        ///
        /// - The pointer does not outlive `self`;
        /// - No aliasing violations are introduced;
        /// - No other mutable access is performed while the pointer is in use;
        /// - The pointer is not dereferenced after the underlying object becomes invalid.
        ///
        /// Consider using [`with_ptr_mut`] whenever possible, as it confines the
        /// lifetime of the raw pointer to the provided closure.
        #[inline]
        unsafe fn as_ptr_mut(&mut self) -> *mut T {
            ptr::from_mut(self.as_mut())
        }

        #[inline]
        fn with_ptr_mut<F>(&mut self, f: F)
                           where
                               F: FnOnce(*mut T),
        {
            f(unsafe { self.as_ptr_mut() })
        }
    }

    pub trait AsRawWdf<T: Copy>: AsRef<T> {
        fn as_raw(&self) -> T {
            *self.as_ref()
        }
    }

    pub trait AsBuff<T>: AsRef<T> {
        /// Do something with `*const Self::Target` using `f`
        /// # Example
        /// ```
        /// my_elem.with_ptr(|ptr| ptr.read());
        /// ```
        #[inline]
        fn with_buff<F>(&self, f: F)
                        where
                            F: FnOnce(&[u8]),
        {
            f(self.as_buff())
        }

        /// Get a pointer to `*const Self::Target`
        #[inline]
        fn as_buff(&self) -> &[u8] {
            unsafe {
                core::slice::from_raw_parts(
                    ptr::from_ref(self.as_ref()).cast::<u8>(),
                    size_of::<T>(),
                )
            }
        }
    }

    pub trait AsBuffMut<T>: Default + AsMut<T> {
        fn with_buff_mut<F>(&mut self, f: F)
                            where
                                F: FnOnce(&mut [u8]),
        {
            f(self.as_buff_mut())
        }

        fn as_buff_mut(&mut self) -> &mut [u8] {
            unsafe {
                // SAFETY: buff cannot be null in this trait
                core::slice::from_raw_parts_mut(
                    ptr::from_mut(self.as_mut()).cast::<u8>(),
                    size_of::<T>(),
                )
            }
        }
    }

    pub trait AsBuilder {
        type Descriptor<'a>
            where
                Self: 'a;

        #[must_use]
        fn build(&self) -> Self::Descriptor<'_>;
    }

    pub trait AsBuilderMut: AsBuilder {
        fn build_mut(&mut self) -> Self::Descriptor<'_>;
    }

    pub trait AsMappableBuff<T>: AsBuff<T> {
        type Descriptor<'a>
            where
                Self: 'a;

        fn map<U>(&self, f: impl FnMut(&u8) -> U) -> Vec<U> {
            self.as_buff().iter().map(f).collect()
        }
    }

    pub trait AsOptionalBuff<T>: AsRef<Option<T>> {
        /// Permit doing something with the optional buffer
        #[inline]
        fn with_buff<F>(&self, f: F)
                        where
                            F: FnOnce(Option<&[u8]>),
        {
            f(self.as_buff())
        }
        /// Get a const pointer to the optional buffer
        fn as_buff(&self) -> Option<&[u8]> {
            self.as_ref().as_ref().map(|value| unsafe {
                core::slice::from_raw_parts(
                    ptr::from_ref(value).cast::<u8>(),
                    size_of::<T>(),
                )
            })
        }
    }

    pub trait AsOptionalBuffMut<T>: AsOptionalBuff<T> + AsMut<Option<T>> {
        /// Permit doing something with the optional buffer
        #[inline]
        fn with_buff_mut<F>(&mut self, f: F)
                            where
                                F: FnOnce(Option<&mut [u8]>),
        {
            f(self.as_buff_mut())
        }
        /// Get a mut to the optional buffer
        fn as_buff_mut(&mut self) -> Option<&mut [u8]> {
            self.as_mut().as_mut().map(|value| unsafe {
                core::slice::from_raw_parts_mut(
                    ptr::from_mut(value).cast::<u8>(),
                    size_of::<T>(),
                )
            })
        }
    }

    pub trait AsBuffPtr<T>: AsPtr<T> + AsBuff<T> {
        /// Get a `const` ptr to the buffer
        #[inline]
        unsafe fn as_buff(&self) -> *const T {
            unsafe { self.as_ptr() }
        }
    }

    pub trait AsBuffPtrMut<T>: AsBuffPtr<T> + AsBuffMut<T> + AsPtrMut<T> {
        /// Get a mutable ptr to the buffer
        #[inline]
        fn as_buff_mut(&mut self) -> &mut T {
            unsafe { &mut *self.as_ptr_mut() }
        }
    }

    pub trait AsCtxDesc: Sized + Default {
        #[allow(unused_variables)]
        fn wdf_get<O>(
            obj: HandleRef<'_, O>,
        ) -> Option<HandleRef<'_, Self>> {
            None
        }
        fn unique() -> Option<PCWDF_OBJECT_CONTEXT_TYPE_INFO> {
            None
        }
        #[allow(unused_variables)]
        fn wdf_get_mut<O>(
            obj: HandleRef<'_, O>,
        ) -> Option<HandleMut<'_, Self>> {
            None
        }
        fn wdf_type_name() -> Option<&'static str> {
            None
        }
    }

    pub trait AsCtx<O, C: AsCtxDesc>: Sized + Deref {
        /// Get a pointer to the allocated kernel context object
        fn ctx(&self) -> Option<Handle<*const C>>;

        /// Get a mutable pointer to the allocated kernel context object
        fn ctx_mut(&mut self) -> Option<*mut C> {
            let ctx = self.ctx()?;
            Some(ctx.deref().cast_mut())
        }

        /// Get the WDF object type name
        fn wdf_type_name(&self) -> Option<&'static str> {
            C::wdf_type_name()
        }

        /// Get the unique WDF object context type info
        unsafe fn unique(
            &self,
        ) -> Option<PCWDF_OBJECT_CONTEXT_TYPE_INFO> {
            C::unique()
        }
    }

    pub trait AsWdfOwner<O>: Sized + AsPtr<O> + AsRef<O> {
        type Conf;
        type Owned;

        /// Create a WDF object of type [`O`] and materialize it as `Self`.
        ///
        /// # Parameters
        /// * `owner` - A reference to the WDF object required to create the new object, if any.
        /// * `attrs` - Optional WDF object attributes.
        /// * `conf` - The WDF configuration used during creation.
        fn from_owned_with_attrs<D>(
            owned: Self::Owned,
            conf: Self::Conf,
            attrs: Option<WdfObjAttrs<D>>,
        ) -> NtResult<Self>
        where
            D: AsCtxDescriptor;

        /// Create a WDF object of type [`O`] and materialize it as `Self`.
        ///
        /// # Parameters
        /// * `owner` - A reference to the WDF object required to create the new object, if any.
        /// * `conf` - The WDF configuration used during creation.
        fn from_owned(
            owned: Self::Owned,
            conf: Self::Conf,
        ) -> NtResult<Self> {
            Self::from_owned_with_attrs::<WdfCtxNoneDesc>(
                owned, conf, None,
            )
        }
    }

    pub trait AsWdfOwned<O>: Sized + AsPtr<O> + AsRef<O> {
        type Owner;
        fn from_owner(
            owner: &Self::Owner,
        ) -> NtResult<Self>;
    }

    pub trait IntoInner<T>: AsRef<T> {
        fn into_inner(self) -> T;
    }

    pub trait AsRaw<T: Copy>: AsRef<T> {
        fn raw(&self) -> T {
            *self.as_ref()
        }
    }

    pub trait AsRawWithBorrow<T: Copy>: Borrow<T> {
        fn raw_with_borrow(&self) -> T {
            *self.borrow()
        }
    }

    /// Marker trait implemented by context descriptors that represent
    /// the absence of a context object.
    ///
    /// # Warning
    /// This trait is not intended to be implemented by user code.
    ///
    /// It is used internally by [`WdfNoneCtxUnique`] to describe WDF
    /// objects with no associated context.
    pub trait AsNoneCtxUnique: AsUnique {}

    pub trait AsNoneCtxDesc<O = ()>: AsCtxDescriptor {}

    /// Describe a generic data type that is able to expose an handle
    /// to a static instance of `WDF_OBJECT_CONTEXT_TYPE_INFO`
    /// # Note
    /// `WDF_OBJECT_CONTEXT_TYPE_INFO` is the data structure that WDM use internally
    /// to describe a context type object. WDM uses this struct to have the necessary
    /// information about the context type that needs to be allocated inside the kernel.
    ///
    /// # Safety
    /// This trait is concerned with implementing a handler to one of the static [`WDF_OBJECT_CONTEXT_TYPE_INFO`]
    /// instances that WDM internally uses to describe a kernel-allocated context object.
    /// Since the [`WDF_OBJECT_CONTEXT_TYPE_INFO`] need to be addressed as a static element,
    /// the type that implements this type needs to be sure that [`AsUnique::unique()`]
    /// point to a valid static instance of [`WDF_OBJECT_CONTEXT_TYPE_INFO`]
    ///
    /// # Implementors
    /// This trait is typically implemented by the [`swdf_declare_context_handle!`].
    /// See [`swdf_declare_context_handle!`] examples for more information
    pub unsafe trait AsUnique {
        /// Get the unique type ptr used to describe the context type in `WDF`
        unsafe fn unique(
            &self,
        ) -> PCWDF_OBJECT_CONTEXT_TYPE_INFO;
    }
}

pub use _concepts::NtResult;
pub use _operators::*;