use std::{
    io,
    process::{self, Command},
    sync::mpsc::channel,
    thread,
    time::Duration,
};

use anyhow::Result;
use clap::{CommandFactory, Parser, ValueHint};
use clap_complete::{generate, Shell};
use shadow_rs::shadow;
use sysinfo::{Pid, ProcessRefreshKind, System};

use keep_active::simulate_activity;
use keep_active::Builder;

shadow!(build);

#[derive(Parser)]
#[command(author, version, long_version = build::CLAP_LONG_VERSION, about, long_about = None,
    arg_required_else_help = true)]
struct Cli {
    /// Keep display on
    #[arg(short = 'd', long)]
    display: bool,

    /// Keep system from idle sleeping
    #[arg(short = 'i', long)]
    idle: bool,

    /// Keep system from sleeping (Functionality and conditions for this to work vary by OS)
    #[arg(short = 's', long)]
    sleep: bool,

    /// Keep status trackers active
    #[arg(short = 'a', long)] // Changed short option for status_active
    status_active: bool,

    /// Generate shell completions
    #[arg(long, exclusive = true, value_enum, value_name = "SHELL")]
    completions: Option<Shell>,

    /// Wait for the process with the specified PID to exit.
    /// This option is ignored when used with the COMMAND argument.
    #[arg(short = 'w', value_name = "PID")] // Changed short option for wait
    wait: Option<u32>,

    /// Run the command and wait for it to exit, keeping the computer awake while it runs.
    #[arg(trailing_var_arg = true, value_hint = ValueHint::CommandWithArguments)]
    command: Vec<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Start the activity simulation if the status_active flag is set
    if cli.status_active {
        thread::spawn(|| {
            if let Err(e) = simulate_activity() {
                eprintln!("Failed to simulate activity: {}", e);
            }
        });
    }

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

            loop {
                if !system.refresh_process_specifics(pid, ProcessRefreshKind::new()) {
                    break 0;
                }
                thread::sleep(Duration::from_millis(200));
            }
        } else {
            rx.recv().expect("Could not receive from channel.");
            130
        }
    };

    process::exit(exit_code);
}
