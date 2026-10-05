use std::{
    fs::File,
    io::{self, BufRead, BufReader, IsTerminal},
    path::PathBuf,
};

use anyhow::{Context, Result};
use fastembed::{TextEmbedding, TextInitOptions, similarity::cosine_similarity};

use crate::{
    cache::{EmbeddingCache, get_cache_dir},
    cli::Args,
    hybrid::HybridMatch,
    util::{extract_command, handle_copy_and_exec},
};

/// Handle CLI arguments
pub fn handle_args(args: &Args) -> Result<()> {
    // Clear cache if flags are passed
    if args.clear_cache {
        EmbeddingCache::clear_cache_files(args.clear_all)?;
        return Ok(());
    }

    if args.clear_all {
        EmbeddingCache::clear_cache_files(args.clear_all)?;
        return Ok(());
    }

    // Load candidate dataset
    let candidate_strings = load_commands(args.data_file.clone())?;
    if candidate_strings.is_empty() {
        anyhow::bail!("No candidate lines available to search");
    }

    let commands: Vec<&str> = candidate_strings.iter().map(|s| s.as_str()).collect();

    // Initialize model
    let show_progress = !args.raw && io::stderr().is_terminal();
    let cache_dir = get_cache_dir()?;
    let mut model = TextEmbedding::try_new(
        TextInitOptions::new(fastembed::EmbeddingModel::AllMiniLML6V2)
            .with_cache_dir(cache_dir)
            .with_show_download_progress(show_progress),
    )?;

    let mut cache = EmbeddingCache::load();

    // Get command embeddings
    let cmd_embeddings = cache.get_or_compute_embeddings(&commands, &mut model)?;

    // Get query and embeddings
    let query = match args.query.clone() {
        Some(q) => q,
        None => {
            if args.raw {
                anyhow::bail!("Query (-q) is required when running in raw mode");
            }
            inquire::Text::new("Enter query: ").prompt()?
        }
    };
    let query_embeddings = embed_query(&query, &mut model)?;

    // Get result scores in ascending order
    let vector_res: Vec<(String, f32)> = commands
        .iter()
        .zip(cmd_embeddings.iter())
        .map(|(cmd, emb)| {
            let score = cosine_similarity(&query_embeddings, emb);
            let cmd = cmd.to_string();
            (cmd, score)
        })
        .collect();
    // scored_res.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let scored_res = HybridMatch::rerank(&query, vector_res, 0.7);

    // Prepare final list of choices
    let selected_raw = if args.raw {
        let top_match = scored_res.into_iter().next().context("No matches found")?;
        top_match.command
    } else {
        let top_k = args.top_k;
        let choices: Vec<String> = scored_res
            .into_iter()
            .take(top_k)
            .map(|m| format!("[{:.2}] {}", m.final_score, m.command))
            .collect();

        if choices.is_empty() {
            anyhow::bail!("No proper command found");
        }

        let selected = inquire::Select::new("Matches:", choices).prompt()?;
        // println!("Selected: {}", selected);
        extract_command(&selected).to_string()
    };

    if args.raw || (!args.copy && !args.exec && !args.raw) {
        println!("{}", selected_raw);
    }

    if args.copy || args.exec {
        handle_copy_and_exec(&selected_raw, args.copy, args.exec)?;
    }

    Ok(())
}

/// Load command list from file or piped input
fn load_commands(data_file_arg: Option<PathBuf>) -> Result<Vec<String>> {
    let stdin = io::stdin();

    // Check if piped
    if !stdin.is_terminal() {
        let lines: Vec<String> = stdin
            .lock()
            .lines()
            .map_while(Result::ok)
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
            .map_while(Result::ok)
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect();

        return Ok(lines);
    }

    // Default - load bash history
    load_default_commands()
}

/// Load command list from bash/zsh history (default)
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
        .map_while(Result::ok)
        .map(|l| parse_history_line(&l))
        .filter(|l| !l.is_empty())
        .collect();
    lines.dedup();

    Ok(lines)
}

fn parse_history_line(line: &str) -> String {
    let trimmed = line.trim();
    if trimmed.starts_with(':')
        && let Some((_, cmd)) = trimmed.split_once(';')
    {
        return cmd.trim().to_string();
    }
    trimmed.to_string()
}

/// Create query embeddings
fn embed_query(query: &str, model: &mut TextEmbedding) -> Result<Vec<f32>> {
    let query = query.to_string();
    let mut embeddings = model.embed(vec![query], None)?;

    if embeddings.is_empty() {
        anyhow::bail!("Failed to generate query embedding");
    }

    Ok(embeddings.remove(0))
}
