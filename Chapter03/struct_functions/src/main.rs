#[derive(Debug)]
struct Circle {
    radius: f32,
}

// "implement Circle"
impl Circle {
    fn compute_area(&self) -> f32 {
        self.radius * self.radius * 3.14159
    }

    fn compute_circumference(&self) -> f32 {
        2.0 * self.radius * 3.14159
    }

    fn smaller(&self, other: &Self) -> bool {
        //Self ist immer struct-name
        self.radius < other.radius
    }
}

fn main() {
    let c1 = Circle { radius: 1.0 };
    let c2 = Circle { radius: 2.0 };

    println!("Area: {}", c1.compute_area());
    println!("Circumference: {}", c1.compute_circumference());
    println!("c1 < c2: {}", c1.smaller(&c2));
}
