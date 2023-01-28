pub fn build_proverb(list: &[&str]) -> String {
  let mut proverb = String::new();

  if list.is_empty() {
    return proverb;
  }

  list
    .iter()
    .zip(list.iter().skip(1))
    .for_each(|(first, second)| {
      proverb.push_str(&format!("For want of a {} the {} was lost.\n", first, second));
    });
  proverb.push_str(&format!("And all for the want of a {}.", list[0]));

  proverb
}
