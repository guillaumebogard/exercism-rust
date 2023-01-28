pub fn verse(n: u32) -> String {
  if n == 0 {
    "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string()
  } else {
    format!("{} of beer on the wall, {} of beer.\nTake {} down and pass it around, {} of beer on the wall.\n",
      match n {
        1 => "1 bottle".to_string(),
        _ => format!("{} bottles", n)
      },
      match n {
        1 => "1 bottle".to_string(),
        _ => format!("{} bottles", n)
      },
      match n {
        1 => "it".to_string(),
        _ => "one".to_string()
      },
      match n - 1 {
        0 => "no more bottles".to_string(),
        1 => "1 bottle".to_string(),
        _ => format!("{} bottles", n - 1)
      }
    )
  }
}

pub fn sing(start: u32, end: u32) -> String {
  (end..start + 1).rev().map(|n| verse(n)).collect::<Vec<String>>().join("\n")
}
