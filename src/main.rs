// mod greetings;

// fn main() {
//   greetings::hello::hello();
//   greetings::hello::main2();
//   // greetings::hello::utils::print_hello();
//   // greetings::hello::utils::test();
// }

mod generic;
// mod viking;
// use viking;

fn main() {
  // viking::viking::example()
  // println!("__root__/src/main.rs");
  generic::date::today_to_string(2000, 10, 3);
}