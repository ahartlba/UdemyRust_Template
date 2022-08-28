fn main() {
    let numer = 13;

    match numer {
        1 => {
            println!("One!")
        } // auch geschweifte Klammern möglich
        2 | 3 | 5 | 7 | 11 => println!("Prime!"),
        _ => println!("None!"), //default case
    }

    let boolean = true;

    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);
}
