use lib::{compare_two_strings, find_best_match};

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
