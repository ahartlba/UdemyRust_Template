fn main() {
    let s = "Jan Schaffranek".to_owned();

    /* error (vgl std::move C++) */
    // take_ownership(s);
    // take_ownership2(s);

    take_reference(&s); // immutable reference
    take_reference2(&s);

    take_reference("Hallo"); // immutable reference
    take_reference2("Hallo");
}

fn take_ownership(p: String) {
    println!("{}", p);
}

fn take_ownership2(p: String) {
    println!("{}", p);
}

fn take_reference(p: &str) {
    println!("{}", p);
}

fn take_reference2(p: &str) {
    println!("{}", p);
}
