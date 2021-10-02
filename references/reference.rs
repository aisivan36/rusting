pub fn reference() {
    println!("reference == ====== = == = =");

    let country = String::from("Australia");
    let ref_one = &country;
    let ref_two = &country;
    println!("{}", ref_one);
    println!("{}", ref_two);

    // let theCountry = return_str();
    // println!("{}", theCountry);
}

// fn return_str() -> &str {
//   let country = String::from("Austria");
//   let country_ref = &country;
//   country_ref // ⚠️
// }
