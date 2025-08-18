struct user {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = user {
        active: true,
        username: String::from("someusername"),
        email: String::from("someone@gmail.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@gmail.com");
}