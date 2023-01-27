/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {

  if code.chars().any(|c| !c.is_digit(10) && c != ' ') {
    return false;
  }

  let code = code.replace(" ", "");

  if code.len() <= 1 {
    return false;
  }

  code.chars()
    .rev()
    .filter_map(|c| c.to_digit(10))
    .enumerate()
    .map(|(i, digit)| if i % 2 == 1 { digit * 2 } else { digit })
    .map(|digit| if digit > 9 { digit - 9 } else { digit })
    .sum::<u32>() % 10 == 0
}
