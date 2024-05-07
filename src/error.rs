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
    /// The `Utf8` variant converts a `std::str::Utf8Error`.
    #[error("Utf8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),
    /// The `Hint` variant encloses a message with an error.
    #[error("Hint: {0}")]
    Hint(String),
    /// The `Unknown` variant is a catch-all error variant for library operations.
    #[error("Unexpected error.")]
    Unknown,
    /// The `BadIcon` results from a failed import of an icon image file into the Dioxus desktop
    /// app.
    #[cfg(feature = "icon")]
    #[cfg_attr(docsrs, doc(cfg(feature = "icon")))]
    #[error("Icon loading error.")]
    BadIcon(#[from] dioxus_desktop::tao::window::BadIcon),
    /// The `Bin` variant indicates a failure during binary encoding in crate `bincode`.
    #[cfg(feature = "bin")]
    #[cfg_attr(docsrs, doc(cfg(feature = "bin")))]
    #[error("Could not serialize to binary.")]
    Bin(#[from] Box<bincode::ErrorKind>),
    /// Error returned by the byte_unit library
    #[cfg(feature = "byte")]
    #[cfg_attr(docsrs, doc(cfg(feature = "byte")))]
    #[error("Byte conversion failed.")]
    Byte(#[from] byte_unit::ParseError),
    /// The `Csv` variant converts an error returned by the `csv` crate. Currently indicates
    /// failure to generate a csv writer.
    #[cfg(feature = "csvs")]
    #[cfg_attr(docsrs, doc(cfg(feature = "csvs")))]
    #[error("Could not make CSV writer.")]
    Csv(#[from] csv::Error),
    /// The `Http` variant converts an error from the `reqwest` crate.
    #[cfg(feature = "req")]
    #[cfg_attr(docsrs, doc(cfg(feature = "req")))]
    #[error("HTTP request error.")]
    Http(#[from] reqwest::Error),
    /// The `Image` variant converts an error from the `image` crate.
    #[cfg(feature = "img")]
    #[cfg_attr(docsrs, doc(cfg(feature = "img")))]
    #[error("Image processing error.")]
    Image(#[from] image::error::ImageError),
    /// The `Oauth2` variant converts an error from the `oauth2` crate.
    #[cfg(feature = "oauth")]
    #[cfg_attr(docsrs, doc(cfg(feature = "oauth")))]
    #[error("Oauth2 error.")]
    Oauth2(
        #[from]
        oauth2::RequestTokenError<
            oauth2::reqwest::Error<reqwest::Error>,
            oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>,
        >,
    ),
    /// The `Serialize` variant converts errors from the `serde` crate.
    #[cfg(feature = "serial")]
    #[cfg_attr(docsrs, doc(cfg(feature = "serial")))]
    #[error("Deserialize error.")]
    Serialize(#[from] serde::de::value::Error),
    /// The `SerdeJson` variant converts an error from the `serde_json` crate.
    #[cfg(feature = "serial")]
    #[cfg_attr(docsrs, doc(cfg(feature = "serial")))]
    #[error("Deserialization error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    /// The `Sqlx` variant converts a general error from the `sqlx` crate.
    #[cfg(feature = "sql")]
    #[cfg_attr(docsrs, doc(cfg(feature = "sql")))]
    #[error("Sqlx command error: {0}")]
    Sqlx(#[from] sqlx::Error),
    /// The `Migrate` variant converts a migration error from the `sqlx` crate.
    #[cfg(feature = "sql")]
    #[cfg_attr(docsrs, doc(cfg(feature = "sql")))]
    #[error("Sqlx migration error: {0}")]
    Migrate(#[from] sqlx::migrate::MigrateError),
    /// The `Uuid` variant converts an error from the `uuid` crate.
    #[cfg(feature = "id")]
    #[cfg_attr(docsrs, doc(cfg(feature = "id")))]
    #[error("Uuid conversion failed.")]
    Uuid(#[from] uuid::Error),
    /// The `Url` variant converts an error from the `url` crate.
    #[cfg(feature = "urls")]
    #[cfg_attr(docsrs, doc(cfg(feature = "urls")))]
    #[error("Parse error.")]
    Url(#[from] url::ParseError),
    /// The `BitMap` variant converts an error from the `plotters_bitmap` crate.
    #[cfg(feature = "plot")]
    #[cfg_attr(docsrs, doc(cfg(feature = "plot")))]
    #[error("Plotting backend error.")]
    BitMap(#[from] plotters_bitmap::BitMapBackendError),
    /// The `Plot` variant converts an error from the `plotters` crate.
    #[cfg(feature = "plot")]
    #[cfg_attr(docsrs, doc(cfg(feature = "plot")))]
    #[error("Plotting drawing error.")]
    Plot(#[from] plotters::drawing::DrawingAreaErrorKind<plotters_bitmap::BitMapBackendError>),
    /// The `GeoJson` variant converts an error from the `geojson` crate.
    #[cfg(feature = "gis")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gis")))]
    #[error("Error reading geojson file: {0}")]
    GeoJson(#[from] geojson::Error),
    /// The `TraceInit` variant converts an error from the `tracing_subscriber` crate.
    #[cfg(feature = "trace")]
    #[cfg_attr(docsrs, doc(cfg(feature = "trace")))]
    #[error("Problem initializing subscriber: {0}")]
    TraceInit(#[from] tracing_subscriber::util::TryInitError),
    /// The `Axum` variants converts an *axum::Error* from the `axum` crate.
    #[cfg(feature = "route")]
    #[cfg_attr(docsrs, doc(cfg(feature = "route")))]
    #[error("Axum error: {0}")]
    Axum(#[from] axum::Error),
    /// The `AxumHttp` variant converts an axum::http error from the `axum` crate.
    #[cfg(feature = "route")]
    #[cfg_attr(docsrs, doc(cfg(feature = "route")))]
    #[error("Axum http error: {0}")]
    AxumHttp(#[from] axum::http::Error),
    /// The `Hyper` variant converts a `hyper_util::error::Error` from the `hyper_util` crate.
    #[cfg(feature = "hype")]
    #[cfg_attr(docsrs, doc(cfg(feature = "hype")))]
    #[error("Legacy client error: {0}")]
    Hyper(#[from] hyper::Error),
    /// The `HyperUtil` variant converts a `hyper_util::client::legacy::Error` from the `hyper_util` crate.
    #[cfg(feature = "hype")]
    #[cfg_attr(docsrs, doc(cfg(feature = "hype")))]
    #[error("Legacy client error: {0}")]
    HyperUtil(#[from] hyper_util::client::legacy::Error),
    /// The `HyperUtil` variant converts a `hyper_util::client::legacy::Error` from the `hyper_util` crate.
    #[cfg(feature = "gis")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gis")))]
    #[error("Shapefile error: {0}")]
    Shapefile(#[from] shapefile::Error),
    /// The `EventLoop` variant converts a [`winit::error::EventLoopError`] from the `winit` crate.
    #[cfg(feature = "win")]
    #[cfg_attr(docsrs, doc(cfg(feature = "win")))]
    #[error("Winit event loop error: {0}")]
    EventLoop(#[from] winit::error::EventLoopError),
    /// The `WinOs` variant converts a [`winit::error::OsError`] from the `winit` crate.
    #[cfg(feature = "win")]
    #[cfg_attr(docsrs, doc(cfg(feature = "win")))]
    #[error("OS error: {0}")]
    WinOs(#[from] winit::error::OsError),
    /// The `WinitIcon` variant converts a [`winit::window::BadIcon`] from the `winit` crate.
    #[cfg(feature = "win")]
    #[cfg_attr(docsrs, doc(cfg(feature = "win")))]
    #[error("Winit icon error: {0}")]
    WinitIcon(#[from] winit::window::BadIcon),
    /// The `WgpuSurface` variant converts a [`wgpu::CreateSurfaceError`] from the `wgpu` crate.
    #[cfg(feature = "gpu")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gpu")))]
    #[error("From wgpu: {0}")]
    WgpuSurface(#[from] wgpu::CreateSurfaceError),
    /// The `WgpuDevice` variant converts a [`wgpu::RequestDeviceError`] from the `wgpu` crate.
    #[cfg(feature = "gpu")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gpu")))]
    #[error("From wgpu: {0}")]
    WgpuDevice(#[from] wgpu::RequestDeviceError),
}

#[cfg(feature = "gis")]
#[cfg_attr(docsrs, doc(cfg(feature = "gis")))]
impl<T> From<std::sync::PoisonError<T>> for Bandage {
    fn from(input: std::sync::PoisonError<T>) -> Self {
        let msg = format!("{}", input.to_string());
        Self::Hint(msg)
    }
}
