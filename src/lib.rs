#![cfg_attr(feature = "wdk-runtime", no_std)]
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
#[cfg(any(
    all(not(feature = "test-runtime"), not(feature = "wdk-runtime")),
    all(feature = "test-runtime", feature = "wdk-runtime"),
))]
compile_error!(
    "No runtime selected. Enable either `wdk-runtime` or `test-runtime`"
);
pub extern crate alloc;

mod runtime;
mod wdf;
pub mod rt {
    pub use crate::runtime::*;

    #[cfg(feature = "wdk-runtime")]
    mod wdk_runtime {
        pub use wdk;
        pub use wdk_alloc::WdkAllocator;
        pub use wdk_sys;

        pub extern crate wdk_panic;
    }

    #[cfg(feature = "wdk-runtime")]
    pub use wdk_runtime::*;
}

#[cfg(feature = "wdk-runtime")]
#[doc(hidden)]
pub use paste::paste as __swdk_paste;
pub use wdf::*;
pub use wdf::handle::*;
#[cfg(feature = "wdk-runtime")]
pub use wdk::println;
