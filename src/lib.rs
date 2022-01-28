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

#[cfg(test)]
mod tests {
    use super::compare_two_strings;

    #[test]
    fn check() {
        let result: f64 = compare_two_strings("Night", "Nacht");
        assert_eq!(result, 0.25);
        assert_ne!(result, 0.5);
    }

    #[test]
    fn check_space() {
        let result: f64 = compare_two_strings("Night Night Night", "Nacht Nacht Nacht");
        assert_eq!(result, 0.35714285714285715);
        assert_ne!(result, 0.5);
    }
}
