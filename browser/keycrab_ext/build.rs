use std::{io, process::Command};

fn main() -> io::Result<()> {
    Command::new("npm")
        .current_dir("./scripts")
        .args(&["run", "build"])
        .spawn()?;

    Ok(())
}
