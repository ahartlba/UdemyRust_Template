// As many immuatable references as we want, but then no mutable reference
// Only one mutable reference, but then no immuatable references

fn main() {
    let mut s = String::from("Jan");
    {
        let _r1 = &s; // not a pointer, but variable that has same adress
        let _r2 = &s;
        // let _r3 = &mut s; // does not work (as stated above)

        println!("{} {}", _r1, _r2);
    }
    {
        // new scope
        let _r1 = &mut s;
        // let _r2 = & s; // also not possible

        println!("{}", _r1);
    }
}
