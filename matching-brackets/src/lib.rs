pub fn brackets_are_balanced(string: &str) -> bool {
  let mut stack = vec![];

  string.chars()
    .filter(|c| "()[]{}".contains(*c))
    .for_each(|c| {
      if stack.is_empty() {
        stack.push(c);
      } else {
        match (stack.last().unwrap(), c) {
          ('(', ')') | ('[', ']') | ('{', '}') => { stack.pop(); },
          _ => { stack.push(c); },
        }
      }
    });

  stack.is_empty()
}
