use fastembed::TextEmbedding;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufReader, BufWriter},
    path::PathBuf,
};

use anyhow::{Context, Result};
use bincode;

/// Get cache directory path depending on OS
pub fn get_cache_dir() -> Result<PathBuf> {
    let base_cache = dirs::cache_dir().context("Could not resolve user cache directory")?;
    let fzfx_cache = base_cache.join("fzfx");

    if !fzfx_cache.exists() {
        fs::create_dir_all(&fzfx_cache)?;
    }

    Ok(fzfx_cache)
}

#[derive(Serialize, Deserialize, Default)]
pub struct EmbeddingCache {
    pub store: HashMap<String, Vec<f32>>,
}

impl EmbeddingCache {
    fn cache_file_path() -> Result<PathBuf> {
        Ok(get_cache_dir()?.join("embeddings.bin"))
    }

    pub fn load() -> Self {
        let path = match Self::cache_file_path() {
            Ok(p) => p,
            Err(_) => return Self::default(),
        };

        if !path.exists() {
            return Self::default();
        }

        let file = match File::open(&path) {
            Ok(f) => f,
            Err(_) => return Self::default(),
        };

        let reader = BufReader::new(file);
        bincode::deserialize_from(reader).unwrap_or_default()
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::cache_file_path()?;
        let file = File::create(&path)?;
        let writer = BufWriter::new(file);
        bincode::serialize_into(writer, self)?;
        Ok(())
    }

    pub fn get_or_compute_embeddings(
        &mut self,
        commands: &[&str],
        model: &mut TextEmbedding,
    ) -> Result<Vec<Vec<f32>>> {
        let mut missing_commands: Vec<String> = Vec::new();
        let mut missing_idxs: Vec<usize> = Vec::new();

        let mut results: Vec<Option<Vec<f32>>> = vec![None; commands.len()];

        for (idx, &cmd) in commands.iter().enumerate() {
            if let Some(vec) = self.store.get(cmd) {
                results[idx] = Some(vec.clone());
            } else {
                missing_commands.push(cmd.to_string());
                missing_idxs.push(idx);
            }
        }

        if !missing_commands.is_empty() {
            let computed_embeddings = model.embed(missing_commands.clone(), None)?;

            for (cmd, emb, orig_idx) in izip(missing_commands, computed_embeddings, missing_idxs) {
                self.store.insert(cmd, emb.clone());
                results[orig_idx] = Some(emb);
            }

            let _ = self.save();
        }

        let final_embs = results.into_iter().filter_map(|x| x).collect();
        Ok(final_embs)
    }
}

use std::iter::zip;
fn izip(
    a: Vec<String>,
    b: Vec<Vec<f32>>,
    c: Vec<usize>,
) -> impl Iterator<Item = (String, Vec<f32>, usize)> {
    zip(a, zip(b, c)).map(|(cmd, (emb, idx))| (cmd, emb, idx))
}
