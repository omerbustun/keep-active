//! Keep your computer awake.
//!
//! # Examples
//!
//! ```
//! # fn try_main() -> anyhow::Result<()> {
//! let _awake = keepawake::Builder::new()
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
//! //!
//! ```
//! # fn try_main() -> anyhow::Result<()> {
//! let _awake = keepawake::Builder::new()
//!     .display(true)
//!     .idle(true)
//!     .sleep(true)
//!     .create()?;
//! # Ok(())
//! # }
//! # try_main();
//! ```

use anyhow::Result;
use enigo::{Enigo, Key, MouseControllable, KeyboardControllable};
use std::{thread, time::Duration};
mod sys;

/// Builder for a new [AwakeHandle].
#[derive(Debug)]
pub struct Builder {
    display: bool,
    idle: bool,
    sleep: bool,
    reason: Option<String>,
    app_name: Option<String>,
    app_reverse_domain: Option<String>,
}

impl Builder {
    pub fn new() -> Self {
        Builder {
            display: false,
            idle: false,
            sleep: false,
            reason: None,
            app_name: None,
            app_reverse_domain: None,
        }
    }

    /// Prevent the display from turning off.
    pub fn display(mut self, display: bool) -> Self {
        self.display = display;
        self
    }

    /// Prevent the system from sleeping due to idleness.
    pub fn idle(mut self, idle: bool) -> Self {
        self.idle = idle;
        self
    }

    /// Prevent the system from sleeping. Only works under certain, OS dependant, conditions.
    pub fn sleep(mut self, sleep: bool) -> Self {
        self.sleep = sleep;
        self
    }

    /// Reason the consumer is keeping the system awake. Defaults to `"User requested"`. (Used on Linux & macOS)
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    #[allow(dead_code)] // Unused on Windows
    pub(crate) fn reason_or_default(&self) -> &str {
        // TODO Reconsider this defaults. They are really meant for the CLI.
        self.reason.as_deref().unwrap_or("User requested")
    }

    /// Name of the program keeping the system awake. Defaults to `"keepawake-rs"`. (Used on Linux)
    pub fn app_name(mut self, app_name: impl Into<String>) -> Self {
        self.app_name = Some(app_name.into());
        self
    }

    #[allow(dead_code)] // Unused on macOS and Windows
    pub(crate) fn app_name_or_default(&self) -> &str {
        self.app_name.as_deref().unwrap_or("keepawake-rs")
    }

    /// Reverse domain name of the program keeping the system awake. Defaults to `"io.github.segevfiner.keepawake-rs"`. (Used on Linux)
    pub fn app_reverse_domain(mut self, consumer_reverse_domain: impl Into<String>) -> Self {
        self.app_reverse_domain = Some(consumer_reverse_domain.into());
        self
    }

    #[allow(dead_code)] // Unused on macOS and Windows
    pub(crate) fn app_reverse_domain_or_default(&self) -> &str {
        self.app_reverse_domain
            .as_deref()
            .unwrap_or("io.github.segevfiner.keepawake-rs")
    }

    /// Create the [AwakeHandle], consuming self.
    pub fn create(self) -> Result<AwakeHandle> {
        Ok(AwakeHandle {
            _imp: sys::Awake::new(self)?,
        })
    }
}

impl Default for Builder {
    fn default() -> Self {
        Builder::new()
    }
}

/// Keeps the machine or display awake (as configured), until dropped. Create using [Builder].
pub struct AwakeHandle {
    _imp: sys::Awake,
}

// TODO: gradual movement  
//       exit gracefully on Ctrl+C
pub fn simulate_activity() -> Result<(), Box<dyn std::error::Error>> {
    let mut enigo = Enigo::new();

    loop {
        enigo.mouse_move_relative(100, 100);
        thread::sleep(Duration::from_secs(1));
        enigo.mouse_move_relative(-100, -100);

        enigo.key_down(Key::Shift);
        enigo.key_up(Key::Shift);

        thread::sleep(Duration::from_secs(2)); // TODO: make this configurable
    }
}
