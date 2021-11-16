use itertools::Itertools;
use std::path::PathBuf;
use std::fs::{File, create_dir_all};
use std::io::BufWriter;
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::io::prelude::*;

#[derive(Debug, Clone)]
pub struct CandidateLetterInfo {
    pub name_index: usize,
    pub len: usize,
    pub mask: u16
}

#[derive(Debug, Clone)]
pub struct LetterInfo {
    len: usize,
    mask: u16
}

#[derive(Debug, Clone)]
pub struct CandidateScore {
    matches: u16,
    increment: u16,
    used: u16,
    used_exact: u16,
    partial_jw: u16,
    last_match_letter_index: u16,
    transposition_count: u8
}

impl CandidateScore {
    pub fn new(increment: u16) -> CandidateScore{
        CandidateScore { matches: 0, used_exact: 0, partial_jw: 0, used: 0, last_match_letter_index: 0, transposition_count: 0, increment }
    }

    #[inline]
    pub fn calculate_jaro_winkler(&self) -> f32 {
        let jaro_partial = ((self.partial_jw as f32 / 1000.0)  + 1.0 - (self.transposition_count as f32 / self.matches as f32)) / 3.0;
        let l = (self.used_exact & 0b1111u16).trailing_ones() as f32;
        jaro_partial + 0.1 * l * (1.0 - jaro_partial)
    }
}

pub fn maskify(query: &String) -> Vec<(u8, [u16; 16])> {
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

pub fn build_candidate_lookup(names: &Vec<String>) -> Vec<Vec<CandidateLetterInfo>> {
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
pub fn score_letter(candidate_score: &mut CandidateScore, query_mask: u16, candidate_mask: u16, query_index: usize, query_increment: u16) {
    let whole_mask_result = query_mask & candidate_mask; // Get raw matches
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
}

#[inline]
pub fn compare_batches(mut output_dir: PathBuf, query_names: &Vec<String>, candidate_names: &Vec<String>, min_jaro_winkler: f32) {
    create_dir_all(&mut output_dir).unwrap();
    let base_candidate_lookup = build_candidate_lookup(&candidate_names);
    let base_candidate_scores = candidate_names.iter().map(|name| {
        CandidateScore::new(((1.0 / name.len() as f32) * 1000.0) as u16)
    }).collect::<Vec<CandidateScore>>();
    query_names.par_iter().progress_count(query_names.len() as
      u64).enumerate().for_each(|(i, query_name)| {
        let mut output_path = output_dir.clone();
        let mut file_name = i.to_string();
        file_name.push_str(".txt");
        output_path.push(file_name);
        let mut file = BufWriter::with_capacity(100000, File::create(output_path).unwrap());
        let query_len = query_name.len();
        let query_increment = ((1.0 / query_len as f32) * 1000.0) as u16;
        let query_masks_lookup = maskify(&query_name);

        let mut candidate_scores = base_candidate_scores.clone();
        for (query_index, (letter_index, query_mask_by_candidate_len)) in query_masks_lookup.iter().enumerate() {
             base_candidate_lookup[*letter_index as usize].iter().for_each(|c_info| {
                let candidate_score = &mut candidate_scores[c_info.name_index];
                let query_mask = query_mask_by_candidate_len[c_info.len - 1];
                score_letter(candidate_score, query_mask, c_info.mask, query_index, query_increment);
             });
        }
        candidate_scores.into_iter().enumerate().flat_map(|(i, score)| {
            let jw = score.calculate_jaro_winkler();
            if jw >= min_jaro_winkler {
                Some((i, jw))
            } else { None}
        }).for_each(|(i, jw)| { writeln!(file, "{},{:.2}", i, jw).unwrap(); });
    });
}


#[cfg(test)]
mod tests {
    use crate::compare_batches;
    use serde::{Serialize, Deserialize};
    use std::path::PathBuf;
    use std::fs::{read_dir, remove_dir_all};

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
        remove_dir_all(output_dir.clone()).unwrap();
        compare_batches(output_dir.clone(), &query_names, &candidate_names, 0.0);
        let output_paths = read_dir(output_dir).unwrap().collect::<Vec<_>>();
        let answer_paths = read_dir(PathBuf::from("tests/answer/")).unwrap().collect::<Vec<_>>();
        assert_eq!(output_paths.len(), answer_paths.len(), "# of files differ -- output: {}, answer: {}", output_paths.len(), answer_paths.len());
        output_paths.into_iter().zip(answer_paths).for_each(|(output_path, answer_path)| {
            let output_path = output_path.unwrap();
            let answer_path = answer_path.unwrap();
            let mut output_reader = csv::ReaderBuilder::new().has_headers(false).from_path(output_path.path()).unwrap();
            let mut answer_reader = csv::ReaderBuilder::new().has_headers(false).from_path(answer_path.path()).unwrap();
            let output_results = output_reader.deserialize().map(|rec| rec.unwrap()).collect::<Vec<ResultRec>>();
            let answer_results = answer_reader.deserialize().map(|rec| rec.unwrap()).collect::<Vec<ResultRec>>();
            output_results.iter().zip(answer_results).for_each(|(o_result, a_result)| {
                assert_eq!(o_result.id, a_result.id, "file: {:?}, o_result: {:?}, a_result: {:?}", output_path, o_result, a_result);
                assert!(o_result.jw - a_result.jw < 0.02, "file: {:?}, o_result: {:?}, a_result: {:?}", output_path, o_result, a_result);
            });
        });

    }
}
