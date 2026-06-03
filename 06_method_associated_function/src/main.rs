#[derive(Debug)]
struct User {
    name: String,
    age: i8,
}

impl User {
    fn new(name: String, age: i8) -> Self {
        Self { name, age }
    }

    fn suggest_email(&self) -> String {
        String::from(format!("{}@example.com", self.name))
    }
}

fn main() {
    let user = User::new(String::from("user"), 20);

    let suggested_email = user.suggest_email();

    println!("the user is {user:#?}");

    println!("suggested email is {suggested_email}");
}
