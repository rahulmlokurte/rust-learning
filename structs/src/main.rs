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
    let s =  build_user(String::from("rahul.m.lokurte@gmail.com"), String::from("Rahul Lokurte"));
    println!("The active is {}", s.sign_in_count);
    let s =  build_user_short_hand(String::from("rahul.m.lokurte@gmail.com"), String::from("Rahul Lokurte"));
    println!("The active is {}", s.sign_in_count);

    let user2 = User {
        active: user1.active,
        username: user1.username,

        email: String::from("user2_rahul.m.lokurte@gmail.com"),
        sign_in_count: user1.sign_in_count
    };

    println!("The email for user2 is {}", user2.email);

    let user3 = User {
        email: String::from("user3_rahul.m.lok"),
        ..user2
    };

    println!("The email for user3 is {}", user3.email);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user_short_hand(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 2,
    }
}
