// This file is part of the IPUMS's pseudo_jaro_winkler.
// For copyright and licensing information, see the NOTICE and LICENSE files
// in this project's top-level directory, and also on-line at:
//   https://github.com/ipums/pseudo_jaro_winkler

use std::{
    fs::read_dir,
    fs::File,
    path::PathBuf
};
use polars::prelude::*;
use polars::datatypes::AnyValue;
use rayon::prelude::*;
use itertools::Itertools;
use indicatif::ParallelProgressIterator;
use indicatif::ProgressIterator;
use pseudo_jaro_winkler::pseudo_jaro_winkler;

#[derive(Debug, Clone)]
struct BlockInfo {
    block_key: BlockKey,
    start: i64,
    len: usize
}

#[derive(Debug, Clone, Eq, Ord, PartialOrd, PartialEq)]
struct BlockKey {
    sex: i64,
    bpl: i32,
    replaced_birthyr: i64
}

#[derive(Debug, Clone)]
struct BlockedFrame {
    df: DataFrame,
    block_infos: Vec<BlockInfo>
}

#[derive(Debug, Clone)]
struct Block {
    block_key: BlockKey,
    values: Vec<(BlockInfo, Arc<DataFrame>)>
}

fn main() {
    let paths = read_dir("./input/blocks_a.parquet").unwrap().collect::<Vec<_>>();
    let paths = paths.into_iter().map(|p| p.unwrap().path()).filter(|p| p.to_str().unwrap().ends_with(".parquet")).collect::<Vec<PathBuf>>();
    //let path = paths.first().unwrap();
    let requested_fields = vec!["histid", "sex", "replaced_birthyr", "bpl_orig", "namefrst_std", "namelast_clean"];
    let block_fields = vec!["sex", "bpl_orig", "replaced_birthyr"];
    

    let blocked_frames = paths.par_iter().progress_count(paths.len() as u64).map(|p| {
        let file = File::open(p.as_path()).unwrap();
        let mut df = ParquetReader::new(file).with_columns(Some(requested_fields.iter().map(|s| s.to_string()).collect::<Vec<_>>())).finish().unwrap();
        df.sort_in_place(vec!["sex", "bpl_orig", "replaced_birthyr"], vec![true, true, true]).unwrap();
        let groups = df.groupby(&block_fields).unwrap().take_groups().into_idx();
        let block_infos = groups.into_iter().map(|(first, all)|{ 
            let select = df.select(&block_fields).unwrap();
            let labels = select.get(first as usize).unwrap();
            let sex = match labels[0] { AnyValue::Int64(n) => n, _ => panic!("Couldn't get value") };
            let bpl = match labels[1] { AnyValue::Int32(n) => n, _ => panic!("Couldn't get value") };
            let replaced_birthyr = match labels[2] { AnyValue::Int64(n) => n, _ => panic!("Couldn't get value") };
            let start = first as i64;
            let len = all.len();
            let block_key = BlockKey { sex, bpl, replaced_birthyr };
            let bi = BlockInfo { block_key, start, len };
            bi
        }).collect::<Vec<_>>();
        let bf = BlockedFrame { df, block_infos };
        bf
    }).collect::<Vec<_>>();
    // Will need to read in a different frame here
    let mut dup_blocks = blocked_frames.into_iter().flat_map(|bf| {
        let df_arc = Arc::new(bf.df);
        bf.block_infos.iter().map(|bi| {
            let block_key = bi.block_key.clone();
            let values = vec![(bi.clone(), df_arc.clone())];
            Block { block_key, values }
        }).collect::<Vec<Block>>()
    }).collect::<Vec<Block>>();
    dup_blocks.sort_by_key(|block| block.block_key.clone());
    let blocks = dup_blocks.into_iter().group_by(|block| block.block_key.clone()).into_iter().map(|(block_key, group)| {
        let mut block = Block { block_key, values: Vec::new() };
        group.into_iter().for_each(|dup_block| block.values.extend(dup_block.values));
        block
    }).collect::<Vec<Block>>();
    let blocks = Arc::new(blocks);
    let blocks_a = blocks.clone();
    let _blocks_b = blocks.clone();
    blocks_a.iter().progress().for_each(|block_a| {
        let mut names_a : Vec<String> = Vec::new();
        block_a.values.iter().for_each(|(block_info, df)| {
            let series = df.select_series(&["namefrst_std"]).unwrap();
            let names = series.first().unwrap().utf8().unwrap();
            let slice = &names.slice(block_info.start, block_info.len);
            slice.into_iter().for_each(|name| names_a.push(name.unwrap_or("").to_string()));
        });
        let names_b : Vec<String> = names_a.clone();
        let output_dir = PathBuf::from(format!("./output/{}_{}_{}/", block_a.block_key.sex, block_a.block_key.bpl, block_a.block_key.replaced_birthyr));
        pseudo_jaro_winkler(&names_a, &names_b, output_dir, 0.8);
    });





    //pseudo_jaro_winkler(&names_a, &names_b, PathBuf::from(output_dir), 0.8);



    //}).reduce(|df1, df2| df1.vstack(&df2).unwrap()).unwrap
    //
    //
    //
    //
    //
    //
    //
    //();
    //println!("Rechunking a...");
    //rows.sort_in_place(vec!["sex", "bpl_orig", "replaced_birthyr"], vec![true, true, true]);
    //println!("Sorting df a...");
    //println!("{:?}", rows);
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

