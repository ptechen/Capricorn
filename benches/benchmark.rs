use criterion::{criterion_group, criterion_main, Criterion};

use capricorn::parse;
use quicli::prelude::*;
use serde_yaml;
use std::collections::HashMap;
use capricorn::parse::SelectParams;

pub fn set_heap() {
    let yml = read_file("./test_pages/test.yml").unwrap();
    let v: HashMap<String, SelectParams> = serde_yaml::from_str(&yml).unwrap();
    let html = read_file("./test_pages/test.html").unwrap();
    parse::parse_html(&v, &html);
}

pub fn box_stock() {
    let yml = read_file("./test_pages/test.yml").unwrap();
    let v: HashMap<String, SelectParams> = serde_yaml::from_str(&yml).unwrap();
    let html = read_file("./test_pages/test.html").unwrap();
    Box::new(parse::parse_html(&v, &html));
}

fn criterion_benchmark_heap(c: &mut Criterion) {
    c.bench_function("run ", |b| b.iter(|| set_heap()));
}

fn criterion_benchmark_box(c: &mut Criterion) {
    c.bench_function("box<run> ", |b| b.iter(|| box_stock()));
}

criterion_group!(benches, criterion_benchmark_heap,criterion_benchmark_box);
// criterion_group!(benches, criterion_benchmark_heap);
criterion_main!(benches);