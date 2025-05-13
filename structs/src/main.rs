struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    defining_and_instantiating_structs();
    struct_update_syntax();
    tuple_structs_to_create_types();
    unit_like_structs();
}

fn defining_and_instantiating_structs() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = build_user(String::from("random@example.com"), String::from("John Doe"));

    println!("{}", user2.username);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn struct_update_syntax() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}

fn tuple_structs_to_create_types() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let Point(x, y, z) = origin;
}

fn unit_like_structs() {
    struct AlwaysEqual;
    let subject = AlwaysEqual;
}
