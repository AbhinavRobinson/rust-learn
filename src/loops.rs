pub fn run() {
  // let mut count = 0;

  // // inf loop
  // loop {
  //   count += 1;
  //   println!("{}", count);

  //   // stop loop
  //   if count == 20 {
  //     break;
  //   }
  // }

  // count = 1;
  // // while loop (fizzbuzz)
  // while count <= 100 {
  //   if count % 15 == 0 {
  //     println!("FizzBuzz");
  //   } else if count % 5 == 0 {
  //     println!("Buzz");
  //   } else if count % 3 == 0 {
  //     println!("Fizz");
  //   } else {
  //     println!("{}", count);
  //   }
  //   count += 1;
  // }

  // For Range
  for x in 0..101 {
    if x % 15 == 0 {
      println!("FizzBuzz");
    } else if x % 5 == 0 {
      println!("Buzz");
    } else if x % 3 == 0 {
      println!("Fizz");
    } else {
      println!("{}", x);
    }
  }
}
