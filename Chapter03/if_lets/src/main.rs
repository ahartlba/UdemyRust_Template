#[derive(Debug)]
enum PermissionLevel {
    User,
    Instructor,
    Admin,
}

impl PermissionLevel {
    fn description(&self) -> String {
        match self {
            PermissionLevel::User => String::from("I am a User"),
            PermissionLevel::Instructor => String::from("I am an Instructor"),
            PermissionLevel::Admin => String::from("I am an Admin"),
        }
    }
    fn is_admin(&self) -> bool {
        // if let PermissionLevel::Admin ... "sind wir im state Admin",
        // ... ja ist verwirrend
        let ret = if let PermissionLevel::Admin = self{
            true
        } else{
            false
        }
        return ret
    }
}

fn main() {
    let user1 = PermissionLevel::Admin;
    println!("{:?}", user1);
    println!("{}", user1.description());
    println!("{}", user1.is_admin());
    let user2 = PermissionLevel::Instructor;
    println!("{:?}", user2);
    println!("{}", user2.description());
    println!("{}", user2.is_admin());
    let user3 = PermissionLevel::User;
    println!("{:?}", user3);
    println!("{}", user3.description());
    println!("{}", user3.is_admin());
}
