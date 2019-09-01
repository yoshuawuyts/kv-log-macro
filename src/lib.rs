//! Log macro for log's kv-unstable backend.
//!
//! ## Example
//!
//! ```rust
//! ```

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, missing_doc_code_examples, unreachable_pub)]
// #![cfg_attr(test, deny(warnings))]

use log::{Level, LevelFilter, Record, logger};

use std::fmt;

// WARNING: this is not part of the crate's public API and is subject to change at any time
#[doc(hidden)]
pub fn __private_api_log(
    args: fmt::Arguments<'_>,
    level: Level,
    &(target, module_path, file, line): &(&str, &'static str, &'static str, u32),
    kvs: Option<&[(&str, &str)]>
) {

    // Ideally there would be a `From` impl available for this.
    struct KeyValues<'a> {
        inner: &'a [(&'a str, &'a str)],
    }

    impl<'a> log::kv::Source for KeyValues<'a> {
        fn visit<'kvs>(
            &'kvs self,
            visitor: &mut dyn log::kv::Visitor<'kvs>,
        ) -> Result<(), log::kv::Error> {
            for pair in self.inner {
                visitor.visit_pair(pair.0.into(), pair.1.into())?;
            }
            Ok(())
        }

        #[inline]
        fn count(&self) -> usize {
            self.inner.len()
        }
    }

    let kvs = match kvs {
        Some(kvs) => Some(KeyValues { inner: kvs }),
        None => None,
    };

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

//#[allow(dead_code)]
//#[inline]
//fn max_level() -> LevelFilter {
//    log::max_level()
//}

///// The standard logging macro.
/////
///// ```
///// use kv_log_macro::log;
///// use log::LogLevel;
/////
///// log!(LogLevel::Info, "hello {}", "cats", {
/////   cat1: "chashu",
/////   cat2: "nori",
///// });
///// ```
//#[macro_export(local_inner_macros)]
//macro_rules! log {
//    ($lvl:expr, $($arg:tt)+, { $($fields:tt)* }) => ({
//        let lvl = $lvl;
//        if lvl <= $crate::STATIC_MAX_LEVEL && lvl <= $crate::max_level() {
//            $crate::__private_api_log(
//                format_args!($($arg)+),
//                lvl,
//                &(module_path!(), module_path!(), file!(), line!()),
//            );
//        }
//    });
//}

/// Logs a message at the info level.
///
/// ```
/// use kv_log_macro::info;
///
/// info!("hello");
/// info!("hello",);
/// info!("hello {}", "cats");
/// info!("hello {}", "cats",);
/// info!("hello {}", "cats", {
///     cat_1: "chashu",
///     cat_2: "nori",
/// });
/// ```
#[macro_export]
macro_rules! info {
    // info!("...")
    ($e:expr) => {
        $crate::info_impl!(($e));
    };

    // info!("...", args...)
    ($e:expr, $($rest:tt)*) => {
        $crate::info_impl!(($e) $($rest)*);
    };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! info_impl {
    // End of macro input
    (($($arg:expr),*)) => {
        let lvl = log::Level::Info;
        $crate::__private_api_log(
            __log_format_args!($($arg),*),
            lvl,
            &(__log_module_path!(), __log_module_path!(), __log_file!(), __log_line!()),
            None,
        );
    };

    // // Trailing k-v pairs containing no trailing comma
    (($($arg:expr),*) { $($key:ident : $value:expr),* }) => {
        let lvl = log::Level::Info;
        $crate::__private_api_log(
            __log_format_args!($($arg),*),
            lvl,
            &(__log_module_path!(), __log_module_path!(), __log_file!(), __log_line!()),
            Some(&[$((__log_stringify!($key), $value)),*])
        );
    };

    // Trailing k-v pairs with trailing comma
    (($($e:expr),*) { $($key:ident : $value:expr,)* }) => {
        $crate::info_impl!(($($e),*) { $($key : $value),* });
    };

    // Last expression arg with no trailing comma
    (($($e:expr),*) $arg:expr) => {
        $crate::info_impl!(($($e,)* $arg));
    };

    // Expression arg
    (($($e:expr),*) $arg:expr, $($rest:tt)*) => {
        $crate::info_impl!(($($e,)* $arg) $($rest)*);
    };
}

// /// Logs a message at the trace level.
// #[macro_export(local_inner_macros)]
// macro_rules! trace {
//     ($($arg:tt)+) => (
//         log!($crate::Level::Trace, $($arg)+);
//     )
// }

// /// Logs a message at the debug level.
// #[macro_export(local_inner_macros)]
// macro_rules! debug {
//     ($($arg:tt)+) => (
//         log!($crate::Level::Debug, $($arg)+);
//     )
// }

// /// Logs a message at the info level.
// #[macro_export(local_inner_macros)]
// macro_rules! info {
//     ($($arg:tt)+) => (
//         log!($crate::Level::Info, $($arg)+);
//     )
// }

// /// Logs a message at the warn level.
// #[macro_export(local_inner_macros)]
// macro_rules! warn {
//     ($($arg:tt)+) => (
//         log!($crate::Level::Warn, $($arg)+);
//     )
// }

// /// Logs a message at the error level.
// #[macro_export(local_inner_macros)]
// macro_rules! error {
//     ($($arg:tt)+) => (
//         log!($crate::Level::Error, $($arg)+);
//     )
// }

/// Determines if a message logged at the specified level in that module will
/// be logged.
///
/// This can be used to avoid expensive computation of log message arguments if
/// the message would be ignored anyway.
///
/// # Examples
///
/// ```edition2018
/// use log::Level::Debug;
/// use log::{debug, log_enabled};
///
/// # fn foo() {
/// if log_enabled!(Debug) {
///     let data = expensive_call();
///     debug!("expensive debug data: {} {}", data.x, data.y);
/// }
/// if log_enabled!(target: "Global", Debug) {
///    let data = expensive_call();
///    debug!(target: "Global", "expensive debug data: {} {}", data.x, data.y);
/// }
/// # }
/// # struct Data { x: u32, y: u32 }
/// # fn expensive_call() -> Data { Data { x: 0, y: 0 } }
/// # fn main() {}
/// ```
#[macro_export(local_inner_macros)]
macro_rules! log_enabled {
    (target: $target:expr, $lvl:expr) => {{
        let lvl = $lvl;
        lvl <= $crate::STATIC_MAX_LEVEL
            && lvl <= $crate::max_level()
            && $crate::__private_api_enabled(lvl, $target)
    }};
    ($lvl:expr) => {
        log_enabled!(target: __log_module_path!(), $lvl)
    };
}

// The log macro above cannot invoke format_args directly because it uses
// local_inner_macros. A format_args invocation there would resolve to
// $crate::format_args which does not exist. Instead invoke format_args here
// outside of local_inner_macros so that it resolves (probably) to
// core::format_args or std::format_args. Same for the several macros that
// follow.
//
// This is a workaround until we drop support for pre-1.30 compilers. At that
// point we can remove use of local_inner_macros, use $crate:: when invoking
// local macros, and invoke format_args directly.
#[doc(hidden)]
#[macro_export]
macro_rules! __log_format_args {
    ($($args:tt)*) => {
        format_args!($($args)*)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_module_path {
    () => {
        module_path!()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_file {
    () => {
        file!()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_line {
    () => {
        line!()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __log_stringify {
    ($($args:tt)*) => {
        stringify!($($args)*)
    };
}
