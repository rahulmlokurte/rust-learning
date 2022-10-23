fn main() {
    println!("Learning Structs");
    let user1 = User{
        username: String::from("Rahul Lokurte"),
        active: true,
        email: String::from("rahul.m.lokurte@gmail.com"),
        sign_in_count: 1
    };
    println!("The username is {}", user1.username);
    println!("The active is {}", user1.active);
    println!("The email is {}", user1.email);
    println!("The sign_in_count is {}", user1.sign_in_count);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}
