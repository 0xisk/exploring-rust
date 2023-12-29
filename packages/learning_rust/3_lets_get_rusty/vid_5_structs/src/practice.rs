struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("Bob.com"),
        username: String::from("Bob"),
        active: true,
        sign_in_count: 1,
    };

    let name = user1.username;
    user1.username = String::from("BobNew");

    let user2 = build_user(String::from("Bob2.com"), String::from("Bob"));

    let user3 = User {
        email: String::from("Bob.com"),
        username: String::from("Bob"),
        ..user1
    };

    // Tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
}
 
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
