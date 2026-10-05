use std::process::Command;

use anyhow::{Context, Result};
use arboard::Clipboard;

/// Extract command string from given selected choice
pub fn extract_command(selected: &str) -> &str {
    if let Some((_, cmd)) = selected.split_once("] ") {
        cmd.trim()
    } else {
        selected.trim()
    }
}

/// Checks copy and exec flags and functions accordingly
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

        if !status.success()
            && let Some(code) = status.code()
        {
            eprintln!("Command exited with status code: {}", code);
        }
    }

    Ok(())
}
