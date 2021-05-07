use std::mem;

pub fn run() {
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

  println!("{:?}", numbers);

  // get single value
  println!("Single Value: {}", numbers[0]);

  // reassign value
  numbers[2] = 10;

  // add values
  numbers.push(5);
  numbers.push(6);
  println!("{:?}", numbers);
  numbers.pop();
  println!("{:?}", numbers);

  // get single value (modified)
  println!("Single Value: {}", numbers[2]);

  // get length
  println!("Vector Length: {}", numbers.len());

  // Memory
  println!("Vector Size (i32) (bytes): {}", mem::size_of_val(&numbers));

  // get slice
  let slice: &[i32] = &numbers[0..2];
  println!("Slice: {:?}", slice);

  // Loop through
  for x in numbers.iter() {
    println!("Number: {}", x);
  }

  // Loop and mutate
  for x in numbers.iter_mut() {
    *x *= 2;
  }
  println!("{:?}", numbers);
}
