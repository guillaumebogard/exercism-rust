use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
  let mut result = HashMap::new();
  let mut workers = Vec::new();
  let mut input: Vec<String> = input.iter().map(|s| s.to_string()).collect();
  let chunk_size = input.len() / worker_count;
  let remainder = input.len() % worker_count;

  for i in 0..worker_count {
    let mut local_input = Vec::new();
    for _ in 0..chunk_size {
      local_input.push(input.pop().unwrap());
    }
    if i == worker_count - 1 {
      for _ in 0..remainder {
        local_input.push(input.pop().unwrap());
      }
    }

    let worker = thread::spawn(move || {
      let mut local_result = HashMap::new();
      for line in local_input {
        for c in line.chars() {
          if c.is_alphabetic() {
            *local_result.entry(c.to_lowercase().next().unwrap()).or_insert(0) += 1;
          }
        }
      }
      local_result
    });

    workers.push(worker);
  }

  for worker in workers {
    let local_result = worker.join().unwrap();
    for (c, count) in local_result {
      *result.entry(c).or_insert(0) += count;
    }
  }

  result
}
