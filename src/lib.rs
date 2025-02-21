//! Keep your computer awake.
//!
//! # Examples
//!
//! ```
//! # fn try_main() -> anyhow::Result<()> {
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
//! # fn try_main() -> anyhow::Result<()> {
//! let _awake = keep_active::Builder::default()
//!     .display(true)
//!     .idle(true)
//!     .sleep(true)
//!     .create()?;
//! # Ok(())
//! # }
//! # try_main();
//! ```

use anyhow::Result;
use derive_builder::Builder;
use enigo::{Enigo, MouseControllable};
use std::{thread, time::Duration};

mod sys;

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

    /// Prevent the system from sleeping. Only works under certain, OS dependant, conditions.
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

pub fn simulate_activity() -> Result<(), Box<dyn std::error::Error>> {
    let mut enigo = Enigo::new();
    
    loop {
        // Move mouse by just 1 pixel right
        enigo.mouse_move_relative(1, 0);
        thread::sleep(Duration::from_millis(50));
        
        // Move back to original position
        enigo.mouse_move_relative(-1, 0);
        
        // Wait before next activity simulation
        thread::sleep(Duration::from_secs(60)); // TODO: Make this configurable
    }
}
