#![deny(rust_2018_idioms)]

use migration_helpers::{error, migrate, Migration, MigrationData, Result};
use snafu::ResultExt;
use std::fs;
use std::io;
use std::path::Path;
use std::process;

const SETTING: &str = "settings.kubernetes.cpu-manager-policy";
const CPU_MANAGER_POLICY_CHECKPOINT: &str = "/var/lib/kubelet/cpu_manager_state";
/// We remove cpu manager policy checkpoint value if checkpoint value is different from upgrade version
/// or downgrade version cpu-manager-policy setting value.
pub struct CpuManagerPolicyCleaner;

impl Migration for CpuManagerPolicyCleaner {
    fn forward(&mut self, input: MigrationData) -> Result<MigrationData> {
        println!("CpuManagerPolicyCleaner forward has no work to do on upgrade.");
        Ok(input)
    }

    fn backward(&mut self, mut input: MigrationData) -> Result<MigrationData> {
        // removing existing cpu_manager_policy_state file
        if Path::new(CPU_MANAGER_POLICY_CHECKPOINT).exists() {
            println!(
                "Deleting existing cpu manager policy checkpoint: '{}'",
                CPU_MANAGER_POLICY_CHECKPOINT
            );
            match fs::remove_file(CPU_MANAGER_POLICY_CHECKPOINT) {
                Ok(()) => {}
                Err(e) => {
                    if e.kind() != io::ErrorKind::NotFound {
                        return Err(e).context(error::RemoveFile {
                            path: CPU_MANAGER_POLICY_CHECKPOINT,
                        });
                    } else {
                        println! ("NotFound: '{}'", CPU_MANAGER_POLICY_CHECKPOINT)
                    }
                }
            }
        }

        Ok(input)
    }
}
/// We added a new settings for configuring kubelet, `settings.kubernetes.cpu-manager-policy`
fn run() -> Result<()> {
    migrate(CpuManagerPolicyCleaner)
}

// Returning a Result from main makes it print a Debug representation of the error, but with Snafu
// we have nice Display representations of the error, so we wrap "main" (run) and print any error.
// https://github.com/shepmaster/snafu/issues/110
fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        process::exit(1);
    }
}
