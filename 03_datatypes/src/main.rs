fn main() {
    // datatypes
    // 1. scalar
    //  integer
    let int: i8 = 20;
    println!("the int in {int}");
    // floating-point numbers
    let float = 10.2;
    println!("the float is {float}");

    // Boolean
    let boolean: bool = true;
    println!("the bool is {boolean}");
    // characters (is denoted by singal quote ')
    let character = 'a';
    println!("the char is {character}");

    // 2. compound
    // tuples
    let tuple = (10, 20, 24.5, "the str", 'a');

    println!("the tuple is {tuple:?}"); // debug
    println!("the prety print is {tuple:#?}");

    // arrays
    let arr = [1, 2, 3, 4, 5];

    println!("the array is {arr:#?}");

    // access
    // tuples
    let tuple = (10, 20);
    println!(
        "the first or tuple is {} and the second is {}",
        tuple.0, tuple.1
    ); // by index 
    let (first, second) = tuple; // destructuring
    println!("the destructured values is {first}, {second}");

    // array
    let arr = [10, 20, 30];

    println!(
        "the first of arr is {} ,the second of arr is {} and the last is {}",
        arr[0], arr[1], arr[2]
    );

    let [first, second, _] = arr;

    println!("the destructured values qre {first},{second}");
}
