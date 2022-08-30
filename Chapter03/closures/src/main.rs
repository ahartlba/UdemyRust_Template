fn main() {
    let num = 2;

    let ret = my_function(num);

    println!("{}", ret);

    // closure == lambda == anonymous
    let closure = |inp: i32| -> i32 { inp * 2 };
    let ret2 = closure(num);

    println!("{}", ret2);
}

fn my_function(inp: i32) -> i32 {
    inp * 2 // Zeile ohne semicolon: indirekter Returnaufruf
}
