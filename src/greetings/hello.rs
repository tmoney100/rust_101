pub fn hello() {
  println!("Hello, World!");
}

// Struct(ure)
struct Point {
  x: i32,
  y: i32
}

// Enum(eration)
enum Direction {
  North,
  South,
  East,
  West
}

// fn
fn add(a: i32, b:i32) -> i32 {
  a + b
}

// mod(ule)
mod utils {
  pub fn print_hello() {
    println!("Hello, World!")
  }
}

// trait- collection of methods
trait Printable {
  fn print(&self)
}

