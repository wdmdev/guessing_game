fn main() {
    let user1 = User {
        username: String::from("User1"),
        email: String::from("user1@email.com"),
        sign_in_count: 1,
        active: true
    };

    let user2 = User {
        username: String::from("User2"),
        email: String::from("user2@email.com"),
        ..user1 //fill in rest of info from user1
    };

    let mut user3 = create_user(String::from("User3"), String::from("user3@email.com")); //mutable user

    user3.sign_in_count = 2;

}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn create_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true
    }
}