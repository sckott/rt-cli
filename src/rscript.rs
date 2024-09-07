use std::process::{Command, ExitStatus};
use std::io::Result;

// Run Rscript from Rust
pub fn run_rscript(r_call: &str) -> Result<ExitStatus> {
    // Start the subprocess
    let mut child = Command::new("Rscript").args(["-e", r_call]).spawn()?;

    // Wait for the subprocess to exit
    let exit_status = child.wait()?;

    // Check the exit status if necessary
    if exit_status.success() {
        println!("rt exited :)");
    } else {
        println!("The rt subprocess exited with an error: {:?}", exit_status);
    }

    Ok(exit_status)
}
