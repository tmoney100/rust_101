pub struct Viking {
  pub name: & 'static str,
  pub age: u8,
}

impl Viking {
  pub fn say_hello(&self) {
    println!(
      "Hi, my name is {}. I am {}.",
      self.name, self.age,
    )
  }
}

pub fn example() {
  let ragnar = Viking {
    name: "Ragnar",
    age: 33,
  };

  ragnar.say_hello()
}