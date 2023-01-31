/*
This file should only contain code to read in the command line arguments in and dispatch to the desired functionality.
*/

use anyhow::Result;
use pico_args;
mod generate_bindings;
mod reexport_versionless;

fn main() -> Result<()> {
    let mut parser = pico_args::Arguments::from_env();
    let task = parser
        .subcommand()
        .expect("Task name was no a valid UTF-8 string")
        .expect("xtask task name cannot start with '-'");
    match task.as_str() {
        "generate-bindings" => generate_bindings::generate_bindings(parser)?,
        "reexport-versionless" => reexport_versionless::reexport_versionless(parser)?,
        _ => anyhow::bail!("Task '{task}' not found"),
    }
    Ok(())
}
