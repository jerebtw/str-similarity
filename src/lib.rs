use std::collections::BTreeMap;

pub struct MatchResult {
    pub score: f64,
    pub string: String,
}

pub struct BestMatchResult {
    pub best_result_index: usize,
    pub result: Vec<MatchResult>,
}

#[inline]
fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

#[inline]
pub fn compare_two_strings(first: &str, second: &str) -> f64 {
    let first = remove_whitespace(first);
    let second = remove_whitespace(second);

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

    (2.0 * intersection_size) / (first_len + second_len - 2) as f64
}

#[inline]
pub fn find_best_match(string: &str, arr: Vec<&str>) -> BestMatchResult {
    let mut result: Vec<MatchResult> = vec![];
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

#[cfg(test)]
mod tests {
    use super::{compare_two_strings, find_best_match};

    #[test]
    fn check_compare() {
        let result: f64 = compare_two_strings("Night", "Nacht");
        assert_eq!(result, 0.25);
        assert_ne!(result, 0.5);
    }

    #[test]
    fn check_compare_space() {
        let result: f64 = compare_two_strings("Night Night Night", "Nacht Nacht Nacht");
        assert_eq!(result, 0.35714285714285715);
        assert_ne!(result, 0.5);
    }

    #[test]
    fn check_best_match() {
        let result = find_best_match("Night", vec!["Nacht", "Night", "Nacht"]);
        assert_eq!(result.result.len(), 3);
        assert_eq!(result.result[0].score, 0.25);
        assert_eq!(result.result[1].score, 1.0);
        assert_eq!(result.result[2].score, 0.25);
        assert_eq!(result.best_result_index, 1);
    }

    #[test]
    fn check_best_match_space() {
        let result = find_best_match(
            "Night Night Night",
            vec!["Nacht Nacht Nacht", "Night Night Night", "Night"],
        );
        assert_eq!(result.result.len(), 3);
        assert_eq!(result.result[0].score, 0.35714285714285715);
        assert_eq!(result.result[1].score, 1.0);
        assert_eq!(result.result[2].score, 0.44444444444444444);
        assert_eq!(result.best_result_index, 1);
    }
}
