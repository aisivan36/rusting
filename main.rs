#![warn(clippy::all, clippy::pedantic)]
// use std::io;
// mod references;

mod user_capture;

fn main() {
    user_capture::user_capture();

    //  references::references();

    //  let mylist = ["one", "two", "three", "four", "five"];
    //  for i in &mylist {
    //      println!("{}", i);
    //  }

    // println!("Guess the number!");

    // println!("Please input your guess.");

    // let mut guess = String::new();

    // io::stdin()
    //    .read_line(&mut guess)
    //    .expect("Failed to read line");

    // let x = 13;

    // let mut x = x + 12;
    // println!("You guessed: {}", x);

    // x += 12;

    // println!("You guessed: {}", x);
}
