use std::mem;

pub fn run() {
  let mut numbers: [u8; 5] = [1, 2, 3, 4, 5];

  println!("{:?}", numbers);

  // get single value
  println!("Single Value: {}", numbers[0]);

  // reassign value
  numbers[2] = 10;

  // get single value (modified)
  println!("Single Value: {}", numbers[2]);

  // get length
  println!("Array Length: {}", numbers.len());

  // Memory
  println!("Array Size (u8) (bytes): {}", mem::size_of_val(&numbers));
  // size of a i32
  let i32array: [i32; 5] = [1, 2, 3, 4, 5];
  println!("Array Size (i32) (bytes): {}", mem::size_of_val(&i32array));
  // 4x bigger array

  // get slice
  let slice: &[i32] = &i32array;
  println!("Slice: {:?}", slice);
}
