#![deny(rust_2018_idioms)]

use migration_helpers::common_migrations::AddPrefixesMigration;
use migration_helpers::{migrate, Result};
use std::process;

/// We added two new settings for configuring kubelet, `kubernetes.kube-reserved`
/// and `settings.kubernetes.eviction-hard`.  We don't want to track all possible
/// keys for these settings, so we remove the whole prefix when we downgrade.
fn run() -> Result<()> {
    migrate(AddPrefixesMigration(vec![
        "settings.kubernetes.kube-reserved",
        "settings.kubernetes.eviction-hard",
    ]))
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
