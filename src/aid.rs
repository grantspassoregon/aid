//! The `aid` module defines a library-specific [`Bandage`] alias for `Error`, and an alias for
//! Result, [`Clean`], using the `Error` alias.
use std::error::Error;

/// The `Clean` type is an alias for `Result` using the library-defined [`Bandage`].
pub type Clean<T> = Result<T, Bandage>;

/// The `Bandage` enum is a library-specific error conversion.
// #[derive(Debug, derive_more::Error, derive_more::Display, derive_more::From)]
#[derive(Debug, derive_more::Error, derive_more::Display, derive_more::From)]
pub enum Bandage {
    /// The `Auth` variant indicates an error occurred during the authorization process.
    #[display("Authorization failed: {:?}", self.source())]
    Auth,
    /// The `Env` variant represents error conversions from [`std::env::VarError`].
    #[display("Could not read environmental variables from .env: {:?}", self.source())]
    #[from(std::env::VarError)]
    Env,
    /// The `FileName` variant indicates a malformed file name, from [`std::ffi::OsString`].
    #[display("Bad file name {:?}.", self.source())]
    #[from(std::ffi::OsString)]
    FileName,
    /// The `Int` variant represents error conversions from [`std::num::ParseIntError`],
    /// indicating a failure to parse an integer from a string.
    #[display("Could not parse integer from string: {:?}", self.source())]
    #[from(std::num::ParseIntError)]
    Int,
    /// The `Io` variant represents error conversions from [`std::io::Error`].
    #[display("Input/output error from std: {:?}", self.source())]
    Io(#[from] std::io::Error),
    /// A `Parse` indicates an error occurred during parsing.
    #[display("Parse error.")]
    Parse,
    /// The `UserBuild` indicates an error occurred using a builder pattern.
    #[display("Value not provided for {value:?}.")]
    UserBuild {
        /// The `value` field returns messages on missing parameters in the builder function.
        value: Vec<String>,
    },
    /// The `Utf8` variant converts a `std::str::Utf8Error`.
    #[display("Utf8 error: {:?}", self.source())]
    Utf8(#[from] std::str::Utf8Error),
    ///// The `Hint` variant encloses a message with an error.
    //#[display("Hint: {:?}", self.source())]
    //Hint(String),
    /// The `Unknown` variant is a catch-all error variant for library operations.
    #[display("Unexpected error.")]
    Unknown,
    /// The `BadIcon` results from a failed import of an icon image file into the Dioxus desktop
    /// app.
    #[cfg(feature = "icon")]
    #[cfg_attr(docsrs, doc(cfg(feature = "icon")))]
    #[display("Icon loading error: {:?}", self.source())]
    BadIcon(#[from] dioxus_desktop::tao::window::BadIcon),
    /// The `Bin` variant indicates a failure during binary encoding in crate `bincode`.
    #[cfg(feature = "bin")]
    #[cfg_attr(docsrs, doc(cfg(feature = "bin")))]
    #[display("Could not serialize to binary: {:?}", self.source())]
    Bin(#[from] Box<bincode::ErrorKind>),
    /// Error returned by the byte_unit library
    #[cfg(feature = "byte")]
    #[cfg_attr(docsrs, doc(cfg(feature = "byte")))]
    #[display("Byte conversion failed: {:?}", self.source())]
    Byte(#[from] byte_unit::ParseError),
    /// The `Csv` variant converts an error returned by the `csv` crate. Currently indicates
    /// failure to generate a csv writer.
    #[cfg(feature = "csvs")]
    #[cfg_attr(docsrs, doc(cfg(feature = "csvs")))]
    #[display("Could not make CSV writer: {:?}", self.source())]
    Csv(#[from] csv::Error),
    /// The `Http` variant converts an error from the `reqwest` crate.
    #[cfg(feature = "req")]
    #[cfg_attr(docsrs, doc(cfg(feature = "req")))]
    #[display("HTTP request error: {:?}", self.source())]
    Http(#[from] reqwest::Error),
    /// The `Image` variant converts an error from the `image` crate.
    #[cfg(feature = "img")]
    #[cfg_attr(docsrs, doc(cfg(feature = "img")))]
    #[display("Image processing error: {:?}", self.source())]
    Image(#[from] image::error::ImageError),
    /// The `Oauth2` variant converts an error from the `oauth2` crate.
    #[cfg(feature = "oauth")]
    #[cfg_attr(docsrs, doc(cfg(feature = "oauth")))]
    #[display("Oauth2 error: {:?}", self.source())]
    Oauth2(
        #[from]
        oauth2::RequestTokenError<
            oauth2::reqwest::Error,
            oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>,
        >,
    ),
    /// The `Serialize` variant converts errors from the `serde` crate.
    #[cfg(feature = "serial")]
    #[cfg_attr(docsrs, doc(cfg(feature = "serial")))]
    #[display("Deserialize error: {:?}", self.source())]
    Serialize(#[from] serde::de::value::Error),
    /// The `SerdeJson` variant converts an error from the `serde_json` crate.
    #[cfg(feature = "serial")]
    #[cfg_attr(docsrs, doc(cfg(feature = "serial")))]
    #[display("Deserialization error: {:?}", self.source())]
    SerdeJson(#[from] serde_json::Error),
    /// The `Sqlx` variant converts a general error from the `sqlx` crate.
    #[cfg(feature = "sql")]
    #[cfg_attr(docsrs, doc(cfg(feature = "sql")))]
    #[display("Sqlx command error: {:?}", self.source())]
    Sqlx(#[from] sqlx::Error),
    /// The `Migrate` variant converts a migration error from the `sqlx` crate.
    #[cfg(feature = "sql")]
    #[cfg_attr(docsrs, doc(cfg(feature = "sql")))]
    #[display("Sqlx migration error: {:?}", self.source())]
    Migrate(#[from] sqlx::migrate::MigrateError),
    /// The `Uuid` variant converts an error from the `uuid` crate.
    #[cfg(feature = "id")]
    #[cfg_attr(docsrs, doc(cfg(feature = "id")))]
    #[display("Uuid conversion failed: {:?}", self.source())]
    Uuid(#[from] uuid::Error),
    /// The `Url` variant converts an error from the `url` crate.
    #[cfg(feature = "urls")]
    #[cfg_attr(docsrs, doc(cfg(feature = "urls")))]
    #[display("Parse error: {:?}", self.source())]
    Url(#[from] url::ParseError),
    /// The `BitMap` variant converts an error from the `plotters_bitmap` crate.
    #[cfg(feature = "plot")]
    #[cfg_attr(docsrs, doc(cfg(feature = "plot")))]
    #[display("Plotting backend error: {:?}", self.source())]
    BitMap(#[from] plotters_bitmap::BitMapBackendError),
    /// The `Plot` variant converts an error from the `plotters` crate.
    #[cfg(feature = "plot")]
    #[cfg_attr(docsrs, doc(cfg(feature = "plot")))]
    #[display("Plotting drawing error: {:?}", self.source())]
    Plot(#[from] plotters::drawing::DrawingAreaErrorKind<plotters_bitmap::BitMapBackendError>),
    /// The `GeoJson` variant converts an error from the `geojson` crate.
    #[cfg(feature = "gis")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gis")))]
    #[display("Error reading geojson file: {:?}", self.source())]
    GeoJson(#[from] geojson::Error),
    /// The `TraceInit` variant converts an error from the `tracing_subscriber` crate.
    #[cfg(feature = "trace")]
    #[cfg_attr(docsrs, doc(cfg(feature = "trace")))]
    #[display("Problem initializing subscriber: {:?}", self.source())]
    TraceInit(#[from] tracing_subscriber::util::TryInitError),
    /// The `Axum` variants converts an *axum::Error* from the `axum` crate.
    #[cfg(feature = "route")]
    #[cfg_attr(docsrs, doc(cfg(feature = "route")))]
    #[display("Axum error: {:?}", self.source())]
    Axum(#[from] axum::Error),
    /// The `AxumHttp` variant converts an axum::http error from the `axum` crate.
    #[cfg(feature = "route")]
    #[cfg_attr(docsrs, doc(cfg(feature = "route")))]
    #[display("Axum http error: {:?}", self.source())]
    AxumHttp(#[from] axum::http::Error),
    /// The `Hyper` variant converts a `hyper_util::error::Error` from the `hyper_util` crate.
    #[cfg(feature = "hype")]
    #[cfg_attr(docsrs, doc(cfg(feature = "hype")))]
    #[display("Legacy client error: {:?}", self.source())]
    Hyper(#[from] hyper::Error),
    /// The `HyperUtil` variant converts a `hyper_util::client::legacy::Error` from the `hyper_util` crate.
    #[cfg(feature = "hype")]
    #[cfg_attr(docsrs, doc(cfg(feature = "hype")))]
    #[display("Legacy client error: {:?}", self.source())]
    HyperUtil(#[from] hyper_util::client::legacy::Error),
    /// The `HyperUtil` variant converts a `hyper_util::client::legacy::Error` from the `hyper_util` crate.
    #[cfg(feature = "gis")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gis")))]
    #[display("Shapefile error: {:?}", self.source())]
    Shapefile(#[from] shapefile::Error),
    /// The `EventLoop` variant converts a [`winit::error::EventLoopError`] from the `winit` crate.
    #[cfg(feature = "win")]
    #[cfg_attr(docsrs, doc(cfg(feature = "win")))]
    #[display("Winit event loop error: {:?}", self.source())]
    EventLoop(#[from] winit::error::EventLoopError),
    /// The `WinOs` variant converts a [`winit::error::OsError`] from the `winit` crate.
    #[cfg(feature = "win")]
    #[cfg_attr(docsrs, doc(cfg(feature = "win")))]
    #[display("OS error: {:?}", self.source())]
    WinOs(#[from] winit::error::OsError),
    /// The `WinitIcon` variant converts a [`winit::window::BadIcon`] from the `winit` crate.
    #[cfg(feature = "win")]
    #[cfg_attr(docsrs, doc(cfg(feature = "win")))]
    #[display("Winit icon error: {:?}", self.source())]
    WinitIcon(#[from] winit::window::BadIcon),
    /// The `WgpuSurface` variant converts a [`wgpu::CreateSurfaceError`] from the `wgpu` crate.
    #[cfg(feature = "gpu")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gpu")))]
    #[display("From wgpu: {:?}", self.source())]
    WgpuSurface(#[from] wgpu::CreateSurfaceError),
    /// The `WgpuDevice` variant converts a [`wgpu::RequestDeviceError`] from the `wgpu` crate.
    #[cfg(feature = "gpu")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gpu")))]
    #[display("From wgpu: {:?}", self.source())]
    WgpuDevice(#[from] wgpu::RequestDeviceError),
    /// The `Nom` variant converts errors from the `nom` crate.
    // #[cfg(feature = "parse")]
    // #[cfg_attr(docsrs, doc(cfg(feature = "parse")))]
    // #[display("Nom error: {:?}", self.source())]
    // Nom(String),
    /// The `Tiberius` variant converts errors from the `tiberius` crate.
    #[cfg(feature = "sql")]
    #[cfg_attr(docsrs, doc(cfg(feature = "sql")))]
    #[display("Tiberius error: {:?}", self.source())]
    Tiberius(#[from] tiberius::error::Error),
    /// The `Dotenv` variant converts errors from the `dotenvy` crate.
    #[cfg(feature = "env")]
    #[cfg_attr(docsrs, doc(cfg(feature = "env")))]
    #[display("Dotenvy error: {:?}", self.source())]
    Dotenv(#[from] dotenvy::Error),
    /// The `Jiff` variant converts errors from the `jiff` crate.
    #[cfg(feature = "time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "time")))]
    #[display("Jiff error: {}", 0)]
    Jiff(#[from] jiff::Error),
}

// #[cfg(feature = "gis")]
// #[cfg_attr(docsrs, doc(cfg(feature = "gis")))]
// impl<T> From<std::sync::PoisonError<T>> for Bandage {
//     fn from(input: std::sync::PoisonError<T>) -> Self {
//         Self::Hint(input.to_string())
//     }
// }
//
// #[cfg(feature = "parse")]
// #[cfg_attr(docsrs, doc(cfg(feature = "parse")))]
// impl<'a> From<nom::Err<nom::error::Error<&'a str>>> for Bandage {
//     fn from(nom: nom::Err<nom::error::Error<&'a str>>) -> Self {
//         let message = format!("{}", nom);
//         Self::Nom(message)
//     }
// }
