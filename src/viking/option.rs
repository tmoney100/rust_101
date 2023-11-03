fn find_string_length(s: Option<&str>) -> Option<usize> {
  match s {
      Some(text) => Some(text.len()),
      None => None,
  }
}

fn main() {
  let s = "Hello, Rust!";
  let text = Some(s);
  let absent_text: Option<&str> = None;

  println!("\"{}\":{:?}", s, find_string_length(text));        // Outputs: Some(12)
  println!("{:?}", find_string_length(absent_text)); // Outputs: None
}
