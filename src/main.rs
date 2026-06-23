use std::{
    io,
    process::{self, Command},
    sync::mpsc::channel,
    time::Duration,
};

use anyhow::Result;
use clap::{CommandFactory, Parser, ValueEnum, ValueHint};
use clap_complete::{generate, Shell};
use shadow_rs::shadow;
use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System};

use keep_active::ActivityMethod;
use keep_active::ActivitySimulator;
use keep_active::Builder;

shadow!(build);

/// How activity is simulated when `--status-active` is set.
#[derive(Clone, Copy, ValueEnum)]
enum Method {
    /// Nudge the mouse one pixel and back (visible, most compatible).
    Mouse,
    /// Tap the F15 key (no cursor movement; some trackers may ignore it).
    Key,
}

impl From<Method> for ActivityMethod {
    fn from(method: Method) -> Self {
        match method {
            Method::Mouse => ActivityMethod::Mouse,
            Method::Key => ActivityMethod::Key,
        }
    }
}

#[derive(Parser)]
#[command(author, version, long_version = build::CLAP_LONG_VERSION, about, long_about = None,
    arg_required_else_help = true)]
struct Cli {
    /// Keep display on
    #[arg(short, long)]
    display: bool,

    /// Keep system from idle sleeping
    #[arg(short, long)]
    idle: bool,

    /// Keep system from explicitly sleeping (Functionality and conditions for this to work vary by OS)
    #[arg(short, long)]
    sleep: bool,

    /// Keep status trackers active (e.g. Skype, MS Teams etc.) by simulating activity
    #[arg(short = 'a', long)]
    status_active: bool,

    /// How to simulate activity (used with --status-active)
    #[arg(long, value_enum, default_value_t = Method::Mouse)]
    method: Method,

    /// Interval between simulated activity events, in seconds (used with --status-active)
    #[arg(long, value_name = "SECONDS", default_value_t = 60)]
    interval: u64,

    /// Generate shell completions
    #[arg(long, exclusive = true, value_enum, value_name = "SHELL")]
    completions: Option<Shell>,

    /// Wait for the process with the specified PID to exit.
    /// This option is ignored when used with the COMMAND argument.
    #[arg(short = 'w', value_name = "PID")]
    wait: Option<u32>,

    /// Run the command and wait for it to exit, keeping the computer awake while it runs.
    #[arg(trailing_var_arg = true, value_hint = ValueHint::CommandWithArguments)]
    command: Vec<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(shell) = cli.completions {
        generate(shell, &mut Cli::command(), "keep-active", &mut io::stdout());
        return Ok(());
    }

    let (tx, rx) = channel();

    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");

    let exit_code = {
        let _awake = Builder::default()
            .display(cli.display)
            .idle(cli.idle)
            .sleep(cli.sleep)
            .create()?;

        // Simulate activity for as long as we keep the system awake. The simulator
        // stops its background thread when dropped at the end of this scope.
        let _activity = if cli.status_active {
            Some(ActivitySimulator::start(
                cli.method.into(),
                Duration::from_secs(cli.interval),
            )?)
        } else {
            None
        };

        if !cli.command.is_empty() {
            // TODO Improve exit code in signal exit cases
            Command::new(&cli.command[0])
                .args(&cli.command[1..])
                .spawn()?
                .wait()?
                .code()
                .unwrap_or(128)
        } else if let Some(pid) = cli.wait {
            let pid = Pid::from_u32(pid);
            let mut system = System::new();
            system.refresh_processes_specifics(
                ProcessesToUpdate::Some(&[pid]),
                true,
                ProcessRefreshKind::nothing(),
            );

            if let Some(process) = system.process(pid) {
                process.wait();
            }

            0
        } else {
            rx.recv().expect("Could not receive from channel.");
            130
        }
    };

    process::exit(exit_code);
}
