// This file is part of the IPUMS's pseudo_jaro_winkler.
// For copyright and licensing information, see the NOTICE and LICENSE files
// in this project's top-level directory, and also on-line at:
//   https://github.com/ipums/pseudo_jaro_winkler

use indicatif::{ProgressIterator, ParallelProgressIterator};
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use pseudo_jaro_winkler::*;
use std::{
    fs::File,
    fs::read_dir,
    path::PathBuf,
    path::Path,
    io::{BufRead, BufReader},
    time::Instant
};
use parquet::file::reader::{FileReader, SerializedFileReader};
use std::sync::Arc;
use clap::{Arg, App};
use std::convert::TryFrom;
use parquet::schema::types::Type;
use parquet::record::RowAccessor;
use rayon::prelude::*;
use parquet::record::Row;
use parquet::arrow::ParquetFileArrowReader;
use parquet::arrow::ArrowReader;
use polars::prelude::*;

pub fn make_vec() -> Vec<Row> {
    let layout = std::alloc::Layout::array::<Row>(220_000).unwrap();
    // I copied the following unsafe code from Stack Overflow without understanding
    // it. I was advised not to do this, but I didn't listen. It's my fault.
    unsafe {
        Vec::from_raw_parts(
            std::alloc::alloc_zeroed(layout) as *mut _,
            220_000,
            220_000,
        )
    }
}

fn main() {
    let paths = read_dir("./input/blocks_a.parquet").unwrap().collect::<Vec<_>>();
    let paths = paths.into_iter().map(|p| p.unwrap().path()).filter(|p| p.to_str().unwrap().ends_with(".parquet")).collect::<Vec<PathBuf>>();
    let path = paths.first().unwrap();
    let requested_fields = vec!["histid", "sex", "replaced_birthyr", "bpl_orig", "namefrst_std", "namelast_clean"];

    let mut rows = paths.iter().map(|p| {
        let file = File::open(p.as_path()).unwrap();
        ParquetReader::new(file).with_columns(Some(requested_fields.iter().map(|s| s.to_string()).collect::<Vec<_>>())).finish().unwrap()
    }).reduce(|df1, df2| df1.vstack(&df2).unwrap()).unwrap();
    rows.sort_in_place(vec!["sex", "bpl_orig", "replaced_birthyr"], vec![true, true, true]);
    std::thread::sleep(std::time::Duration::from_secs(10));
    println!("{:?}", rows);
    /*
    let sfr = SerializedFileReader::try_from(path.as_path()).unwrap();
    let schema = sfr.metadata().file_metadata().schema();
    let fields = schema.get_fields().to_vec();
    let mut column_indices = requested_fields.iter().map(|rf| {
        fields.iter().position(|f| rf == &f.name()).unwrap()
    }).collect::<Vec<_>>();



    let mut sfrs = paths.iter().map(|p| {
        SerializedFileReader::try_from(path.as_path()).unwrap()
    }).collect::<Vec<_>>();

    let mut file_batches = sfrs.into_par_iter().progress_count(paths.len() as u64).map(|sfr| {
        let mut arrow_reader = ParquetFileArrowReader::new(Arc::new(sfr));
        let batch_reader = arrow_reader.get_record_reader_by_columns(column_indices.clone(), 2048).unwrap();
        batch_reader.map(|br| br.unwrap()).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let total_rows: usize = file_batches.iter().map(|batches| batches.iter().map(|b| b.num_rows()).sum::<usize>()).sum();
    println!("{}", total_rows);
    println!("{}", file_batches.len());
    */


    /*

    let requested_fields = vec!["histid", "sex", "replaced_birthyr", "bpl_orig", "namefrst_std", "namelast_clean"];
    let mut selected_fields = requested_fields.iter().map(|rf| {
        fields.iter().find(|f| rf == &f.name()).unwrap().clone()
    }).collect::<Vec<_>>();

    let schema_projection = Type::group_type_builder(schema.name())
        .with_fields(&mut selected_fields)
        .build()
        .unwrap();
    let mut sfrs = paths.iter().map(|p| {
        SerializedFileReader::try_from(path.as_path()).unwrap()
    }).collect::<Vec<_>>();
    println!("Got memory.");

    let mut file_rows = sfrs.into_par_iter().progress_count(paths.len() as u64).map(|(mut v, sfr)| {
        sfr.get_row_iter(Some(schema_projection.clone())).unwrap().for_each(|row| {
            v.push(row)
        });
    }).collect::<Vec<_>>();
    println!("First row: {:?}", file_rows.first());
    file_rows.par_sort_unstable_by_key(|r| {
        (r.get_long(1).unwrap(), r.get_long(2).unwrap(), r.get_int(3).unwrap())
    });
    for r in file_rows.iter().take(2) {
        println!("Take 2: {:?}", r);
    }*/
}

/*fn main() {
    let cli_matches = App::new("pseudo_jaro_winkler")
        .version("0.1")
        .author("Jacob Wellington <jakew@umn.edu>")
        .about("Creates very fast jaro winkler scores between two datasets.")
        .arg(Arg::with_name("file_a")
            .help("First file to link. Must be a file where each row is a name.")
            .required(true)
            .index(1))
        .arg(Arg::with_name("file_b")
            .help("Second file to link. Must be a file where each row is a name.")
            .required(true)
            .index(2))
        .arg(Arg::with_name("output_dir")
            .help("Directory to put the output matches.")
            .required(true)
            .index(3))
        .get_matches();
    let file_a : &str = cli_matches.value_of("file_a").unwrap();
    let file_b : &str = cli_matches.value_of("file_b").unwrap();
    let names_a = BufReader::new(File::open(file_a).expect(&format!("Error opening file_a: {}", file_a))).lines().map(|n| n.unwrap()).collect::<Vec<String>>();
    let names_b = BufReader::new(File::open(file_b).expect(&format!("Error opening file_b: {}", file_b))).lines().map(|n| n.unwrap()).collect::<Vec<String>>();
    
    names_a.iter().enumerate().for_each(|(i, name)| {
        assert_ne!(name.len(), 0, "Error: file_a has blank line at line #: {}", i + 1);
    });
    names_b.iter().enumerate().for_each(|(i, name)| {
        assert_ne!(name.len(), 0, "Error: file_b has blank line at line #: {}", i + 1);
    });

    let output_dir = cli_matches.value_of("output_dir").unwrap();
    let start = Instant::now();
    pseudo_jaro_winkler(&names_a, &names_b, PathBuf::from(output_dir), 0.8);
    let elapsed = start.elapsed();
    println!("{} ms", elapsed.as_millis());
}*/

