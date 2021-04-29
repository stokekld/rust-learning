fn main() {
    let a_number = 10; // error: cannot assign twice to immutable variable `a_number`
    let mut other_number = 10;
    let a_boolean = true;

    println!("the mutable number {}", other_number);

    other_number = 15;

    println!("the number {}", a_number);

    let a_number = a_number + 5; // shadowing
    println!("the number {}", a_number);

    println!("the boolean {}", a_boolean);
    println!("the mutable number {}", other_number);

    let number32: u32 = "42".parse().expect("Not a number!"); // type annootation

    println!("The number of 32 bits {}", number32);

    // Floating point
    let x = 2.0;      // f64, default type
    let y: f32 = 3.0; // f32, via type annotation
    println!("floating point vars {} and {}", x, y);

    // Math operations
    // Integer Division
    println!("9 / 2 = {}", 9u32 / 2);
    // Float Division
    println!("9 / 2 = {}", 9.0 / 2.0);

    // booleans
    let is_bigger = 1 > 4;
    println!("{}", is_bigger);  // prints "false"

    // strings
    let mut hello = String::from("Hello, ");  // create a String from a string literal
    hello.push('w');                          // push a character into our String
    hello.push_str("orld!");                  // push a string literal into our String
    println!("{}", hello);

    // Tuples
    let tuple = ("hello", 5, 'c');

    assert_eq!(tuple.0, "hello");
    assert_eq!(tuple.1, 5);
    assert_eq!(tuple.2, 'c');
}
