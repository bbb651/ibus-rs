#![cfg_attr(docsrs, feature(doc_cfg))]

/// No-op.
macro_rules! skip_assert_initialized {
    () => {};
}

/// Asserts that this is the main thread and either `ibus::init` has been called.
macro_rules! assert_initialized_main_thread {
    () => {
        // if !::ibus::is_initialized_main_thread() {
        //     if ::ibus::is_initialized() {
        //         panic!("IBus may only be used from the main thread.");
        //     } else {
        //         panic!("IBus has not been initialized. Call `ibus::init` first.");
        //     }
        // }
    };
}

pub use auto::*;
mod auto;

pub mod functions {
    pub use super::auto::functions::*;
}
