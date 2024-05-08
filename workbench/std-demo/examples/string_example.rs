#![allow(dead_code)]

fn main() {
    let user1 = User {
        user: String::from("alpha"),
        email: String::from("alpha@example.com"),
        active: true,
        sign_in_count: 1,
    };

    // If we had given user2 new String values for both email and username,
    // and thus only used the active and sign_in_count values from user1,
    // then user1 would still be valid after creating user2.
    // The types of active and sign_in_count are types that implement the Copy trait,
    // so the behavior we discussed in the “Stack-Only Data: Copy” section would apply.
    let user2 = User {
        user: String::from("beta"),
        email: String::from("beta@example.com"),
        ..user1
    };

    // let user2 = user1;

    println!("{:?}", user2);

    println!("{:?}", user1);
}

#[derive(Debug)]
struct User {
    active: bool,
    user: String,
    email: String,
    sign_in_count: u64,
}
