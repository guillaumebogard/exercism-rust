pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
  let mut sum = 0;

  (1..limit).for_each(|x| {
    if factors.iter().any(|factor| *factor != 0 && x % factor == 0) {
      sum += n;
    }
  });

  sum
}
