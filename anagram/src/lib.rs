use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result: HashSet<&'a str> = HashSet::new();

    let word = word.to_lowercase();
    let mut word_chars: Vec<char> = word.chars().collect();
    word_chars.sort_unstable();

    possible_anagrams.iter()
      .filter(|&possible_anagram| {
        possible_anagram.to_lowercase() != word
      })
        .filter(|&possible_anagram| {
          let mut possible_anagram_chars: Vec<char> = possible_anagram.to_lowercase().chars().collect();
          possible_anagram_chars.sort_unstable();

          possible_anagram_chars == word_chars
        })
        .for_each(|&possible_anagram| {
          result.insert(possible_anagram);
        });

    result
}
