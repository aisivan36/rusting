use std::io::stdin;

pub fn user_capture() {
    println!("What is your name?");
    let name = what_is_your_name(); // initialize it

    println!("hi! {}", name);
}

/// Make it return function to String
///The function signature is very similar to the main function. The function
///name is different, and -> String denotes that it returns a String
fn what_is_your_name() -> String {
    let mut your_name = String::new();

    stdin() // it return an object granting acces to the Standard Input.
        .read_line(&mut your_name) //"&mut" borrow the variable allowing changes to be made to your variable by the called function.
        .expect("Failed to read"); //unwrap a result object. and terminate the program with the specified message if an error has occured.

    your_name // This line doesn't end with semicolo ; this is the rust shorthand for return
        .to_lowercase()
}
