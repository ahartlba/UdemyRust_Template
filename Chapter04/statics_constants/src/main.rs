const THRESHOLD: i32 = 10; // entspricht #define in c/c++ (Textersetzung)
static _THRESHOLD2: i32 = 10; // normale Konstante

fn is_above_threshold(num: i32) -> bool {
    num > THRESHOLD
}
fn main() {
    let value = 100;

    println!("{}", is_above_threshold(value));
}
