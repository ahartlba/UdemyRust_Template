// Tuples: Fixed length, different dtypes possible

fn main() {
    let mut tpl = (500, "hi", true);

    println!("{:?}", tpl);
    println!("First: \n");
    println!("{:?}", tpl.0);
    println!("{:?}", tpl.1);
    println!("{:?}", tpl.2);
    // println!("{:?}", tpl.3); // results in compile time error

    println!("Second: \n");
    let (x, y, z) = tpl;
    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", z);

    tpl.0 = 20;
    println!("Third");
    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", z);
}
