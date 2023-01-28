pub fn square(s: u32) -> u64 {
  if s == 0 || s > 64 {
    panic!("Square must be between 1 and 64");
  }

  2u64.pow(s - 1)
}

pub fn total() -> u64 {
  let mut total = 1;
  let mut old = 1;

  (2..=64)
    .for_each(|_| {
      let new = old * 2;
      total += new;
      old = new;
    });

  total
}
