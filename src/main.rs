#![allow(warnings, unused)]
use std::cmp;
use eddie::Jaro;
use csv::Writer;
use serde::{Serialize, Deserialize};
use std::time::Instant;
use itertools::Itertools;

fn maskify(query: &String) -> Vec<(u8, [u16; 16])> {
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

struct LetterInfo {
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

fn build_candidate_lookup(names: &Vec<String>) -> Vec<Vec<CandidateLetterInfo>> {
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

fn build_candidate_list(names: &Vec<String>) -> Vec<(u8, Vec<u16>)> {
    names.iter().map(|name| {
        let masks = ('a'..'{').enumerate().map(|(letter_index, letter)| {
            let mut mask : u16 = 0;
            name.chars().positions(|c| c == letter).for_each(|matching_index_in_name| {
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
    let candidate_list = build_candidate_list(&candidate_names);
    let query = "ajke".to_owned();
    let query_len = query.len();
    println!("maskify");
    let query_masks_lookup = maskify(&query);
    println!("{:?}", candidate_list[0]);

    let start = Instant::now();
    let results = candidate_list.iter().flat_map(|(candidate_len, candidate_masks)| {
        /*
        let mut candidate_score = CandidateScore { matches: 0, used: 0, last_match_letter_index: 0, transposition_count: 0, len: *candidate_len };
        for (letter_index, masks_by_len)  in query_masks_lookup.iter() {
            let whole_mask_result = (masks_by_len[(*candidate_len - 1) as usize] & candidate_masks[*letter_index as usize]); // Get raw matches
            let check_used_result = (whole_mask_result | candidate_score.used) ^ candidate_score.used; // Make sure we haven't used that match before
            let last_match_letter_index = (1 << check_used_result.trailing_zeros()) & check_used_result; // Find the first match found
            let mask_result = check_used_result & last_match_letter_index; // Take the first match found
            candidate_score.used |= mask_result;
            candidate_score.matches += (mask_result > 0) as u8;
            candidate_score.transposition_count +=  (mask_result.wrapping_sub(1) < candidate_score.last_match_letter_index) as u8;
            candidate_score.last_match_letter_index |= mask_result;
        }*/
        Some(CandidateScore { matches: 0, used: 0, last_match_letter_index: 0, transposition_count: 0, len: *candidate_len })
    }).collect::<Vec<_>>();
    let elapsed = start.elapsed();
    println!("{} micro", elapsed.as_micros());

    //let mut candidate_names = Vec::new();
}

fn main2() {
    let candidate_names = csv::ReaderBuilder::new().has_headers(false).from_path("./input/prepped_df_a.csv").unwrap().deserialize().map(|rec| {
        let rec: NameRec = rec.unwrap();
        rec.first_name
    }).collect::<Vec<String>>();

    //let mut candidate_names = Vec::new();
    //candidate_names.push("matthew".to_owned());
    let mut candidate_scores = candidate_names.iter().map(|name| {
        CandidateScore { matches: 0, used: 0, last_match_letter_index: 0, transposition_count: 0, len: name.len() as u8 }
    }).collect::<Vec<CandidateScore>>();
    let candidate_lookup = build_candidate_lookup(&candidate_names);

    let query = "ajke".to_owned();
    let query_len = query.len();
    println!("maskify");
    let query_masks_lookup = maskify(&query);

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
            candidate_score.transposition_count +=  (mask_result.wrapping_sub(1) < candidate_score.last_match_letter_index) as u8;
            candidate_score.last_match_letter_index |= mask_result;
         });
    }
    println!("{:?}", candidate_scores[0]);

    let jaro = Jaro::new();
    let mut wtr = Writer::from_path("output.csv").unwrap();
    let query_len_u8 = query_len as u8;
    //candidate_scores.into_iter().zip(candidate_names).filter(|(score, name)| { score.matches > 0 }).map(|(score, name)| { 
    /*let a = candidate_scores.into_iter().filter(|(score)| { 
        score.matches > 0 
            && score.matches as f32 >= 0.8 as f32 * ((3 * score.len * query_len_u8 - (score.len * query_len_u8)) as f32) / (score.len + query_len_u8) as f32
    }).map(|score| {
        //let jw_eddie = jaro.similarity(&query, &name);
        let jw = (1.0 / 3.0) * ( score.matches as f32 / score.len as f32 + score.matches as f32 / query_len as f32 + (score.matches - score.transposition_count) as f32/ score.matches as f32);
        //jw
        //if (jw - jw_eddie as f32).abs() > 0.01 {
        //    wtr.write_record(&[name, jw_eddie.to_string(), jw.to_string()]).unwrap();
        //}
        if jw >= 0.8 {
            wtr.write_record(&[jw.to_string()]).unwrap();
        }
        jw
        //dbg!(name, &score); 
        //dbg!(jw, &jw_eddie);
        //(1.0 / 3.0) * ( score.matches as f32 / score.len as f32 + score.matches as f32 / query_len as f32 + (score.matches - score.transposition_count) as f32/ score.matches as f32)
    }).collect::<Vec<_>>();*/
    /*.map(|score| { 
        //let jw_eddie = jaro.similarity(&query, &name);
        let jw = (1.0 / 3.0) * ( score.matches as f32 / score.len as f32 + score.matches as f32 / query_len as f32 + (score.matches - score.transposition_count) as f32/ score.matches as f32);
        //let jw = 0.7;
        jw
        /*if (jw - jw_eddie as f32).abs() > 0.01 {
            wtr.write_record(&[name, jw_eddie.to_string(), jw.to_string()]).unwrap();
        }*/
        //dbg!(name, &score); 
        //dbg!(jw, &jw_eddie);

    }).filter(|&jw| jw > 0.8).collect::<Vec<_>>();*/
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


