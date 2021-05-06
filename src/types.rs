pub fn run() {
  // Default is i32
  let x = 1;

  // Default is f64
  let y = 2.5;

  // Add Explicit type
  let z: u8 = 4;

  // Find max size of datatype
  println!("Max i32: {}", std::i32::MAX);
  println!("Max i64: {}", std::i64::MAX);

  // Boolean
  let is_active: bool = true;

  println!("{:?}", (is_active, x, y, z));

  // get boolean from expression
  let is_greater = x > z;

  println!("{} is greater than {} : {}", x, z, is_greater);

  // char (unicode)
  let a1: char = '\u{1F602}';

  println!("{:?}", a1);
}
