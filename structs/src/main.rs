fn main() {
    let mut user1 = User {
        username: String::from("someuser123"),
        email: String::from("some@user123.com"),
        active: true,
        sign_in_count: 10,
    };
    user1.sign_in_count = 11;

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // this code below trigger compilation error
    // because we move user1 string (do not implement copy trait)
    // println!(
    //     "{0} - {1} - login: {2}",
    //     user1.username, user1.email, user1.active
    // );

    println!(
        "{0} - {1} - login: {2}",
        user2.username, user2.email, user2.active
    );

    let user3 = build_user(String::from("example@three.com"), String::from("three"));
    println!(
        "{0} - {1} - login: {2}",
        user3.username, user3.email, user3.active
    );
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        // field init shorthand; no need for username: username
        username,
        email,
        sign_in_count: 1,
    }
}
