// This file is part of rlink.
// For copyright and licensing information, see the NOTICE and LICENSE files
// in this project's top-level directory, and also on-line at:
//   https://github.com/ipums/rlink

use rlink::psuedo_jaro_winkler;
use std::{
    fs::File,
    path::PathBuf,
    io::{BufRead, BufReader},
    time::Instant
};
use clap::{Arg, App};

fn main() {
    let cli_matches = App::new("Rlink")
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
    psuedo_jaro_winkler(&names_a, &names_b, PathBuf::from(output_dir), 0.8);
    let elapsed = start.elapsed();
    println!("{} ms", elapsed.as_millis());
}

