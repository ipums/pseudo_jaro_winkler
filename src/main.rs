use std::cmp;
use serde::{Serialize, Deserialize};
use std::time::Instant;
use itertools::Itertools;
use rayon::prelude::*;

fn maskify(query: String) -> Vec<(u8, [u16; 16])> {
    let len = query.len();
    let min_match_dist = if len > 3 { len / 2 - 1 } else { 0 };
    query.chars().enumerate().map( |(i, c)| {
        let index = c as u8 - 'a' as u8;
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
struct CandidateLetterInfo {
    name_index: usize,
    len: usize,
    mask: u16
}

#[derive(Debug)]
struct CandidateScore {
    matches: u8,
    len: u8,
    used: u16,
    last_match_letter_index: u16,
    transposition_count: u8
}

fn build_candidate_lookup(names: Vec<String>) -> Vec<Vec<CandidateLetterInfo>> {
    let mut letter_lookup: Vec<Vec<CandidateLetterInfo>> = Vec::new();
    for (letter_index, letter) in ('a'..'{').enumerate() {
        let mut candidate_infos : Vec<CandidateLetterInfo> = Vec::new();
        for (name_index, name) in names.iter().enumerate() {
            let mut mask : u16 = 0;
            name.chars().positions(|c| c == letter).for_each(|matching_index_in_name| {
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

fn main() {
    /*let candidate_names = csv::ReaderBuilder::new().has_headers(false).from_path("./input/prepped_df_a.csv").unwrap().deserialize().map(|rec| {
        let rec: NameRec = rec.unwrap();
        rec.first_name
    }).collect::<Vec<String>>();*/

    let mut candidate_names = Vec::new();
    candidate_names.push("joaek".to_owned());
    let mut candidate_scores = candidate_names.iter().map(|name| {
        CandidateScore { matches: 0, used: 0, last_match_letter_index: 0, transposition_count: 0, len: name.len() as u8 }
    }).collect::<Vec<CandidateScore>>();
    let candidate_lookup = build_candidate_lookup(candidate_names);

    let query = "jkeo".to_owned();
    let query_len = query.len();
    println!("maskify");
    let query_masks_lookup = maskify(query);

    println!("{:?}", candidate_lookup[0].len());

    let start = Instant::now();
    for (query_index, (letter_index, query_mask_by_candidate_len)) in query_masks_lookup.iter().enumerate() {
         candidate_lookup[*letter_index as usize].iter().for_each(|c_info| {
            let candidate_score = &mut candidate_scores[c_info.name_index];
            let whole_mask_result = (query_mask_by_candidate_len[c_info.len - 1] & c_info.mask); // Get raw matches
            let check_used_result = (whole_mask_result | candidate_score.used) ^ candidate_score.used; // Make sure we haven't used that match before
            let last_match_letter_index = (1 << check_used_result.trailing_zeros()) & check_used_result; // Find the first match found
            let mask_result = check_used_result & last_match_letter_index; // Take the first match found
            candidate_score.used |= mask_result;

            candidate_score.matches += (mask_result > 0) as u8;
            dbg!(mask_result, last_match_letter_index);
            let last_match_letter_index =; // make these no ops on last_match_letter_index is 0
            candidate_score.transposition_count +=  (last_match_letter_index < candidate_score.last_match_letter_index) as u8;
            candidate_score.last_match_letter_index = last_match_letter_index;
         });
    }
    println!("{:?}", candidate_scores[0]);

    candidate_scores.into_iter().filter(|score| { score.matches > 0 }).take(1).for_each(|score| println!("num matches: {:?}", score.matches));
    let elapsed = start.elapsed();
    println!("{} micro", elapsed.as_micros());
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


