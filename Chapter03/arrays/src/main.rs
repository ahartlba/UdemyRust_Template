// Arrays: fixed length, only same dtypes

fn main() {
    let _array = [1, 2, 3];

    println!("{:?}", _array); //:? other format, :#? pretty printing format
    println!("{}", _array[0]);
    println!("{}", _array[1]);
    println!("{}", _array[2]);

    let _array2: [i32; 5] = [1337; 5]; //1337 5mal apspeichern (als i32)

    // println!("{}", _array2[5]); // throws out of bounds exception
}
