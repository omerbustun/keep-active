//! Keep your computer awake (and active).
//!
//! # Examples
//!
//! ```
//! # fn try_main() -> keep_active::Result<()> {
//! let _awake = keep_active::Builder::default()
//!     .display(true)
//!     .reason("Video playback")
//!     .app_name("My prog")
//!     .app_reverse_domain("io.github.myprog")
//!     .create()?;
//! # Ok(())
//! # }
//! # try_main();
//! ```
//!
//! ```
//! # fn try_main() -> keep_active::Result<()> {
//! let _awake = keep_active::Builder::default()
//!     .display(true)
//!     .idle(true)
//!     .sleep(true)
//!     .create()?;
//! # Ok(())
//! # }
//! # try_main();
//! ```

use derive_builder::Builder;
use thiserror::Error;

mod sys;

#[cfg(feature = "activity")]
pub mod activity;

#[cfg(feature = "activity")]
pub use activity::{ActivityError, ActivityMethod, ActivitySimulator};

/// A system error whose actual type varies by target.
pub use sys::Error as SystemError;

/// Error type.
#[derive(Error, Debug)]
pub enum Error {
    #[error("builder: {0}")]
    Builder(#[from] BuilderError),

    #[error("system: {0}")]
    System(#[from] SystemError),

    #[cfg(feature = "activity")]
    #[error("activity: {0}")]
    Activity(#[from] ActivityError),
}

/// A specialized [`Result`](std::result::Result) type for this crate.
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Builder, Debug)]
#[builder(public, name = "Builder", build_fn(private))]
#[allow(dead_code)] // Some fields are unused on some platforms
struct Options {
    /// Prevent the display from turning off.
    #[builder(default)]
    display: bool,

    /// Prevent the system from sleeping due to idleness.
    #[builder(default)]
    idle: bool,

    /// Prevent the system from explicitly sleeping. Only works under certain, OS dependant, conditions.
    #[builder(default)]
    sleep: bool,

    // TODO Reconsider this defaults. They are really meant for the CLI.
    /// Reason the consumer is keeping the system awake. Defaults to `"User requested"`. (Used on Linux & macOS)
    #[builder(setter(into), default = "\"User requested\".to_string()")]
    reason: String,

    /// Name of the program keeping the system awake. Defaults to `"keep-active"`. (Used on Linux)
    #[builder(setter(into), default = "\"keep-active\".to_string()")]
    app_name: String,

    /// Reverse domain name of the program keeping the system awake. Defaults to `"io.github.omerbustun.keep-active"`. (Used on Linux)
    #[builder(
        setter(into),
        default = "\"io.github.omerbustun.keep-active\".to_string()"
    )]
    app_reverse_domain: String,
}

impl Builder {
    /// Create the [`KeepActive`].
    pub fn create(&self) -> Result<KeepActive> {
        Ok(KeepActive {
            _imp: sys::KeepActive::new(self.build()?)?,
        })
    }
}

/// Keeps the machine or display awake (as configured), until dropped. Create using [struct@Builder].
pub struct KeepActive {
    _imp: sys::KeepActive,
}
