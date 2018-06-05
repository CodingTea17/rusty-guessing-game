extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  println!("Guess the number!");

  // Generate a random num using the rand crate
  let number_to_guess = rand::thread_rng().gen_range(1, 101);

  println!("The secret number is: {}", number_to_guess);

  println!("Input your guess:");

  let mut guess = String::new();

  io::stdin().read_line(&mut guess)
    .expect("Failed to read line");

  let guess: u32 = guess.trim().parse()
    .expect("Please type a number!");

  println!("You guess: {}", guess);

  match guess.cmp(&number_to_guess) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => println!("You win!"),
  }
}
