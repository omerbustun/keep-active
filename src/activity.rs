//! Simulate user activity to keep status trackers (e.g. Skype, MS Teams) "active".
//!
//! This nudges the mouse cursor by one pixel and immediately moves it back, on a
//! configurable interval, which is enough to register as activity without the cursor
//! visibly drifting.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

use enigo::{Coordinate, Direction, Enigo, Key, Keyboard, Mouse, Settings};
use thiserror::Error;

/// Errors that can occur while simulating activity.
#[derive(Error, Debug)]
pub enum ActivityError {
    /// The input-simulation backend could not be initialized.
    #[error("failed to initialize input simulation: {0}")]
    Connection(#[from] enigo::NewConError),

    /// A simulated input event failed to dispatch.
    #[error("failed to simulate input: {0}")]
    Input(#[from] enigo::InputError),

    /// The worker thread terminated before it could report initialization.
    #[error("activity thread terminated unexpectedly")]
    Terminated,
}

/// How long to wait between consecutive `stop` checks while sleeping, so the
/// simulator shuts down promptly instead of after a full interval.
const STOP_POLL_INTERVAL: Duration = Duration::from_millis(100);

/// How the simulator registers activity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ActivityMethod {
    /// Nudge the mouse one pixel and back. Visible, but reliably registers
    /// activity across status trackers.
    #[default]
    Mouse,

    /// Tap the `F15` key, which virtually nothing binds. Resets the idle/away
    /// timer with no visible effect, but some trackers may ignore it.
    Key,
}

/// Simulates user activity on a background thread until dropped.
///
/// Dropping the simulator signals the background thread to stop and waits for it
/// to finish, so activity simulation is tied to the lifetime of this value.
pub struct ActivitySimulator {
    stop: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl ActivitySimulator {
    /// Start simulating activity using `method`, once every `interval`.
    ///
    /// The input backend is initialized on the worker thread; any initialization
    /// failure is reported synchronously, so a returned [`ActivitySimulator`] is
    /// guaranteed to have a live backend.
    pub fn start(method: ActivityMethod, interval: Duration) -> Result<Self, ActivityError> {
        let stop = Arc::new(AtomicBool::new(false));
        let stop_worker = Arc::clone(&stop);
        let (init_tx, init_rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            // `Enigo` is not `Send` on all platforms, so it must be created on the
            // thread that uses it. Report the result back so `start` can surface it.
            let mut enigo = match Enigo::new(&Settings::default()) {
                Ok(enigo) => {
                    if init_tx.send(Ok(())).is_err() {
                        return;
                    }
                    enigo
                }
                Err(e) => {
                    let _ = init_tx.send(Err(ActivityError::from(e)));
                    return;
                }
            };

            run(&mut enigo, method, interval, &stop_worker);
        });

        match init_rx.recv() {
            Ok(Ok(())) => Ok(Self {
                stop,
                handle: Some(handle),
            }),
            Ok(Err(e)) => {
                let _ = handle.join();
                Err(e)
            }
            // The thread vanished before reporting; join to recover the panic, if any.
            Err(_) => {
                let _ = handle.join();
                Err(ActivityError::Terminated)
            }
        }
    }
}

impl Drop for ActivitySimulator {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

/// Simulate activity on `interval`, returning when `stop` is set.
fn run(enigo: &mut Enigo, method: ActivityMethod, interval: Duration, stop: &AtomicBool) {
    while !stop.load(Ordering::Relaxed) {
        let result = match method {
            ActivityMethod::Mouse => nudge_mouse(enigo),
            ActivityMethod::Key => tap_key(enigo),
        };
        if let Err(e) = result {
            eprintln!("keep-active: activity simulation stopped: {e}");
            return;
        }
        sleep_interruptible(interval, stop);
    }
}

/// Move the cursor one pixel and back, leaving its position unchanged.
fn nudge_mouse(enigo: &mut Enigo) -> Result<(), ActivityError> {
    enigo.move_mouse(1, 0, Coordinate::Rel)?;
    thread::sleep(Duration::from_millis(50));
    enigo.move_mouse(-1, 0, Coordinate::Rel)?;
    Ok(())
}

/// Tap `F15`, a key virtually nothing binds, without moving the cursor.
fn tap_key(enigo: &mut Enigo) -> Result<(), ActivityError> {
    enigo.key(Key::F15, Direction::Click)?;
    Ok(())
}

/// Sleep for `total`, waking early if `stop` is set.
fn sleep_interruptible(total: Duration, stop: &AtomicBool) {
    let mut remaining = total;
    while !remaining.is_zero() {
        if stop.load(Ordering::Relaxed) {
            return;
        }
        let nap = remaining.min(STOP_POLL_INTERVAL);
        thread::sleep(nap);
        remaining -= nap;
    }
}
