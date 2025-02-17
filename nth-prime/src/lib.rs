pub fn nth(n: u32) -> u32 {
  fn is_prime(n: u32) -> bool {
    if n < 2 {
      return false;
    }
    for i in 2..n {
      if n % i == 0 {
        return false;
      }
    }
    true
  }

  let mut count = 0;
  let mut i = 0;

  while count <= n {
    i += 1;
    if is_prime(i) {
      count += 1;
    }
  }

  i
}
