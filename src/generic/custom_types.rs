
#![allow(dead_code)]


// 3 types of struc(ures) that can be created
#[derive(Debug)]
struct Person {
  name: String,
  age: u8,
}

// unit struct
#[derive(Debug)]
struct Unit;

// tuple struct
struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
  x: f32,
  y: f32,
}

#[derive(Debug)]
struct Rectangle {
  top_left: Point,
  bottom_right: Point,
}

fn area(rect: &Rectangle) -> f32 {
  let width = (rect.top_left.x - rect.bottom_right.x).abs();
  let h = (rect.top_left.y - rect.bottom_right.y).abs();

  width * h
  // width & h; ====> help: remove this semicolon to return this value
}

fn main () {
  // let name = String::from("Peter");
  // let name = "Peter"; won't work: expects String but is &str
  // String - owned, updateable string type, no immutable references, heap-allocated, resizeable.
  // &str - reference to a string "string slice" or "string reference", memory, not resizeable.
  // String -owned
  // &str - borrowed
  let name = "Peter".to_string();
  let age = 32;
  let peter = Person {name, age};

  println!("{:?}", peter);

  let point: Point = Point { x: 12.43, y: 4.12 };
  println!("point coordinates: ({}, {}, {:?})", point.x, point.y, point);

  // use point as base. like {..obj, x: 1} in JS.
  // let bottom_right = Point { ..point, x: 5.2 }; Won't work bc base struct needs to be the last element.
  let bottom_right = Point { x: 5.2, ..point };
  println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

  // destructure and rename vars using let
  let Point { x: left_edge, y: top_edge } = point;
  println!("x=> left_edge: {}", left_edge);
  println!("y=> top_edge: {}", top_edge);

  let rectangle = Rectangle {
      // struct instantiation is an expression too
      top_left: Point { x: left_edge * 5.0, y: top_edge + 5.0 },
      // both work: 
      // bottom_right: bottom_right,
      bottom_right,
  };
  println!("rect: {:?}", rectangle);

  // Instantiate a unit struct
  let unit = Unit;
  println!("Unit - {:?}", unit);

  let pair = Pair(1,3.3);
  // Access the fields of a tuple struct
  println!("pair contains {:?} and {:?}", pair.0, pair.1);

  // Destructure a tuple struct
  let Pair(integer, decimal) = pair;
  println!("pair contains {:?} and {:?}", integer, decimal);

  let a = area(&rectangle);
  println!("area: {}", a);
}