#[cfg(all(not(target_os = "macos"), not(target_os = "linux"), not(windows)))]
compile_error!("Unsupported platform!");

mod args;
mod board;
mod cargo_build;
mod ccxml;
mod chip;
mod cli;
mod command_exists;
mod common;
mod configuration;
mod confirm_install;
mod connection;
mod download_file;
mod driver;
mod dslite;
mod flash_target;
mod from_str_error;
mod install_uniflash;
mod md5_verify;
mod progress;
mod raw_args;
mod read_until_matches;
mod warn_experimental_platforms;

use crate::common::*;

fn main() {
    if let Err(err) = cli() {
        eprintln!(
            "{}{} {err}",
            Style::new().red().bold().apply_to("error"),
            Style::new().bold().apply_to(":")
        );
        process::exit(1);
    }
}
