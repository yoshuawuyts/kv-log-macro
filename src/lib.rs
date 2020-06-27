//! Log macro for log's kv-unstable backend.
//!
//! ## Example
//!
//! ```rust
//! use kv_log_macro::info;
//!
//! femme::start(log::LevelFilter::Info).unwrap();
//!
//! info!("hello");
//! info!("hello",);
//! info!("hello {}", "cats");
//! info!("hello {}", "cats",);
//! info!("hello {}", "cats", {
//!     cat_1: "chashu",
//!     cat_2: "nori",
//! });
//! ```

use log::{logger, Level, Record};
use std::fmt;

// WARNING: this is not part of the crate's public API and is subject to change at any time
#[doc(hidden)]
fn __private_api_log(
    args: fmt::Arguments<'_>,
    level: Level,
    &(target, module_path, file, line): &(&str, &'static str, &'static str, u32),
    kvs: Option<&[(&str, &dyn log::kv::ToValue)]>,
) {
    logger().log(
        &Record::builder()
            .args(args)
            .level(level)
            .target(target)
            .module_path_static(Some(module_path))
            .file_static(Some(file))
            .line(Some(line))
            .key_values(&kvs)
            .build(),
    );
}

use proc_macro_hack::proc_macro_hack;

/// Log info
#[proc_macro_hack]
pub use kv_log_macro_impl::log;

/// Log info
#[proc_macro_hack]
pub use kv_log_macro_impl::info;
