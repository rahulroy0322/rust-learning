fn main() {
    let s1 = String::from("Hello, world!");

    let s1_copy = s1.clone();

    takes_ownership(s1);
    // s1 no longer valid
    takes_reference(&s1_copy);
}

fn takes_ownership(s: String) {
    println!("ownership str is {s}")
}

fn takes_reference(s: &String) {
    println!("reference str is {s}");
}
