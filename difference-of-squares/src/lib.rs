pub fn square_of_sum(n: u32) -> u32 {
  let sum_of_first_n_natural_numbers = (1..n + 1).fold(0, |acc, x| acc + x);

  sum_of_first_n_natural_numbers.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
  (1..n + 1)
    .map(|x| x.pow(2))
    .fold(0, |acc, x| acc + x)
}

pub fn difference(n: u32) -> u32 {
  square_of_sum(n) - sum_of_squares(n)
}
