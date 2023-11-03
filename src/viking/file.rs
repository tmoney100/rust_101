use std::fs::File;

fn main() {
  let file = File::open("./src/viking/sample.txt");

  let v = match file {
    Ok(file) => file,
    Err(error) => panic!(
      "problem with {:?}", error
    )
  };

  println!("name: {:?}", v)
}