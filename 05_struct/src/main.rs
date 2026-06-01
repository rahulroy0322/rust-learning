// define struct
#[derive(Debug)]
struct User {
    name: String,
    email: String,
    age: i8,
}

fn main() {
    let user = User {
        name: String::from("Jhon Doe"),
        email: String::from("example@example.com"),
        age: 40,
    };
    println!("the user is {user:#?}");

    let user_2 = User {
        email: String::from("user_2@example.com"),
        ..user
    };
    println!("the copy user is {user_2:#?}");
}
