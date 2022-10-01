fn main() {
    let user = build_user(String::from("hello@example.com"), String::from("Bob"));

    let user_with_new_email = User {
        email: String::from("world@example.com"),
        ..user // this moves the fields from user; user is no longer valid after this
    };

    println!("Hello, world!");
}

struct User {
    active: bool,
    // Why not use &str to allow literals?
    // Decause it's a reference, it means the struct doesn't own the data.
    // Using String means this struct fully owns this data.
    user_name: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, user_name: String) -> User {
    User {
        email,
        user_name,
        active: true,
        sign_in_count: 1,
    }
}
