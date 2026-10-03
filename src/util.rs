use std::process::Command;

use anyhow::{Context, Result};
use arboard::Clipboard;

pub fn extract_command(selected: &str) -> &str {
    if let Some((_, cmd)) = selected.split_once("] ") {
        cmd.trim()
    } else {
        selected.trim()
    }
}

pub fn handle_copy_and_exec(cmd: &str, copy: bool, exec: bool) -> Result<()> {
    if copy {
        let mut clipboard = Clipboard::new().context("Failed to initialize system clipboard")?;
        clipboard
            .set_text(cmd)
            .context("Failed to set clipboard contents")?;
        eprintln!("Copied to clipboard successfully!");
    }

    if exec {
        eprintln!("Executing '{}'...\n", cmd);

        let status = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .status()
            .context("Failed to execute command")?;

        if !status.success() {
            if let Some(code) = status.code() {
                eprintln!("Command exited with status code: {}", code);
            }
        }
    }

    Ok(())
}
