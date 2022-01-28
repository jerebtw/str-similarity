#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::BTreeMap;

pub fn compare_two_strings(first: &str, second: &str) -> f64 {
    lazy_static! {
        static ref RE: Regex = Regex::new("\\s+").unwrap();
    }

    let first = RE.replace_all(first, "");
    let second = RE.replace_all(second, "");

    if first == second {
        return 1.0;
    }

    let first_len = first.len();
    let second_len = second.len();

    if first_len < 2 || second_len < 2 {
        return 0.0;
    }

    let mut first_val: BTreeMap<&str, i32> = BTreeMap::new();

    for i in 0..first_len - 1 {
        let val = first.get(i..i + 2).unwrap();
        let count = first_val.get(val).unwrap_or(&0) + 1;
        first_val.insert(val, count);
    }

    let mut intersection_size: f64 = 0.0;
    for i in 0..second_len - 1 {
        let val = second.get(i..i + 2).unwrap();
        let count: i32 = *first_val.get(val).unwrap_or(&0);
        if count > 0 {
            intersection_size += 1.0;
            first_val.insert(val, count - 1);
        }
    }

    (2.0 * intersection_size) / (first.len() as f64 + second.len() as f64 - 2.0)
}

pub struct MatchResult {
    pub score: f64,
    pub string: String,
}

pub struct BestMatchResult {
    pub best_result_index: usize,
    pub result: Vec<MatchResult>,
}

pub fn find_best_match(string: &str, arr: Vec<&str>) -> BestMatchResult {
    let mut result: Vec<MatchResult> = Vec::new();
    let mut best_result_index: usize = 0;

    for (index, item) in arr.iter().enumerate() {
        let score = compare_two_strings(string, item);

        result.push(MatchResult {
            score,
            string: item.to_string(),
        });

        if score > result[best_result_index].score {
            best_result_index = index;
        }
    }

    BestMatchResult {
        best_result_index,
        result,
    }
}
