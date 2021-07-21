#![allow(unused_imports)]
use format_bytes::format_bytes;
use itertools::Itertools;
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
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
struct NameRec {
    histid: String,
    first_name: String,
    last_name: String,
    bigrams: String
}

fn main() {
    let a_block_dir = Path::new("./output/blocks_a");
    let b_block_dir = Path::new("./output/blocks_b");
    write_blocks(Path::new("./data/large/a_blocking/"), a_block_dir);
    write_blocks(Path::new("./data/large/b_blocking/"), b_block_dir);
    let output_file = "output.csv";
    File::create(output_file).unwrap();
    let a_block_files = fs::read_dir(a_block_dir).unwrap().flat_map(|a| a).collect::<Vec<_>>();
    let b_block_files = fs::read_dir(b_block_dir).unwrap().flat_map(|b| b).collect::<Vec<_>>();
    a_block_files.par_iter().progress_count(a_block_files.len() as u64).fold(
        || (csv::WriterBuilder::new().has_headers(false).from_writer(OpenOptions::new().append(true).open(output_file).unwrap()), JaroWinkler::new()),
        |(mut wtr, jw), a_block_file| {
            let matching_b_blocks = b_block_files.iter().filter(|b_block_file| { 
                let a_name = a_block_file.file_name().into_string().unwrap();
                let b_name = b_block_file.file_name().into_string().unwrap();
                let (sex_a, bpl_a, birthyr_a, race_a) = a_name.split(',').next_tuple().unwrap();
                let (sex_b, bpl_b, birthyr_b, race_b) = b_name.split(',').next_tuple().unwrap();
                sex_a == sex_b && bpl_a == bpl_b && (birthyr_a.parse::<i32>().unwrap() - birthyr_b.parse::<i32>().unwrap()).abs() <= 3 && race_a == race_b 
            });
            for b_block_file in matching_b_blocks  {
                let mut a_rdr = csv::Reader::from_path(a_block_file.path()).unwrap();
                let mut b_rdr = csv::Reader::from_path(b_block_file.path()).unwrap();
                println!("{:?}, {:?}", a_block_file, b_block_file);
                for a_result in a_rdr.deserialize()  {
                    let a_rec: NameRec = a_result.unwrap();
                    for b_result in b_rdr.deserialize() {
                        let b_rec: NameRec = b_result.unwrap();
                        let f_jw = jw.similarity(&a_rec.first_name, &b_rec.first_name);
                        let l_jw = jw.similarity(&a_rec.last_name, &b_rec.last_name);
                        if f_jw > 0.7 && l_jw > 0.7 {
                            wtr.serialize((&a_rec, &b_rec, f_jw, l_jw)).unwrap();
                        }
                    }
                }
            }
            (wtr, jw)
        }).collect::<Vec<_>>();
    exit(1);
}

/*fn output_file(output_dir: Path) -> BufWriter<File> {
    let output_path = output_dir.join(format!("{}.csv", current_thread_index()));
    BufWriter::new(
    
}*/

fn write_blocks(input_dir:  &Path, output_dir: &Path) {
    if output_dir.exists() {
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
            let mut wtr = csv::Writer::from_path(output_dir.join(String::from_utf8(block.to_vec()).unwrap())).unwrap();
            for rec in recs.into_inner().unwrap() {
                wtr.serialize(rec).unwrap();
            }
            wtr.flush().unwrap();
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
    while rdr.read_byte_record(&mut record).unwrap() {
        let bigram_iter = str::from_utf8(&record[6]).unwrap().split("|").map(|bigram| bigram.to_string()).collect::<Vec<String>>();
        let name_rec = NameRec { 
            histid: str::from_utf8(&record[0]).unwrap().to_string(),
            first_name: str::from_utf8(&record[1]).unwrap().to_string(),
            last_name: str::from_utf8(&record[2]).unwrap().to_string(),
            bigrams: str::from_utf8(&record[6]).unwrap().to_string()
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
    }
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
