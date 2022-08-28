fn main() {
    println!("Hello, world!");

    // checker();
    // looper();
    // whiler();
    forer();
}

fn checker() {
    let num = 1;
    if num > 1 {
        println!("Greater");
    } else if num == 1 {
        println!("Equal");
    } else {
        println! {"Smaller"};
    }
}

fn looper() {
    let mut counter = 0;

    // name loop (not necessary)
    'loopname: loop {
        counter += 1;
        println!("{}", counter);

        if counter == 10 {
            break 'loopname; // necessary to stop loop
        }
    }
}

fn whiler() {
    let mut counter = 0;

    'loop_name: while counter < 10 {
        counter += 1;
        println!("{}", counter);
    }
}

fn forer() {
    let nums = 0..11; // [0, ..., 10]

    for num in nums {
        println!("{}", num);
    }
}
