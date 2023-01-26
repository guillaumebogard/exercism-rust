pub fn annotate(minefield: &[&str]) -> Vec<String> {
  let mut array: Vec<Vec<u8>> = minefield.iter().map(|s| s.as_bytes().to_vec()).collect();
  let mut result: Vec<String> = Vec::new();

  for i in 0..array.len() {
    let mut row = String::new();

    for j in 0..array[i].len() {
      if array[i][j] == b'*' {
        row.push('*');
        continue;
      }

      let mut count = 0;

      let top_left = i > 0 && j > 0 && array[i - 1][j - 1] == b'*';
      let top = i > 0 && array[i - 1][j] == b'*';
      let top_right = i > 0 && j < array[i].len() - 1 && array[i - 1][j + 1] == b'*';
      let left = j > 0 && array[i][j - 1] == b'*';
      let right = j < array[i].len() - 1 && array[i][j + 1] == b'*';
      let bottom_left = i < array.len() - 1 && j > 0 && array[i + 1][j - 1] == b'*';
      let bottom = i < array.len() - 1 && array[i + 1][j] == b'*';
      let bottom_right = i < array.len() - 1 && j < array[i].len() - 1 && array[i + 1][j + 1] == b'*';

      if top_left {
        count += 1;
      }
      if top {
        count += 1;
      }
      if top_right {
        count += 1;
      }
      if left {
        count += 1;
      }
      if right {
        count += 1;
      }
      if bottom_left {
        count += 1;
      }
      if bottom {
        count += 1;
      }
      if bottom_right {
        count += 1;
      }

      if count > 0 {
        array[i][j] = count + b'0';
      }

      row.push(array[i][j] as char);
    }
    result.push(row);
  }
  result
}
