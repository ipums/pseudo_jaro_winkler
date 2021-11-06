#![allow(unused_imports)]
pub use eddie::utils;
use eddie::slice;
use eddie::utils::Buffer;
use format_bytes::format_bytes;
use indicatif::{ProgressBar, ParallelProgressIterator, ProgressIterator};
use std::io::Write;
use std::process::exit;
use bytelines::*;
use std::collections::HashMap;
use std::fs::{remove_dir_all, create_dir_all, File, OpenOptions};
use std::io::BufReader;
use std::io::BufRead;
use eddie::JaroWinkler;
use std::time::{Duration, Instant};
use std::str;
use rayon::prelude::*;
use std::fs::{self, DirEntry};
use std::collections::HashSet;
use std::iter::FromIterator;
use std::path::Path;
use std::sync::{RwLock, Mutex};
use std::error::Error;
use std::fmt;
use strsim::jaro_winkler;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
//"histid","sex","birthyr","bpl","namefrst","namelast","namefrst_clean","namelast_clean"
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

#[derive(Serialize, Deserialize, Debug)]
struct NameRecB {
    histid: String,
    sex: String,
    birthyr: String,
    bpl: String,
    namefrst_raw: String,
    namelast_raw: String,
    first_name: Vec<u8>,
    last_name: Vec<u8>,
}

fn main() {
    //let a_blocks = write_blocks(Path::new("./data/large/a_blocking/"), Path::new("./output/blocks_a"));
    //let b_blocks = write_blocks(Path::new("./data/large/b_blocking/"), Path::new("./output/blocks_b"));
    /*File::create(output_file).unwrap();
    let paths = fs::read_dir(input_dir).unwrap().flat_map(|entry| {
        let entry = entry.unwrap();
        let path = entry.path();
        match path.extension() {
            Some(ext) if ext == "csv" => {Some(path)},
            _ => {None}
        }
    }).collect::<Vec<_>>();*/
    /*a_blocks.par_iter().progress_count(a_blocks.len() as u64).fold(
        || (OpenOptions::new().append(true).open(output_file).unwrap(), JaroWinkler::new()),
        |(mut file, jw), (a_block, a_records)| {
            if let Some(b_records) = b_blocks.get(a_block) {
                for (a_rec, b_rec) in a_records.iter().zip(b_records) {
                    let f_jw = jw.similarity(&a_rec.first_name, &b_rec.first_name);
                    let l_jw = jw.similarity(&a_rec.last_name, &b_rec.last_name);
                    if f_jw > 0.7 && l_jw > 0.7 {
                        writeln!(file, "{},{},{},{}", a_rec, b_rec, f_jw, l_jw).unwrap();
                    }
                }
            }
            (file, jw)
        }).collect::<Vec<_>>();*/
    let df_a = csv::ReaderBuilder::new().has_headers(false).from_path("./input/prepped_df_a.csv").unwrap().deserialize().map(|rec| {
        let rec: NameRec = rec.unwrap();
        NameRecB {
            histid: rec.histid,
            sex: rec.sex,
            birthyr: rec.birthyr,
            bpl: rec.bpl,
            namefrst_raw: rec.namefrst_raw,
            namelast_raw: rec.namelast_raw,
            first_name: rec.first_name.into_bytes(),
            last_name: rec.last_name.into_bytes(),
        }
    }).collect::<Vec<NameRecB>>();
    let df_b = csv::ReaderBuilder::new().has_headers(false).from_path("./input/prepped_df_b.csv").unwrap().deserialize().map(|rec| {
        let rec: NameRec = rec.unwrap();
        NameRecB {
            histid: rec.histid,
            sex: rec.sex,
            birthyr: rec.birthyr,
            bpl: rec.bpl,
            namefrst_raw: rec.namefrst_raw,
            namelast_raw: rec.namelast_raw,
            first_name: rec.first_name.into_bytes(),
            last_name: rec.last_name.into_bytes(),
        }
    }).collect::<Vec<NameRecB>>();
    let pmatch = df_a.par_iter().progress_count(df_a.len() as u64).flat_map(|a_rec| {
        let jw = JaroWinkler::new();
        df_b.iter().flat_map(|b_rec| {
            let f_jw = jw.similarity(&a_rec.first_name, &b_rec.first_name);
            let l_jw = jw.similarity(&a_rec.last_name, &b_rec.last_name);
            if f_jw >= 0.8 && l_jw >= 0.8 {
                Some((f_jw, l_jw, a_rec.clone(), b_rec.clone()))
            } else {
                None
            }
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    println!("Done comparing.");
    let mut f = File::create("output.csv").unwrap();
    for (f_jw, l_jw, a_rec, b_rec) in pmatch {
        f.write(format!("{}, {}, {:?}, {:?}", f_jw, l_jw, a_rec, b_rec).as_bytes()).unwrap();
    }
    
    
    exit(1);
}

/*fn output_file(output_dir: Path) -> BufWriter<File> {
    let output_path = output_dir.join(format!("{}.csv", current_thread_index()));
    BufWriter::new(
    
}*/
/*
fn write_blocks(input_dir: &Path, output_dir: &Path) {
    if output_dir.exists() && false {
        println!("Skipping creation of blocks: {:?}", output_dir);
    } else {
        remove_dir_all(output_dir);
        create_dir_all(output_dir);
        println!("Processing input dir");
        let paths = fs::read_dir(input_dir).unwrap().flat_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            match path.extension() {
                Some(ext) if ext == "csv" => {Some(path)},
                _ => {None}
            }
        }).collect::<Vec<_>>();
        let now = Instant::now();
        let max_size = paths.len();
        println!("Creating blocks");
        let mut blocks_lock = RwLock::new(HashMap::new());
        paths.par_iter().take(max_size).progress_count(max_size as u64).for_each(|path| {
            read_blocks_from_file(path, &blocks_lock);
        });
        let blocks = blocks_lock.into_inner().unwrap();
        let num_blocks = blocks.len();
        blocks.into_par_iter().progress_count(num_blocks as u64).for_each(|(block, recs)| {
            let mut output_file = File::create(output_dir.join(String::from_utf8(block.to_vec()).unwrap())).unwrap();
            for rec in recs.into_inner().unwrap() {
                writeln!(output_file, "{}", rec);
            }
        });




        /*let split_blocks = paths.par_iter().take(max_size).progress_count(max_size as u64).map(|path| { 
            let blocks = read_blocks_from_file(path);
            println!("{} -- {:?}", now.elapsed().as_secs(), path);
            blocks
        }).collect::<Vec<_>>();
        let mut final_blocks = HashMap::new();
        println!("Combining blocks");
        split_blocks.iter().progress_count(max_size as u64).for_each(|split_block| {
            for (block, recs) in split_block {
                let final_recs = final_blocks.entry(block.clone()).or_insert_with(|| Vec::new());
                final_recs.extend_from_slice(&recs);
            }
        });
        println!("{} -- final blocks len: {}", now.elapsed().as_secs(), final_blocks.len());
        println!("Writing blocks.");*/
        /*final_blocks.par_iter().progress_count(max_size as u64).for_each(|(block, recs)| {
            let mut output_file = File::create(output_dir.join(String::from_utf8(block.to_vec()).unwrap())).unwrap();
            for rec in recs {
                writeln!(output_file, "{}", rec);
            }
        });*/
    }
}

fn read_blocks_from_file(path: &Path, blocks_lock: &RwLock<HashMap<Vec<u8>, Mutex<Vec<NameRec>>>>) {
    let mut rdr = csv::Reader::from_path(path).unwrap();
    let mut record = csv::ByteRecord::new();
//"histid","sex","birthyr","bpl","namefrst","namelast","namefrst_clean","namelast_clean"
    while rdr.read_byte_record(&mut record).unwrap() {
        let bigram_iter = str::from_utf8(&record[6]).unwrap().split("|").map(|bigram| bigram.to_string()).collect::<Vec<String>>();
        let name_rec = NameRec { 
            histid: str::from_utf8(&record[0]).unwrap().to_string(),
            first_name: str::from_utf8(&record[1]).unwrap().to_string(),
            last_name: str::from_utf8(&record[2]).unwrap().to_string(),
            bigrams: HashSet::from_iter(bigram_iter)
        };
        let block_key = format_bytes!(b"{},{},{},{}", record[3], record[4], record[5], record[7]);
        let mut recs_lock: Option<Mutex<Vec<NameRec>>> = None;
        {
            let recs_lock = blocks_lock.read().unwrap().get(&block_key);
        }
        match recs_lock {
            Some(lock) => { lock.lock().unwrap().push(name_rec) },
            None => {
                let mut blocks = blocks_lock.write().unwrap();
                match blocks.get(&block_key) {
                    Some(recs) => { recs.lock().unwrap().push(name_rec); },
                    None => { 
                        let mut recs = Vec::new();
                        recs.push(name_rec);
                        blocks.insert(block_key, Mutex::new(recs));
                    }
                }
            }
        }
    }gg
}

        /*let mut comma_count = 0;
        let mut commas = [0, 0, 0];
        let mut line_iter = line.into_iter();
        let mut pos = 0;
        while let Some(&c) = line_iter.next() { 
            if c == b',' { 
                commas[comma_count] = pos;
                comma_count += 1;
                if comma_count == 3 {
                    break;
                }
            }
            pos += 1;
        };*/
*/
