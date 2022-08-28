fn first_word(s: &str) -> &str {
    //str is NOT String !!! str is string literal!
    let bytes = s.as_bytes();

    for (i, &character) in bytes.iter().enumerate() {
        if character == b' ' {
            return &s[0..i];
        }
    }

    return &s;
}

fn main() {
    let mut hello = String::from("Hello world!"); //per default immutable -> mut

    println!("{}", hello);
    hello.push('w');
    println!("{}", hello);

    // funktioniert nicht! wegen verschiedener character-größen
    // println!("{}", hello[0]);

    for b in hello.bytes() {
        println!("{}", b);
    }

    for c in hello.chars() {
        println!("{}", c);
    }

    let word = first_word(&hello);
    println!("First word: {}", word);
}
