#![cfg_attr(docsrs, feature(doc_cfg))]
pub mod error;

pub mod prelude {
    pub use crate::error::{Clean, Bandage};
}
