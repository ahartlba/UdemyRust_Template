fn main() {
    let s = "Hello World"; // variable mit let

    print!("{}", s);
    println!("{}", s);

    eprint!("{}", s);
    eprintln!("{}", s);

    let name = "Daniel";
    let s2 = format!("Hello {} nice to meet you", name); // string formatting

    // since rust 1.58
    let s3 = format!("Hello {name} nice to meet you", name); // string formatting

    println!("{}", s2);
}
