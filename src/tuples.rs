pub fn run() {
  let person: (&str, &str, i8) = ("Abhinav", "Robinson", 20);

  println!("{} {} is {}", person.0, person.1, person.2);
}
