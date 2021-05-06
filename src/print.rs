pub fn run() {
  // Print to console
  println!("hello from print.rs");
  // Basic formatting
  println!("{} is from {}", "Abhinav", "Dehradun");
  // Positional Args
  println!(
    "{0} is from {1} and {0} likes to {2}",
    "Abhinav", "Dehradun", "Codes"
  );
  // Name Args
  println!(
    "{name} likes to play {activity}",
    name = "Abhinav",
    activity = "Piano"
  );
}
