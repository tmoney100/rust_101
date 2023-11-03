fn is_palindrome(x: std::string::String) -> bool {
  let rev = x.chars().rev().collect::<String>();
  return rev == x;
}

// 1
// fn is_palindrome(x: &std::string::String) -> bool {
//   let rev = x.chars().rev().collect::<String>();
//   return rev == *x;
// }

fn main() {
  let name = "anna".to_string();
  let is_palin = is_palindrome(name);
  // 1
  // let is_palin = is_palindrome(&name);
  // 2
  // let is_palin = is_palindrome(name.clone());

  if is_palin {
      println!("{} is a palindrome", name)
  } else {
      println!("{} is not a palindrome", name)
  }
}
