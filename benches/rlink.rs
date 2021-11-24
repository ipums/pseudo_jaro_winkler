// This file is part of the IPUMS's psuedo_jaro_winkler.
// For copyright and licensing information, see the NOTICE and LICENSE files
// in this project's top-level directory, and also on-line at:
//   https://github.com/mnpopcenter/psuedo_jaro_winkler

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::path::PathBuf;
use std::time::Duration;
use serde::{Serialize, Deserialize};
use psuedo_jaro_winkler::psuedo_jaro_winkler;

#[derive(Serialize, Deserialize, Debug)]
struct NameRec {
    histid: String,
    sex: String,
    birthyr: String,
    bpl: String,
    namefrst_raw: String,
    namelast_raw: String,
    first_name: String,
    last_name: String,
}

fn bench_compare(c: &mut Criterion) {
    let query_names = csv::ReaderBuilder::new().has_headers(false).from_path("./tests/input/prepped_df_b.csv").unwrap().deserialize().map(|rec| {
        let rec: NameRec = rec.unwrap();
        rec.first_name
    }).filter(|name| name.len() > 0).take(10).collect::<Vec<String>>();

    let candidate_names = csv::ReaderBuilder::new().has_headers(false).from_path("./tests/input/prepped_df_a.csv").unwrap().deserialize().map(|rec| {
        let rec: NameRec = rec.unwrap();
        rec.first_name
    }).filter(|name| name.len() > 0).take(100000).collect::<Vec<String>>();
    c.bench_function("psuedo_jaro_winkler", |b| b.iter(|| {
        psuedo_jaro_winkler(black_box(&query_names), black_box(&candidate_names),PathBuf::from("./tests/output/"), 0.8);
    }));
}

criterion_group!{
    name = benches;
    config = Criterion::default().warm_up_time(Duration::new(1,0)).measurement_time(Duration::new(1, 0)).sample_size(10);
    targets = bench_compare
}
criterion_main!(benches);
