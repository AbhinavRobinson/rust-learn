pub fn run() {
  let mut hello = String::from("Hello");

  println!("{}", hello);
  // Get Length
  println!("Length: {}", hello.len());

  // Concat one char
  hello.push(' ');
  // Concat String
  hello.push_str("World");

  println!("{}", hello);

  // Get max capacity
  println!("Capacity: {}", hello.capacity());

  // is empty?
  println!("IsEmpty? {}", hello.is_empty());

  // contains?
  println!("Contains 'World': {}", hello.contains("World"));

  // replace
  println!("Replace: {}", hello.replace("World", "There!"));

  // loop through string
  for word in hello.split_whitespace() {
    println!("{}", word);
  }

  // Create string with capacity
  let mut s = String::with_capacity(10);
  s.push('A');
  s.push('B');

  println!("{}", s);

  // Assertion testing
  assert_eq!(2, s.len());
  assert_eq!(10, s.capacity());
}
