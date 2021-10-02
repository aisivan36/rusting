// pub mod owner;

pub fn function() {
  // owner::owner();
  let mut greet = String::from("hello");
  greet.push_str(", world!");

  println!(" {}", greet);
}
