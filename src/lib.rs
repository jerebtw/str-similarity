use std::cmp::{self, min};

pub struct MatchResult {
  pub score: f64,
  pub string: String,
}

pub struct BestMatchResult {
  pub best_result_index: usize,
  pub result: Vec<MatchResult>,
}

#[inline]
pub fn compare_two_strings(first: &str, second: &str) -> f64 {
  let first_len = first.len();
  let second_len = second.len();
  let mut d = vec![0; (first_len + 1) * (second_len + 1)];

  for i in 0..=first_len {
    d[i * (second_len + 1)] = i;
  }
  for j in 0..=second_len {
    d[j] = j;
  }

  for i in 1..=first_len {
    for j in 1..=second_len {
      let cost = if first.get(i - 1..i) == second.get(j - 1..j) {
        0
      } else {
        1
      };
      d[i * (second_len + 1) + j] = min(
        d[(i - 1) * (second_len + 1) + j] + 1,
        min(
          d[i * (second_len + 1) + j - 1] + 1,
          d[(i - 1) * (second_len + 1) + j - 1] + cost,
        ),
      );
      if i > 1
        && j > 1
        && first.get(i - 1..i) == second.get(j - 2..j - 1)
        && first.get(i - 2..i - 1) == second.get(j - 1..j)
      {
        d[i * (second_len + 1) + j] = min(
          d[i * (second_len + 1) + j],
          d[(i - 2) * (second_len + 1) + j - 2] + 1,
        );
      }
    }
  }

  1.0
    - (d[first_len * (second_len + 1) + second_len] as f64) / cmp::max(first_len, second_len) as f64
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
    assert_eq!(result, 0.6);
    let result: f64 = compare_two_strings("Night Night Night", "Night Night Night");
    assert_eq!(result, 1.0);
    let result: f64 = compare_two_strings("Night Night Night", "Nacht Nacht Nacht");
    assert_eq!(result, 0.6470588235294117);
  }

  #[test]
  fn check_best_match() {
    let result = find_best_match("Night", vec!["Nacht", "Night", "Nacht"]);
    assert_eq!(result.result.len(), 3);
    assert_eq!(result.result[0].score, 0.6);
    assert_eq!(result.result[1].score, 1.0);
    assert_eq!(result.result[2].score, 0.6);
    assert_eq!(result.best_result_index, 1);
    let result = find_best_match(
      "Night Night Night",
      vec!["Nacht Nacht Nacht", "Night Night Night", "Night"],
    );
    assert_eq!(result.result.len(), 3);
    assert_eq!(result.result[0].score, 0.6470588235294117);
    assert_eq!(result.result[1].score, 1.0);
    assert_eq!(result.result[2].score, 0.2941176470588235);
    assert_eq!(result.best_result_index, 1);
  }
}
