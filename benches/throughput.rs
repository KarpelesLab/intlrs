//! Per-algorithm throughput benchmarks across representative corpora. Run with
//! `cargo bench --features alloc`. These establish the baseline for the planned
//! table-representation (two-level trie) optimization.

use criterion::{Criterion, Throughput, black_box, criterion_group, criterion_main};
use intl::unicode::{collate, general_category, graphemes, nfc, nfd, words};

const ASCII: &str = "The quick brown fox jumps over the lazy dog. ";
const LATIN: &str = "Ça résume bien l'idée — naïveté, cœur, déjà vu. ";
const CJK: &str = "快速的棕色狐狸跳过了懒狗。日本語のテキストも含む。";
const MIXED: &str = "Hello мир 世界 🌍 café — l'été ☕ 3.14 ½ Δx ⊕";

fn corpora() -> [(&'static str, String); 4] {
    [
        ("ascii", ASCII.repeat(20)),
        ("latin", LATIN.repeat(20)),
        ("cjk", CJK.repeat(20)),
        ("mixed", MIXED.repeat(20)),
    ]
}

fn bench(c: &mut Criterion) {
    for (name, text) in corpora() {
        let bytes = text.len() as u64;

        let mut g = c.benchmark_group("general_category");
        g.throughput(Throughput::Bytes(bytes));
        g.bench_function(name, |b| {
            b.iter(|| black_box(&text).chars().map(general_category).count())
        });
        g.finish();

        let mut g = c.benchmark_group("nfc");
        g.throughput(Throughput::Bytes(bytes));
        g.bench_function(name, |b| b.iter(|| nfc(black_box(&text).chars()).count()));
        g.finish();

        let mut g = c.benchmark_group("nfd");
        g.throughput(Throughput::Bytes(bytes));
        g.bench_function(name, |b| b.iter(|| nfd(black_box(&text).chars()).count()));
        g.finish();

        let mut g = c.benchmark_group("graphemes");
        g.throughput(Throughput::Bytes(bytes));
        g.bench_function(name, |b| b.iter(|| graphemes(black_box(&text)).count()));
        g.finish();

        let mut g = c.benchmark_group("words");
        g.throughput(Throughput::Bytes(bytes));
        g.bench_function(name, |b| b.iter(|| words(black_box(&text)).count()));
        g.finish();

        let mut g = c.benchmark_group("sort_key");
        g.throughput(Throughput::Bytes(bytes));
        g.bench_function(name, |b| b.iter(|| collate::sort_key(black_box(&text))));
        g.finish();
    }
}

criterion_group!(benches, bench);
criterion_main!(benches);
