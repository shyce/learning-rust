
use std::io::{self, Write};
use std::str::FromStr;

// Generic version:
pub fn read<T: FromStr>() -> T 
where
  <T as FromStr>::Err: std::fmt::Debug,
{
  let mut user_input = String::new();
  io::stdin()
      .read_line(&mut user_input)
      .expect("Could not read user input.");
  user_input
      .trim()
      .parse::<T>()
      .expect("Invalid entry. Try again.")
}

pub fn create_menu<T>(name: &str, items: &Vec<&str>) -> String {
  println!("\n---------------");
  println!("{}", name.to_string());
  println!("---------------");
  for (index, item) in items.iter().enumerate() {
    println!("{}. {}", index + 1, item)
  }
  print!("Enter your choice (or type exit): ");
  io::stdout().flush().expect("Could not flush stdout"); 
  read()
}
