// use std::mem; 

fn main () {
  // array all same type T; contiguious memory; length known at compile time; signature: [T; length]
  let xs: [i32; 5] = [1,2,3,4,5];
  let ys: [i32; 500] = [0; 500];

  println!("First element of the array: {}", xs[0]);
  println!("Second element of the array: {}", xs[1]);

    // `len` returns the count of elements in the array.
  println!("Number of elements in array: {}", xs.len()); 
  println!("Number of elements in array: {}", ys.len());  

  println!("Borrow a section of the array as a slice.");
  // start a 1 .. end at 4, but don't include 4.
  analyze_slice(&ys[1 .. 4]);

  let empty_array: [u32; 0] = [];
  assert_eq!(&empty_array, &[]);
  assert_eq!(&empty_array, &[][..]); // Same but more verbose

  for i in 0..xs.len() + 1 {
    match xs.get(i) {
      Some(xs_i) => println!("{}: {}", i, xs_i),
      None => println!("Slow down! {} is too far!", i),
    }
  }

  for (i, val) in xs.iter().enumerate() {
    println!("i, val: {}, {}", i, val);
  }
}

fn analyze_slice(slice: &[i32]) {
  println!("First element of the slice: {}", slice[0]);
  println!("The slice has {} elements", slice.len());
}