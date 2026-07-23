#![cfg_attr(feature = "kmdf-runtime", no_std)]
/*#![feature(
    //trait_alias,
    //lazy_type_alias,
    //associated_type_defaults,
    //min_specialization,
    //generic_const_exprs,
    //type_alias_impl_trait,
    //negative_impls
    //impl_trait_in_assoc_type,
)]*/
#[cfg(all(not(feature = "test-runtime"), not(feature = "kmdf-runtime")))]
compile_error!(
    "Multiple runtime behavior selected. Select only one! ('kmdf-runtime', 'test-runtime')"
);

#[cfg(all(feature = "test-runtime", feature = "kmdf-runtime"))]
compile_error!(
    "Select a valid runtime behavior for swdk. ('kmdf-runtime', 'test-runtime')"
);

pub extern crate alloc;

mod runtime;

pub(crate) mod wdf;

pub mod rt {
    #[cfg(feature = "kmdf-runtime")]
    mod __kmdf {
        pub use wdk;
        pub use wdk_sys;
        pub use wdk_alloc;

        pub extern crate wdk_panic;

        #[cfg(feature = "kmdf-runtime")]
        pub use crate::runtime::logging;

        pub use crate::runtime::utils;

        pub(crate) use crate::runtime::kmdf;
    }

    #[cfg(feature = "kmdf-runtime")]
    pub use __kmdf::*;

    #[cfg(feature = "test-runtime")]
    pub use crate::runtime::test::*;
}

#[doc(hidden)]
pub use paste::paste as __swdk_paste;

#[cfg(feature = "kmdf-runtime")]
pub use wdk::println;

mod __public_api {
    pub use crate::wdf::handle::*;

    pub use crate::wdf::values as val;
    pub use crate::wdf::generators as gens;
    pub use crate::wdf::context as ctx;
    pub use crate::wdf::builders as bd;
    pub use crate::wdf::operators as op;
    
    pub use crate::wdf::ioctl;
}

pub use __public_api::*;