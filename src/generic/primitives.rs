fn main () {
  // let logical: bool = true;

  // let a_float: f64 = 1.9;
  // let an_integer = 5i32;
  // let an_integer2: i32 = 5;

  // let default_float = 3.324;
  // let default_integer = 1213412;

  // let mut inferred_type = 12;
  // inferred_type = 123;

  // let arr = [1,2,3];
  // let tup = (3,4,2);

  // integers 2, 3,5 - hex, octal or binary 0x, 0o or 0b
  // 1000 same as 1_000 or 0.000001 is same as 0.000_001
  // floats 2.3
  // characters 'a'
  // strings "asdf"
  // booleans  true/false
  // unit type ()
  // Integer addition
  println!("1 + 2 = {}", 1u32 + 2);

  // Integer subtraction
  println!("1 - 2 = {}", 1f32 - 2.3);
  // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

  // Scientific notation
  println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

  // Short-circuiting boolean logic
  println!("true AND false is {}", true && false);
  println!("true OR false is {}", true || false);
  println!("NOT true is {}", !true);

  // Bitwise operations
  println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
  println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
  println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
  println!("1 << 5 is {}", 1u32 << 5);
  println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

  // Use underscores to improve readability!
  println!("One million is written as {}", 1_000_000u32);
}