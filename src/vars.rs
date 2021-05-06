pub fn run() {
  // immutable
  let name = "Abhinav";
  // mutable
  let mut age = 20;

  println!("My name is {} and I am {}", name, age);

  age = 21;

  println!("My name is {} and I am {}", name, age);

  // Defin Constant
  const ID: i32 = 001;

  println!("ID: {}", ID);

  // Assign multiple variables
  let (my_name, my_age) = (name, age);

  println!("{} is {}", my_name, my_age);
}
