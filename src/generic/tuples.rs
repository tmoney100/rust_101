use std::fmt;

fn main () {
  let long_tuple = (
    1u8, 2u16, 3u32, 4u64,
    -1i8, -2i16, -3i32, -4i64,
    0.1f32, 0.2f64,
    'a', true);

  println!("Long tuple var: {}", long_tuple.0);
  println!("Long tuple var: {}", long_tuple.1);
  println!("Long tuple var: {}", long_tuple.2);
  println!("Long tuple var: {}", long_tuple.5);
  println!("Long tuple var: {}", long_tuple.11);
  

  let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
  println!("Long tuple var: {:?}", tuple_of_tuples);
  let ms = MyStruct {name: "Sally".to_string(),age: 3};
  println!("myStruct {:?}", ms);

  let pair = (1, true);
  println!("Pair is {:?}", pair);
  let reverse_pair = reverse(pair);
  println!("Pair is {:?}", reverse_pair);

  // tuple vs literal in parens
  println!("One element tuple: {:?}", (5u32,));
  println!("Just an integer: {:?}", (5u32));

  // destructure
  let tuple = (1, "hello", 4.5, true);
  let (a, b, c, d) = tuple;
  println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

  let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
  println!("{:?}", matrix);
}

fn reverse(pair: (i32, bool)) -> (bool, i32) {
  let (int_param, bool_param) = pair;
  (bool_param, int_param)
}

// // The following struct is for the activity.
// without next line error comes: `Matrix` cannot be formatted using `{:?}`
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

struct MyStruct {
  name: String,
  age: u32,
}

impl fmt::Debug for MyStruct {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "MyStruct looks like {{ name: {}, age: {} }}", self.name, self.age)
  }
}