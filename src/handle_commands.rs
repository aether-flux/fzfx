use std::{
    fs::File,
    io::{self, BufRead, BufReader, IsTerminal},
    path::PathBuf,
};

use anyhow::{Context, Result};
use fastembed::{TextEmbedding, TextInitOptions, similarity::cosine_similarity};

use crate::cli::Args;

pub fn handle_args(args: &Args) -> Result<()> {
    // Load candidate dataset
    let candidate_strings = load_commands(args.data_file.clone())?;
    if candidate_strings.is_empty() {
        anyhow::bail!("No candidate lines available to search");
    }

    let commands: Vec<&str> = candidate_strings.iter().map(|s| s.as_str()).collect();

    // Initialize model
    let mut model = TextEmbedding::try_new(
        TextInitOptions::new(fastembed::EmbeddingModel::AllMiniLML6V2)
            .with_show_download_progress(true),
    )?;

    // Get command embeddings
    let cmd_embeddings = embed_commands(&commands, &mut model)?;

    // Get query and embeddings
    let query = match args.query.clone() {
        Some(q) => q,
        None => inquire::Text::new("Enter query: ").prompt()?,
    };
    let query_embeddings = embed_query(query, &mut model)?;

    // Get result scores in ascending order
    let mut scored_res: Vec<(&str, f32)> = commands
        .iter()
        .zip(cmd_embeddings.iter())
        .map(|(cmd, emb)| {
            let score = cosine_similarity(&query_embeddings, emb);
            (*cmd, score)
        })
        .collect();
    scored_res.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // Prepare final list of choices
    let top_k = 3;
    let choices: Vec<String> = scored_res
        .into_iter()
        .take(top_k)
        // .filter(|(_, score)| *score >= 0.20f32)
        .map(|(cmd, score)| format!("[{:.2}] {}", score, cmd))
        .collect();

    if choices.is_empty() {
        anyhow::bail!("No proper command found");
    }

    let selected = inquire::Select::new("Matches:", choices).prompt()?;
    println!("Selected: {}", selected);

    Ok(())
}

fn load_commands(data_file_arg: Option<PathBuf>) -> Result<Vec<String>> {
    let stdin = io::stdin();

    // Check if piped
    if !stdin.is_terminal() {
        let lines: Vec<String> = stdin
            .lock()
            .lines()
            .filter_map(Result::ok)
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect();

        if !lines.is_empty() {
            return Ok(lines);
        }
    }

    // Load file from flag value
    if let Some(path) = data_file_arg {
        let file = File::open(&path).with_context(|| format!("Failed to open file: {:?}", path))?;
        let reader = BufReader::new(file);

        let lines: Vec<String> = reader
            .lines()
            .filter_map(Result::ok)
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect();

        return Ok(lines);
    }

    // Default - load bash history
    load_default_commands()
}

fn load_default_commands() -> Result<Vec<String>> {
    let home_dir = dirs::home_dir().context("Could not determine home directory")?;

    let bash_path = home_dir.join(".bash_history");
    let zsh_path = home_dir.join(".zsh_history");

    let path = if bash_path.exists() {
        bash_path
    } else if zsh_path.exists() {
        zsh_path
    } else {
        anyhow::bail!("No command history file found at ~/.bash_history or ~/.zsh_history");
    };

    let file = File::open(&path).with_context(|| format!("Failed to open file: {:?}", path))?;
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = reader
        .lines()
        .filter_map(Result::ok)
        .map(|l| parse_history_line(&l))
        .filter(|l| !l.is_empty())
        .collect();
    lines.dedup();

    Ok(lines)
}

fn parse_history_line(line: &str) -> String {
    let trimmed = line.trim();
    if trimmed.starts_with(':') {
        if let Some((_, cmd)) = trimmed.split_once(';') {
            return cmd.trim().to_string();
        }
    }
    trimmed.to_string()
}

/// Create command embeddings
fn embed_commands(commands: &Vec<&str>, model: &mut TextEmbedding) -> Result<Vec<Vec<f32>>> {
    let fmt_commands: Vec<String> = commands.iter().map(|c| format!("{}", c)).collect();
    let embeddings = model.embed(fmt_commands, None)?;

    Ok(embeddings)
}

/// Create query embeddings
fn embed_query(query: String, model: &mut TextEmbedding) -> Result<Vec<f32>> {
    let query = format!("{}", query);
    let mut embeddings = model.embed(vec![query], None)?;

    if embeddings.is_empty() {
        anyhow::bail!("Failed to generate query embedding");
    }

    Ok(embeddings.remove(0))
}
