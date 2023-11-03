fn transform(y: &u32) -> u32 {
  y * y
}

fn filter(y: &u32) -> bool {
  *y % 2 == 1
}

fn main() {
  let nums = vec![1,2,3,4,5];
  let res = nums.iter()
                .map(transform)
                .filter(filter)
                .collect::<Vec<u32>>();
                
  println!("{:?}", res);
}
