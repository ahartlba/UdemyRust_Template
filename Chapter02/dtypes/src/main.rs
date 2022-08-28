fn main() {
    /* INTEGERS */

    let x0 = 0; //std integer 32 bit signed

    // signed integers
    let _x1: i8 = 0; // underscore prefix to remove unused variable warning
    let _x2: i16 = 0;
    let _x3: i32 = 0;
    let _x4: i64 = 0;
    let _x5: i128 = i128::MIN; //get min value of datatype

    // unsigned integers
    let _x1: u8 = u8::MAX; // get max value of datatype
    let _x2: u16 = 0;
    let _x3: u32 = 0;
    let _x4: u64 = 0;
    let _x5: u128 = 0;

    println!("{}, {}, {}", _x1, _x2, _x3);

    // auch möglich
    let _dec = 255;
    println!("dec: {}", _dec);
    let _hex = 0xff;
    println!("dec: {}", _hex);
    let _bin = 0b11111111;
    println!("dec: {}", _bin);

    /* FLOATS */
    let _f1 = 2.0;
    let _f2: f64 = 0.0;
    let _f3: f32 = 0.0;

    let _f4 = 2.0_f32; // für numerische datentypen auch mit Unterstrich möglich
    let _f5 = 2.0_f64;

    /* BOOLEAN */
    let _flag1 = true;
    let _flag2 = false;
}
