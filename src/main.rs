use rlink::compare_batches;
use std::path::PathBuf;
use std::time::Instant;
use serde::{Serialize, Deserialize};


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
fn main() {
    let candidate_names = csv::ReaderBuilder::new().has_headers(false).from_path("./tests/input/prepped_df_a.csv").unwrap().deserialize().map(|rec| {
        let rec: NameRec = rec.unwrap();
        rec.first_name
    }).filter(|name| name.len() > 0).collect::<Vec<String>>();

    let query_names = csv::ReaderBuilder::new().has_headers(false).from_path("./tests/input/prepped_df_b.csv").unwrap().deserialize().map(|rec| {
        let rec: NameRec = rec.unwrap();
        rec.first_name
    }).filter(|name| name.len() > 0).take(1000).collect::<Vec<String>>();

    /*let mut candidate_names = Vec::new();
    candidate_names.push("isabella e".to_owned());
    let mut query_names = Vec::new();
    query_names.push("nellie".to_owned());*/
    let start = Instant::now();
    compare_batches(PathBuf::from("./tests/output/"), &query_names, &candidate_names, 0.8);
    let elapsed = start.elapsed();
    println!("{} ms", elapsed.as_millis());
}

