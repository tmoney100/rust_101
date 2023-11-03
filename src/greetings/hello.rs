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

// mod(ule) - namespace that groups related definitions
pub mod utils {
  pub fn print_hello() {
    println!("Hello, World!")
  }
  pub fn test () {
    const dir: Direction = Direction::North;
    match dir {
      Direction::North => println!("Going North!"),
      Direction::South => println!("Going South"),
      _ => println!("invalid direction")
    }
  }
}

// trait- collection of methods
trait Printable {
  fn print(&self);
}

struct Person {
  name: String,
}

impl Printable for Person {
  fn print(&self) {
    println!("Name: {}", self.name)
  }  
}

// impl(ementation)
struct Circle {
  radius: f64
}

impl Circle {
  fn area(&self) -> f64 {
    3.14159 * self.radius * self.radius
  }
}



// const(ant) - unchangeable 
const MAX_VALUE: i32 = 100;
// static - fixed address in memory; internally mutable.
static GLOBAL_VAR: i32 = 10;
// type alias - gives another name to an existing type; used to reduce verbosity.
type Integer = i32;
type Int = Integer;


// macro - 
macro_rules! say_hello {
  () => {
    println!("Hello, World!");
  };
}
say_hello!();

// union - struct that can hold multiple types of data, but only one at a time.
union IntOrFloat {
  i: i32,
  f: f32
}

const d1: IntOrFloat = IntOrFloat { i: 42 };
const d2: IntOrFloat = IntOrFloat { f: 43.234234234 };
unsafe fn dangerous() {
  println!("Data as integer: {}", data.i);
  println!("Data as integer: {}", data.f);
}

// &beforeVar is "borrowing" means your working with a reference3 to the value rather than the value itself.
// self      - consumes the instance, can't use the instance after the call
// &self     - borrows the instance, immutably. can't mod the instance.
// &mut self - borrows the instance, mutable. can mod the instance.

impl Point {
  // Takes ownership of the instance
  fn consume(self) {}
  
  // Borrows the instance immutably
  fn inspect(&self) {
      println!("({}, {})", self.x, self.y);
  }
  
  // Borrows the instance mutably
  fn move_by(&mut self, dx: i32, dy: i32) {
      self.x += dx;
      self.y += dy;
  }
}


pub fn main2() {
  utils::print_hello();
  utils::test();
  const p: Point = Point {x: 4, y: 3};
  println!("x: {}. y: {}", p.x, p.y);
  const r: i32 = add(1,3);
  println!("result: {}", r);
  const person: Person = Person { name: String::from("Alice") };
  person::print!();
  const c: Cicle = Circle { radius: 3.0 };
  println!("Area: {}", c.area());
  const num: Integer = 42;
  const num2: Int = 43;
  println!("Number: {}, {}", num, num2);
}