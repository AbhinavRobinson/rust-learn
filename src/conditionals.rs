pub fn run() {
  let age: u8 = 22;
  let check_id: bool = false;
  let knows: bool = true;

  // if/else
  if age >= 21 && check_id || knows {
    println!("More than 21.");
  } else if age < 21 && check_id {
    println!("Less than 21.");
  } else {
    println!("No ID");
  }

  // short hand
  let is_of_age = if age >= 21 { true } else { false };
  println!("{}", is_of_age);
}
