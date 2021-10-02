pub fn mut_ref() {
  println!("mutable reference == ====== = == = =");
  let mut my_number = 8;
  let num_ref = &mut my_number;
  *num_ref += 10;
  println!("{}", my_number);

  let second_number = 800;
  let triple_reference = &&&second_number;
  println!("{}", triple_reference);
}
