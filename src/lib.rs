#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
//! An error-handling library for bubbling up errors in library code.

/// The `error` module holds aliases for `Result` and `Error`.
pub mod error;

/// The `prelude` module exports library types intended for public use.
pub mod prelude {
    pub use crate::error::{Bandage, Clean};
}
