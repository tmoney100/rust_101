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
  let mut scores = HashMap::new();

  build_scores(&mut scores);

  let blue_team = String::from("Blue");
  let score = scores.get(&blue_team); // get returns Option<&V> which is Some(&10) or None
  
  println!("a: {:?}", score);
  
  let blue = scores.get("Blue");
  println!("b: {:?}", blue);


  for (key, value) in &scores {
    println!("k:{}", key);
    println!("v:{}", value);
  }
}
