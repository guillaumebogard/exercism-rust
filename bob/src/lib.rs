pub fn reply(message: &str) -> &str {
  let message = message.trim();

  if message.is_empty() {
    "Fine. Be that way!"
  } else if message.ends_with('?') {
    if message.contains(char::is_alphabetic) && message.to_uppercase() == message {
      "Calm down, I know what I'm doing!"
    } else {
      "Sure."
    }
  } else if message.contains(char::is_alphabetic) && message.to_uppercase() == message {
    "Whoa, chill out!"
  } else {
    "Whatever."
  }
}
