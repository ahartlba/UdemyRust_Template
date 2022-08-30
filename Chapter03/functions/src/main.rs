fn main() {
    let num = 2;

    let ret = my_function(num);

    println!("{}", ret);
}

fn my_function(inp: i32) -> i32 {
    inp * 2 // Zeile ohne semicolon: indirekter Returnaufruf
}
