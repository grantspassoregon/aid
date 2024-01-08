//! The `error` module defines a library-specific [`Bandage`] alias for `Error`, and an alias for
//! Result, [`Clean`], using the `Error` alias.
use thiserror::Error;

/// The `Clean` type is an alias for `Result` using the library-defined [`Bandage`].
pub type Clean<T> = Result<T, Bandage>;

/// The `Bandage` enum is a library-specific error conversion.
#[derive(Error, Debug)]
pub enum Bandage {
    /// The `Auth` variant indicates an error occurred during the authorization process.
    #[error("Authorization failed.")]
    Auth,
    /// The `Env` variant represents error conversions from [`std::env::VarError`].
    #[error("Could not read environmental variables from .env.")]
    Env(#[from] std::env::VarError),
    /// The `FileName` variant indicates a malformed file name, from [`std::ffi::OsString`].
    #[error("Bad file name {0:?}.")]
    FileName(std::ffi::OsString),
    /// The `Int` variant represents error conversions from [`std::num::ParseIntError`],
    /// indicating a failure to parse an integer from a string.
    #[error("Could not parse integer from string.")]
    Int(#[from] std::num::ParseIntError),
    /// The `Io` variant represents error conversions from [`std::io::Error`].
    #[error("Input/output error from std.")]
    Io(#[from] std::io::Error),
    /// A `Parse` indicates an error occurred during parsing.
    #[error("Parse error.")]
    Parse,
    /// The `UserBuild` indicates an error occurred using a builder pattern.
    #[error("Value not provided for {value:?}.")]
    UserBuild {
        /// The `value` field returns messages on missing parameters in the builder function.
        value: Vec<String>,
    },
    #[cfg(feature = "icon")]
    #[cfg_attr(docsrs, doc(cfg(feature = "icon")))]
    #[error("Icon loading error.")]
    BadIcon(#[from] dioxus_desktop::tao::window::BadIcon),
    #[cfg(feature = "bin")]
    #[cfg_attr(docsrs, doc(cfg(feature = "bin")))]
    #[error("Could not serialize to binary.")]
    Bin(#[from] Box<bincode::ErrorKind>),
    /// Error returned by the byte_unit library
    #[cfg(feature = "byte")]
    #[cfg_attr(docsrs, doc(cfg(feature = "byte")))]
    #[error("Byte conversion failed.")]
    Byte(#[from] byte_unit::ParseError),
    #[cfg(feature = "csvs")]
    #[cfg_attr(docsrs, doc(cfg(feature = "csvs")))]
    #[error("Could not make CSV writer.")]
    Csv(#[from] csv::Error),
    #[cfg(feature = "req")]
    #[cfg_attr(docsrs, doc(cfg(feature = "req")))]
    #[error("HTTP request error.")]
    Http(#[from] reqwest::Error),
    #[cfg(feature = "img")]
    #[cfg_attr(docsrs, doc(cfg(feature = "img")))]
    #[error("Image processing error.")]
    Image(#[from] image::error::ImageError),
    #[cfg(feature = "oauth")]
    #[cfg_attr(docsrs, doc(cfg(feature = "oauth")))]
    #[error("Oauth2 error.")]
    Oauth2(#[from] oauth2::RequestTokenError<oauth2::reqwest::Error<reqwest::Error>,
    oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>>),
    /// The `Serialize` variant converts errors from the `serde` crate.
    #[cfg(feature = "serial")]
    #[cfg_attr(docsrs, doc(cfg(feature = "serial")))]
    #[error("Deserialize error.")]
    Serialize(#[from] serde::de::value::Error),
    #[cfg(feature = "serial")]
    #[cfg_attr(docsrs, doc(cfg(feature = "serial")))]
    #[error("Deserialization error.")]
    Conversion(#[from] serde_json::Error),
    #[cfg(feature = "uuids")]
    #[cfg_attr(docsrs, doc(cfg(feature = "uuids")))]
    #[error("Uuid conversion failed.")]
    Uuid(#[from] uuid::Error),
    #[cfg(feature = "urls")]
    #[cfg_attr(docsrs, doc(cfg(feature = "urls")))]
    #[error("Parse error.")]
    Url(#[from] url::ParseError),
    #[cfg(feature = "plot")]
    #[cfg_attr(docsrs, doc(cfg(feature = "plot")))]
    #[error("Plotting backend error.")]
    BitMap(#[from] plotters_bitmap::BitMapBackendError),
    #[cfg(feature = "plot")]
    #[cfg_attr(docsrs, doc(cfg(feature = "plot")))]
    #[error("Plotting drawing error.")]
    Plot(#[from] plotters::drawing::DrawingAreaErrorKind<plotters_bitmap::BitMapBackendError>),
    /// The `Unknown` variant is a catch-all error variant for library operations.
    #[error("Unexpected error.")]
    Unknown,
}
