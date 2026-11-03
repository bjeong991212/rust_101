struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("bjeong"),
        email: String::from("byungho.jeong@qraftec.com"),
        sign_in_count: 33,
    };

    user1.sign_in_count = 34;

    let mut user2 = User {
        username: String::from("cjeong"),
        ..user1
    };

    // println!("{}", user1.username) // compile error
    println!("{}", user2.username)
}

fn new_user(email: String, username: String) -> User {
  User {
    active: true,
    username,
    email,
    sign_in_count: 1,
  }
}
