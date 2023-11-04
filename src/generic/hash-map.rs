use std::collections::HashMap;

fn build_scores(scores: &mut HashMap<String, i32>) {
  scores.insert(String::from("Blue"), 10);
  scores.insert(String::from("Red"), 50);
  scores.insert(String::from("Green"), 20);
  scores.insert(String::from("Yellow"), 30);
  scores.insert(String::from("Orange"), 40);
  scores.insert(String::from("Purple"), 60);
  scores.insert(String::from("Pink"), 70);
  scores.insert(String::from("Brown"), 80);
  scores.insert(String::from("Black"), 90);
  scores.insert(String::from("White"), 100);
  scores.insert(String::from("Gray"), 110);
  scores.insert(String::from("Cyan"), 120);
}

fn main () {
  let mut scores: HashMap<String, i32> = HashMap::new();
  // let scores: HashMap<String, i32> = HashMap::new();

  build_scores(&mut scores);

  let blue_team = String::from("Blue");
  let score = scores.get(&blue_team); // get returns Option<&V> which is Some(&10) or None
  
  println!("a: {:?}", score);
  
  let blue = scores.get("Blue");
  println!("b: {:?}", blue);
  let v = scores.entry("Blue".to_string());
  println!("c: {:?}", v);

  let mut initial_order = String::new();
  for key in scores.keys() {
    initial_order.push_str(key);
  }

  for _ in 0..100_000_000 {
    let mut new_order = String::new();

    let scores_clone = scores.clone();
    for key in scores_clone.keys() {
      new_order.push_str(key);
    }
    
    if initial_order != new_order {
      println!("Initial Order: {}", initial_order);
      println!("New Order: {}", new_order);
      println!("order differs");
    }
  }
}
