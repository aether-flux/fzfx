use std::{
    fs,
    hint::black_box,
    path::PathBuf,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use criterion::{Criterion, criterion_group, criterion_main};
use fastembed::{TextEmbedding, TextInitOptions};
use fzfx::{EmbeddingCache, HybridMatch, get_cache_dir};

fn create_isolated_temp_dir(prefix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("cmd_bench_{}_{}", prefix, nanos));
    fs::create_dir_all(&dir).expect("Failed to create temporary bench directory");
    dir
}

fn bench_embedding(c: &mut Criterion) {
    let sample_commands: Vec<&str> = vec![
        "git status",
        "git commit -m \"fix: resolve lifetime issues\"",
        "cargo build --release",
        "docker compose up -d --build",
        "kubectl get pods -n kube-system",
        "systemctl status systemd-resolved.service",
        "find . -type f -name \"*.rs\" -exec grep -H \"fn\" {} +",
        "tar -xvf archive.tar.gz -C /opt/",
        "awk -F':' '{print $1}' /etc/passwd",
        "ssh -i ~/.ssh/id_ed25519 user@192.168.1.100",
    ];
    let query = "find files and search text";

    let mut group = c.benchmark_group("pipeline_cache_matrix");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(110));

    // Benchmark 1: Fully Cold Pipeline (Fresh model cache dir + Fresh embedding cache)
    group.bench_function("01_fully_cold", |b| {
        b.iter(|| {
            let tmp_dir = create_isolated_temp_dir("fully_cold");

            // Initialize model pointed to clean directory
            let mut model = TextEmbedding::try_new(
                TextInitOptions::new(fastembed::EmbeddingModel::AllMiniLML6V2)
                    .with_cache_dir(tmp_dir.clone())
                    .with_show_download_progress(false),
            )
            .expect("Failed to init model");

            let mut cache = EmbeddingCache::new_ephemeral();
            let embeddings = cache
                .get_or_compute_embeddings(black_box(&sample_commands), &mut model)
                .expect("Failed to compute embeddings");

            // Cleanup isolated folder
            let _ = fs::remove_dir_all(tmp_dir);
            black_box(embeddings)
        });
    });

    // Benchmark 2: Warm Model, Cold Embedding Cache (Model ready, fresh embed cache)
    let cache_dir = get_cache_dir().expect("Failed to get system cache directory");
    let mut warm_model = TextEmbedding::try_new(
        TextInitOptions::new(fastembed::EmbeddingModel::AllMiniLML6V2)
            .with_cache_dir(cache_dir)
            .with_show_download_progress(false),
    )
    .expect("Failed to initialize warm model");

    group.bench_function("02_warm_model_cold_embed_cache", |b| {
        b.iter(|| {
            // Re-instantiate empty cache on each iteration
            let mut cold_cache = EmbeddingCache::new_ephemeral();
            let embeddings = cold_cache
                .get_or_compute_embeddings(black_box(&sample_commands), &mut warm_model)
                .expect("Failed to compute embeddings");

            black_box(embeddings)
        });
    });

    // Benchmark 3: Warm Model + Warm Embedding Cache (Cache Hit Path)
    group.sample_size(100); // Higher sample size since this path is fast
    let mut warm_cache = EmbeddingCache::load();
    // Warm up the cache once before running benchmark iterations
    let _ = warm_cache.get_or_compute_embeddings(&sample_commands, &mut warm_model);

    group.bench_function("03_warm_model_warm_embed_cache", |b| {
        b.iter(|| {
            let embeddings = warm_cache
                .get_or_compute_embeddings(black_box(&sample_commands), &mut warm_model)
                .expect("Failed cache lookup");

            black_box(embeddings)
        });
    });

    // Benchmark 4: Pure Rerank & Hybrid Scoring Math
    let mock_vector_res: Vec<(String, f32)> = sample_commands
        .iter()
        .enumerate()
        .map(|(idx, &cmd)| (cmd.to_string(), 0.5 + (idx as f32 * 0.04)))
        .collect();

    group.bench_function("04_hybrid_rerank_only", |b| {
        b.iter(|| {
            let ranked = HybridMatch::rerank(
                black_box(query),
                black_box(mock_vector_res.clone()),
                black_box(0.7),
            );
            black_box(ranked)
        });
    });

    group.finish();
}

criterion_group!(benches, bench_embedding);
criterion_main!(benches);
