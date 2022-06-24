// This file is part of the IPUMS's pseudo_jaro_winkler.
// For copyright and licensing information, see the NOTICE and LICENSE files
// in this project's top-level directory, and also on-line at:
//   https://github.com/ipums/pseudo_jaro_winkler

//! Creates fast pseudo jaro winkler scores between two vectors of strings.

#![allow(arithmetic_overflow)]
use itertools::Itertools;
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs::{File, create_dir_all};
use std::io::BufWriter;
use indicatif::ParallelProgressIterator;
use flate2::write::GzEncoder;
use flate2::Compression;
use rayon::prelude::*;
use std::io::prelude::*;
use std::fmt;

#[derive(Debug, Clone)]
struct CandidateLetterInfo {
    name_index: usize,
    len: usize,
    mask: u16
}

#[derive(Clone)]
struct CandidateScore {
    matches: u8,
    used: u16,
    used_exact: u16,
    len_partial: u16,
    last_match_letter_index: u16,
    transposition_count: u8,
}
impl fmt::Debug for CandidateScore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CandidateScore")
         .field("matches", &self.matches)
         .field("used", &format!("{:b}", &self.used))
         .field("used_exact", &format!("{:b}", &self.used_exact))
         .field("last_match_letter_index", &format!("{}", &self.last_match_letter_index))
         .field("transpositions", &format!("{}", &self.transposition_count))
         .finish()
    }
}

impl CandidateScore {
    fn new(len: u8) -> CandidateScore{
        CandidateScore { matches: 0, used_exact: 0, used: 0, transposition_count: 0, last_match_letter_index: 0, len_partial: ((1.0 / len as f64) * 1024.0) as u16 }
    }

    #[inline]
    fn calculate_jaro_winkler(&self, query_partial: u16) -> f32 {
        let transpositions = if self.transposition_count > self.matches / 2 { self.transposition_count - 1 } else { self.transposition_count };
        let partial = ((self.matches as u16 * self.len_partial) + (self.matches as u16 * query_partial)) as f32 / 1024.0;
        let jaro = (partial + 1.0 - (transpositions as f32 / self.matches as f32)) / 3.0;
        let l = (self.used_exact & 0b1111u16).trailing_ones() as f32;
        jaro + 0.1 * l * (1.0 - jaro)
    }
}

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
            for _ in 0..match_distance {
                query_mask = query_mask << 1 | query_mask;
                query_mask = query_mask >> 1 | query_mask;
            }
            masks_by_candidate_len[candidate_len - 1] = query_mask;
        }
       (index, masks_by_candidate_len)
    }).collect()
}

fn build_candidate_lookup(names: &Vec<String>) -> Vec<Vec<CandidateLetterInfo>> {
    let mut letter_lookup: Vec<Vec<CandidateLetterInfo>> = Vec::new();
    for letter in '`'..'{' {
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

#[inline]
fn score_letter(candidate_score: &mut CandidateScore, query_mask: u16, candidate_mask: u16, query_index: usize) {
    let whole_mask_result = query_mask & candidate_mask; // Get raw matches
    let check_used_result = (whole_mask_result | candidate_score.used) ^ candidate_score.used; // Make sure we haven't used that match before
    let last_match_letter_index = (1 << check_used_result.trailing_zeros()) & check_used_result; // Find the first match found
    let mask_result = check_used_result & last_match_letter_index; // Take the first match found
    let is_match_mask = !(((mask_result >> mask_result.trailing_zeros()) & 1) - 1); // All 1s if there is a result, else all 0s
    candidate_score.used |= mask_result;
    candidate_score.used_exact |= mask_result & (1 << query_index);
    candidate_score.matches += (is_match_mask & 1) as u8;
    candidate_score.transposition_count +=  (mask_result - 1 < candidate_score.last_match_letter_index) as u8;
    candidate_score.last_match_letter_index |= mask_result;
}

/// Compares two vectors of strings using the pseudo jaro winkler algorithm.
#[inline]
pub fn pseudo_jaro_winkler(names_a: &Vec<String>, names_b: &Vec<String>, mut output_dir: PathBuf, min_jaro_winkler: f32) {
    let lookup_a_by_name = names_a.iter().enumerate().fold(HashMap::new(), |mut lookup, (i, name)|  { 
        let entry = lookup.entry(name).or_insert(Vec::new());
        entry.push(i);
        lookup
    });
    let mut names_a = names_a.clone();
    names_a.sort();
    names_a.dedup();
    let lookup_a_by_new_id = names_a.iter().enumerate().map(|(i, name)| {
        (i, lookup_a_by_name[name].clone())
    }).collect::<HashMap<_, _>>();

    let lookup_b_by_name = names_b.iter().enumerate().fold(HashMap::new(), |mut lookup, (i, name)|  { 
        let entry = lookup.entry(name).or_insert(Vec::new());
        entry.push(i);
        lookup
    });
    let mut names_b = names_b.clone();
    names_b.sort();
    names_b.dedup();
    let lookup_b_by_new_id = names_b.iter().enumerate().map(|(i, name)| {
        (i, lookup_b_by_name[name].clone())
    }).collect::<HashMap<_, _>>();

    create_dir_all(&mut output_dir).unwrap();
    let base_candidate_lookup = build_candidate_lookup(&names_b);
    let base_candidate_scores = names_b.iter().map(|name| {
        CandidateScore::new(name.len() as u8)
    }).collect::<Vec<CandidateScore>>();
    names_a.par_iter().enumerate().for_each(|(new_a_id, query_name)| {
        let query_masks_lookup = maskify(&query_name);
        let query_partial = ((1.0 / query_name.len() as f32) * 1024.0) as u16;

        let mut candidate_scores = base_candidate_scores.clone();
        for (query_index, (letter_index, query_mask_by_candidate_len)) in query_masks_lookup.iter().enumerate() {
             base_candidate_lookup[*letter_index as usize].iter().for_each(|c_info| {
                let candidate_score = &mut candidate_scores[c_info.name_index];
                let query_mask = query_mask_by_candidate_len[c_info.len - 1];
                score_letter(candidate_score, query_mask, c_info.mask, query_index);
             });
        }
        let a_ids = lookup_a_by_new_id.get(&new_a_id).unwrap();
        let mut a_files = a_ids.iter().map(|a_id| {
            let mut output_path = output_dir.clone();
            let mut file_name = a_id.to_string();
            file_name.push_str(".txt");
            output_path.push(file_name);
            //GzEncoder::new(BufWriter::with_capacity(100000, File::create(output_path).unwrap()), Compression::fast())
            BufWriter::with_capacity(100000, File::create(output_path).unwrap())
        }).collect::<Vec<_>>();
        candidate_scores.into_iter().enumerate().flat_map(|(score_i, score)| {
            let jw = score.calculate_jaro_winkler(query_partial);
            if jw >= min_jaro_winkler {
                Some((score_i, jw))
            } else { None}
        }).for_each(|(score_i, jw)| { 
            //writeln!(file, "{},{:.2}", score_i, jw).unwrap(); 
            let b_ids = lookup_b_by_new_id.get(&score_i).unwrap();
            //writeln!(file, "{}", ids.iter().fold(0_usize, |sum, val| sum + val)).unwrap();
            a_files.iter_mut().for_each(|file| {
                b_ids.iter().for_each(|id| { writeln!(file, "{},{:.2}", id, jw).unwrap(); }); 
            });
        });
    });
}

#[inline]
pub fn strsim_jaro_winkler(names_a: &Vec<String>, names_b: &Vec<String>, mut output_dir: PathBuf, min_jaro_winkler: f32) {
    create_dir_all(&mut output_dir).unwrap();
    names_a.par_iter().progress_count(names_a.len() as
      u64).enumerate().for_each(|(i, name_a)| {
        let mut output_path = output_dir.clone();
        let mut file_name = i.to_string();
        file_name.push_str(".txt");
        output_path.push(file_name);
        let mut file = BufWriter::with_capacity(100000, File::create(output_path).unwrap());
        names_b.iter().enumerate().flat_map(|(name_b_i, name_b)| {
            let jw = strsim::jaro_winkler(name_a, name_b);
            if jw >= min_jaro_winkler as f64{
                Some((name_b_i, jw))
            } else { None}
        }).for_each(|(name_b_i, jw)| { writeln!(file, "{},{:.2}", name_b_i, jw).unwrap(); });
    });
}

#[inline]
pub fn eddie_jaro_winkler(names_a: &Vec<String>, names_b: &Vec<String>, mut output_dir: PathBuf, min_jaro_winkler: f32) {
    create_dir_all(&mut output_dir).unwrap();
    names_a.par_iter().enumerate().for_each(|(i, name_a)| {
        let mut output_path = output_dir.clone();
        let mut file_name = i.to_string();
        file_name.push_str(".txt");
        output_path.push(file_name);
        let mut file = BufWriter::with_capacity(100000, File::create(output_path).unwrap());
        let jaro_winkler = eddie::JaroWinkler::new();
        names_b.iter().enumerate().flat_map(|(name_b_i, name_b)| {
            let jw = jaro_winkler.similarity(name_a, name_b);
            if jw >= min_jaro_winkler as f64{
                Some((name_b_i, jw))
            } else { None}
        }).for_each(|(name_b_i, jw)| { writeln!(file, "{},{:.2}", name_b_i, jw).unwrap(); });
    });
}


#[cfg(test)]
mod tests {
    use crate::pseudo_jaro_winkler;
    use serde::{Serialize, Deserialize};
    use std::path::PathBuf;
    use std::fs::{read_dir, remove_dir_all};
    use statistical::*;

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

    #[derive(Serialize, Deserialize, Debug)]
    struct ResultRec {
        id: String,
        jw: f64
    }

    #[test]
    fn test_batch() {
        let query_names = csv::ReaderBuilder::new().has_headers(false).from_path("./tests/input/prepped_df_b.csv").unwrap().deserialize().map(|rec| {
            let rec: NameRec = rec.unwrap();
            rec.first_name
        }).take(10).collect::<Vec<String>>();

        let candidate_names = csv::ReaderBuilder::new().has_headers(false).from_path("./tests/input/prepped_df_a.csv").unwrap().deserialize().map(|rec| {
            let rec: NameRec = rec.unwrap();
            rec.first_name
        }).take(100000).collect::<Vec<String>>();
        let output_dir = PathBuf::from("./tests/output/");
        remove_dir_all(output_dir.clone()).ok();
        pseudo_jaro_winkler(&query_names, &candidate_names, output_dir.clone(), 0.0);
        let output_paths = read_dir(output_dir.clone()).unwrap().collect::<Vec<_>>();
        let answer_paths = read_dir(PathBuf::from("tests/answer/")).unwrap().collect::<Vec<_>>();
        assert_eq!(output_paths.len(), answer_paths.len(), "# of files differ -- output: {}, answer: {}", output_paths.len(), answer_paths.len());
        assert_eq!(output_paths.len(), 10);
        let mut errors = output_paths.into_iter().zip(answer_paths).flat_map(|(output_path, answer_path)| {
            let output_path = output_path.unwrap();
            let answer_path = answer_path.unwrap();
            let mut output_reader = csv::ReaderBuilder::new().has_headers(false).from_path(output_path.path()).unwrap();
            let mut answer_reader = csv::ReaderBuilder::new().has_headers(false).from_path(answer_path.path()).unwrap();
            let mut output_results = output_reader.deserialize().map(|rec| rec.unwrap()).collect::<Vec<ResultRec>>();
            output_results.sort_by(|result_a, result_b| result_a.id.cmp(&result_b.id));
            let mut answer_results = answer_reader.deserialize().map(|rec| rec.unwrap()).collect::<Vec<ResultRec>>();
            answer_results.sort_by(|result_a, result_b| result_a.id.cmp(&result_b.id));
            output_results.iter().zip(answer_results.iter()).filter(|(_, a_result)| a_result.jw > 0.7).map(|(o_result, a_result)| { (o_result.jw -a_result.jw).abs() as f64 }).collect::<Vec<f64>>()
        }).collect::<Vec<f64>>();
        let mean_error: f64 = mean(errors.as_slice());
        let std_dev = standard_deviation(errors.as_slice(), Some(mean_error));
        errors.sort_by(|a, b| b.partial_cmp(a).unwrap());
        assert!(mean_error < 0.002);
        assert!(std_dev < 0.01);
        let errors_over_two_points_off = errors.iter().filter(|&&e| e > 0.02).count() as f32 / errors.len() as f32;
        assert!(errors_over_two_points_off < 0.02);
        remove_dir_all(output_dir.clone()).unwrap();
    }
}
