#![allow(warnings, unused)]
use std::cmp::{max, min};
use eddie::JaroWinkler;
use csv::Writer;
use serde::{Serialize, Deserialize};
use indicatif::{ProgressBar, ParallelProgressIterator, ProgressIterator};
use std::time::Instant;
use rayon::prelude::*;
use itertools::Itertools;
use strsim::{jaro_winkler,jaro};
use std::str::Chars;

fn maskify(query: &String) -> Vec<(u8, [u16; 16])> {
    let len = query.len();
    let min_match_dist = if len > 3 { len / 2 - 1 } else { 0 };
    query.replace(" ", "`").chars().enumerate().map( |(i, c)| {
        let index = c as u8 - '`' as u8;
        let base_mask = 1 << i;
        let mut masks_by_candidate_len: [u16; 16] = [0; 16];
        for candidate_len in 1..17 {
            let match_distance = if candidate_len <= 3 || candidate_len / 2 - 1 <= min_match_dist {
                min_match_dist
            } else {
                candidate_len / 2 - 1
            };
            let mut query_mask = base_mask.clone();
            for i in 0..match_distance {
                query_mask = query_mask << 1 | query_mask;
                query_mask = query_mask >> 1 | query_mask;
            }
            masks_by_candidate_len[candidate_len - 1] = query_mask;
        }
       (index, masks_by_candidate_len)
    }).collect()
}

#[derive(Debug, Clone)]
struct CandidateLetterInfo {
    name_index: usize,
    len: usize,
    mask: u16
}

#[derive(Debug, Clone)]
struct LetterInfo {
    len: usize,
    mask: u16
}

#[derive(Debug, Clone)]
struct CandidateScore {
    matches: u16,
    increment: u16,
    used: u16,
    used_exact: u16,
    partial_jw: u16,
    last_match_letter_index: u16,
    transposition_count: u8
}

fn build_candidate_lookup(names: &Vec<String>) -> Vec<Vec<CandidateLetterInfo>> {
    let mut letter_lookup: Vec<Vec<CandidateLetterInfo>> = Vec::new();
    for (letter_index, letter) in ('`'..'{').enumerate() {
        let mut candidate_infos : Vec<CandidateLetterInfo> = Vec::new();
        for (name_index, name) in names.iter().enumerate() {
            let mut mask : u16 = 0;
            name.replace(" ", "`").chars().positions(|c| c == letter).for_each(|matching_index_in_name| {
                mask += u16::pow(2, matching_index_in_name as u32);
            });
            if mask != 0 {
                let info = CandidateLetterInfo { name_index, len: name.len(), mask };
                candidate_infos.push(info);
            }
        }
        letter_lookup.push(candidate_infos);
    }
    letter_lookup
}

fn build_candidate_list(names: &Vec<String>) -> Vec<(u8, Vec<u16>)> {
    names.iter().map(|name| {
        let masks = ('`'..'{').enumerate().map(|(letter_index, letter)| {
            let mut mask : u16 = 0;
            name.replace(" ", "`").chars().positions(|c| c == letter).for_each(|matching_index_in_name| {
                mask += u16::pow(2, matching_index_in_name as u32);
            });
            mask
        }).collect();
        (name.len() as u8, masks)
    }).collect()
}

fn main() {
    let candidate_names = csv::ReaderBuilder::new().has_headers(false).from_path("./input/prepped_df_a.csv").unwrap().deserialize().map(|rec| {
        let rec: NameRec = rec.unwrap();
        rec.first_name
    }).filter(|name| name.len() > 0).collect::<Vec<String>>();

    let query_names = csv::ReaderBuilder::new().has_headers(false).from_path("./input/prepped_df_b.csv").unwrap().deserialize().map(|rec| {
        let rec: NameRec = rec.unwrap();
        rec.first_name
    }).filter(|name| name.len() > 0).take(100).collect::<Vec<String>>();

    //let mut candidate_names = Vec::new();
    //candidate_names.push("jane a k".to_owned());
    //let query = "jake".to_owned();
    compare_batches(query_names, candidate_names);
}

fn compare_batches(query_names: Vec<String>, candidate_names: Vec<String>) {

    let start = Instant::now();
    let base_candidate_lookup = build_candidate_lookup(&candidate_names);
    let base_candidate_scores = candidate_names.iter().map(|name| {
        CandidateScore { matches: 0, used_exact: 0, partial_jw: 0, used: 0, last_match_letter_index: 0, transposition_count: 0, increment: ((1.0 / name.len() as f32) * 1000.0) as u16 }
    }).collect::<Vec<CandidateScore>>();
    query_names.iter().progress_count(query_names.len() as u64).for_each(|query_name| {
        let candidate_lookup = base_candidate_lookup.clone();
        let mut candidate_scores = base_candidate_scores.clone();

        let query_len = query_name.len();
        let query_increment = ((1.0 / query_len as f32) * 1000.0) as u16;
        let query_masks_lookup = maskify(&query_name);
        //println!("{:?}", candidate_lookup[0].len());

        for (query_index, (letter_index, query_mask_by_candidate_len)) in query_masks_lookup.iter().enumerate() {
             candidate_lookup[*letter_index as usize].iter().for_each(|c_info| {
                let candidate_score = &mut candidate_scores[c_info.name_index];
                let query_mask = query_mask_by_candidate_len[c_info.len - 1];
                let whole_mask_result = (query_mask & c_info.mask); // Get raw matches
                let check_used_result = (whole_mask_result | candidate_score.used) ^ candidate_score.used; // Make sure we haven't used that match before
                let last_match_letter_index = (1 << check_used_result.trailing_zeros()) & check_used_result; // Find the first match found
                let mask_result = check_used_result & last_match_letter_index; // Take the first match found
                let is_match_mask = !(((mask_result >> mask_result.trailing_zeros()) & 1) - 1); // All 1s if there is a result, else all 0s
                candidate_score.used |= mask_result;
                candidate_score.used_exact |= mask_result & (1 << query_index);
                candidate_score.matches += is_match_mask & 1;
                candidate_score.partial_jw += is_match_mask & candidate_score.increment;
                candidate_score.partial_jw += is_match_mask & query_increment;
                candidate_score.transposition_count +=  (mask_result - 1 < candidate_score.last_match_letter_index) as u8;
                candidate_score.last_match_letter_index |= mask_result;
             });
        }
        candidate_scores.into_iter().filter(|score| {
            let jaro_partial = ((score.partial_jw as f32 / 1000.0)  + 1.0 - (score.transposition_count as f32 / score.matches as f32)) / 3.0;
            let l = (score.used_exact & 0b1111u16).trailing_ones() as f32;
            let jw = jaro_partial + 0.1 * l * (1.0 - jaro_partial);
            jw > 0.8
        }).collect::<Vec<_>>();
    });
    let elapsed = start.elapsed();
    println!("{} ms", elapsed.as_millis());
}

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


