use std::io; // import package

fn main() {
    println!("Please enter your number:");

    let mut guess = String::new(); // constructor leerer string (Heap)

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read number!");

    println!("Your entered: {}", guess)
}
