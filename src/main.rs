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
use packed_simd::u16x16 as u16s;

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
    transposition_count: u16
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
    }).filter(|name| name.len() > 0).take(1000).collect::<Vec<String>>();

    //let mut candidate_names = Vec::new();
    //candidate_names.push("jane a k".to_owned());

    let query = "jake".to_owned();
    let start = Instant::now();
    let base_candidate_lookup = build_candidate_lookup(&candidate_names);
    let base_candidate_scores = candidate_names.iter().map(|name| {
        CandidateScore { matches: 0, used_exact: 0, partial_jw: 0, used: 0, last_match_letter_index: 0, transposition_count: 0, increment: ((1.0 / name.len() as f32) * 1000.0) as u16 }
    }).collect::<Vec<CandidateScore>>();
    query_names.par_iter().progress_count(query_names.len() as u64).for_each(|query_name| {
        let candidate_lookup = base_candidate_lookup.clone();
        let mut candidate_scores = base_candidate_scores.clone();

        let query_len = query.len();
        let query_increment = ((1.0 / query_len as f32) * 1000.0) as u16;
        let query_masks_lookup = maskify(&query);
        //println!("{:?}", candidate_lookup[0].len());

        for (query_index, (letter_index, query_mask_by_candidate_len)) in query_masks_lookup.iter().enumerate() {
            candidate_lookup[*letter_index as usize].chunks_exact(16).for_each(|c_infos| {

                let c_info_masks = u16s::new(
                    c_infos[0].mask,
                    c_infos[1].mask,
                    c_infos[2].mask,
                    c_infos[3].mask,
                    c_infos[4].mask,
                    c_infos[5].mask,
                    c_infos[6].mask,
                    c_infos[7].mask,
                    c_infos[8].mask,
                    c_infos[9].mask,
                    c_infos[10].mask,
                    c_infos[11].mask,
                    c_infos[12].mask,
                    c_infos[13].mask,
                    c_infos[14].mask,
                    c_infos[15].mask
                );

                let query_masks = u16s::new(
                    query_mask_by_candidate_len[c_infos[0].len - 1],
                    query_mask_by_candidate_len[c_infos[1].len - 1],
                    query_mask_by_candidate_len[c_infos[2].len - 1],
                    query_mask_by_candidate_len[c_infos[3].len - 1],
                    query_mask_by_candidate_len[c_infos[4].len - 1],
                    query_mask_by_candidate_len[c_infos[5].len - 1],
                    query_mask_by_candidate_len[c_infos[6].len - 1],
                    query_mask_by_candidate_len[c_infos[7].len - 1],
                    query_mask_by_candidate_len[c_infos[8].len - 1],
                    query_mask_by_candidate_len[c_infos[9].len - 1],
                    query_mask_by_candidate_len[c_infos[10].len - 1],
                    query_mask_by_candidate_len[c_infos[11].len - 1],
                    query_mask_by_candidate_len[c_infos[12].len - 1],
                    query_mask_by_candidate_len[c_infos[13].len - 1],
                    query_mask_by_candidate_len[c_infos[14].len - 1],
                    query_mask_by_candidate_len[c_infos[15].len - 1]
                );

                let used = u16s::new(
                    candidate_scores[c_infos[0].name_index].used,
                    candidate_scores[c_infos[1].name_index].used,
                    candidate_scores[c_infos[2].name_index].used,
                    candidate_scores[c_infos[3].name_index].used,
                    candidate_scores[c_infos[4].name_index].used,
                    candidate_scores[c_infos[5].name_index].used,
                    candidate_scores[c_infos[6].name_index].used,
                    candidate_scores[c_infos[7].name_index].used,
                    candidate_scores[c_infos[8].name_index].used,
                    candidate_scores[c_infos[9].name_index].used,
                    candidate_scores[c_infos[10].name_index].used,
                    candidate_scores[c_infos[11].name_index].used,
                    candidate_scores[c_infos[12].name_index].used,
                    candidate_scores[c_infos[13].name_index].used,
                    candidate_scores[c_infos[14].name_index].used,
                    candidate_scores[c_infos[15].name_index].used
                );

                let whole_mask_result = (query_masks & c_info_masks);
                let check_used_result = (whole_mask_result | used);



                let check_used_result = (whole_mask_result | used) ^ used; // Make sure we haven't used that match before
                let last_match_letter_index = (u16s::splat(1) << check_used_result.trailing_zeros()) & check_used_result; // Find the first match found
                let mask_result = check_used_result & last_match_letter_index; // Take the first match found
                let is_match_mask = !(((mask_result >> mask_result.trailing_zeros()) & 1) - 1); // All 1s if there is a result, else all 0s


                for i in 0..16 {
                    let mask_result_i = mask_result.extract(i as usize);
                    let is_match_mask_i = is_match_mask.extract(i as usize);
                    let mut candidate_score = &mut candidate_scores[c_infos[i].name_index];
                    candidate_score.used |= mask_result_i;
                    candidate_score.used_exact |= mask_result_i & (1 << query_index);
                    candidate_score.matches += is_match_mask_i & 1;
                    candidate_score.partial_jw += is_match_mask_i & candidate_score.increment;
                    candidate_score.partial_jw += is_match_mask_i & query_increment;
                    candidate_score.transposition_count +=  (mask_result_i - 1 < candidate_score.last_match_letter_index) as u16;
                    candidate_score.last_match_letter_index |= mask_result_i;
                }
            });
        }
        candidate_scores.into_iter().filter(|score| {
            let jaro_partial = ((score.partial_jw as f32 / 1000.0)  + 1.0 - (score.transposition_count as f32 / score.matches as f32)) / 3.0;
            let l = (score.used_exact & 0b1111u16).trailing_ones() as f32;
            let jw = jaro_partial + 0.1 * l * (1.0 - jaro_partial);
            jw > 0.8
        }).collect::<Vec<_>>();
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
        //let mut wtr = Writer::from_path("output.csv").unwrap();
        /*candidate_scores.into_iter().zip(candidate_names).for_each(|(score, name)| { 
            //let jw = (1.0 / 3.0) * ( score.matches as f32 / score.len as f32 + score.matches as f32 / query_len as f32 + (score.matches - score.transposition_count) as f32/ score.matches as f32);
            //let jw = (score.partial_jw as f32 + 1.0 - (score.transposition_count as f32 / score.matches as f32)) / 3.0;
            let jaro_partial = ((score.partial_jw as f32 / 1000.0)  + 1.0 - (score.transposition_count as f32 / score.matches as f32)) / 3.0;
            let l = (score.used_exact & 0b1111u16).trailing_ones() as f32;
            let jw = jaro_partial + 0.1 * l * (1.0 - jaro_partial);
            //let jw_strsim = jaro(&query, &name);
            //let jw_strsim = jaro_winkler(&query, &name);
            //let jw = 0.7;
            //dbg!(&[&name, &jw_eddie.to_string(), &jw.to_string()]);
            //if (jw - jw_strsim as f32).abs() > 0.001 {
            //    //wtr.write_record(&[name, jw_strsim.to_string(), jw.to_string()]).unwrap();
            //    dbg!(&[&query, &name, &jw_strsim.to_string(), &jw.to_string()]);
            //}
            if jw > 0.9 {
         //       wtr.write_record(&[name, jw.to_string()]).unwrap();
            }

            //dbg!(name, &score); 
            //dbg!(jw, &jw_eddie);

        });//.filter(|&jw| jw > 0.8).collect::<Vec<_>>();*/
    //let j = JaroWinkler::new();
    /*candidate_names.into_iter().filter(|name| {
        j.similarity(name, &query) > 0.8
    }).collect::<Vec<_>>();*/
    });
    let elapsed = start.elapsed();
    println!("{} ms", elapsed.as_millis());
}
struct StringWrapper<'a>(&'a str);

impl<'a, 'b> IntoIterator for &'a StringWrapper<'b> {
    type Item = char;
    type IntoIter = Chars<'b>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.chars()
    }
}
pub fn jaros(a: &str, b: &str) -> f64 {
    generic_jaro(&StringWrapper(a), &StringWrapper(b))
}

pub fn generic_jaro<'a, 'b, Iter1, Iter2, Elem1, Elem2>(a: &'a Iter1, b: &'b Iter2) -> f64
    where &'a Iter1: IntoIterator<Item=Elem1>,
          &'b Iter2: IntoIterator<Item=Elem2>,
          Elem1: PartialEq<Elem2>,
          Elem1: std::fmt::Debug,
          Elem2: std::fmt::Debug {
    let a_len = a.into_iter().count();
    let b_len = b.into_iter().count();

    // The check for lengths of one here is to prevent integer overflow when
    // calculating the search range.
    if a_len == 0 && b_len == 0 {
        return 1.0;
    } else if a_len == 0 || b_len == 0 {
        return 0.0;
    } else if a_len == 1 && b_len == 1 {
        return if a.into_iter().eq(b.into_iter()) { 1.0} else { 0.0 };
    }

    let search_range = (max(a_len, b_len) / 2) - 1;

    let mut b_consumed = Vec::with_capacity(b_len);
    for _ in 0..b_len {
        b_consumed.push(false);
    }
    let mut matches = 0.0;

    let mut transpositions = 0.0;
    let mut b_match_index = 0;

    for (i, a_elem) in a.into_iter().enumerate() {
        let min_bound =
            // prevent integer wrapping
            if i > search_range {
                max(0, i - search_range)
            } else {
                0
            };

        let max_bound = min(b_len - 1, i + search_range);

        if min_bound > max_bound {
            continue;
        }

        for (j, b_elem) in b.into_iter().enumerate() {
            if min_bound <= j && j <= max_bound && a_elem == b_elem &&
                !b_consumed[j] {
                b_consumed[j] = true;
                matches += 1.0;
                dbg!(a_elem, b_elem, j);

                if j < b_match_index {
                    transpositions += 1.0;
                }
                b_match_index = j;

                break;
            }
        }
    }
    dbg!(transpositions);

    if matches == 0.0 {
        0.0
    } else {
        (1.0 / 3.0) * ((matches / a_len as f64) +
            (matches / b_len as f64) +
            ((matches - transpositions) / matches))
    }
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


