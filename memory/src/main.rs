fn process(s: String) {
    println!("{}", s);
}

fn print_greeting(message: &String) {
    println!("Greeting: {}", message);
}

fn change(text: &mut String) {
    text.push_str(", world");
}

// lifetime
fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct Highlight<'document>(&'document str);

fn main() {
    let s = String::from("Hello, world!");
    process(s.clone()); // Passing another value, cloned from `s`.
    process(s); // s was never moved and so it can still be used.

    // borrowing
    let greeting = String::from("hello");
    let _greeting_reference = &greeting; // We borrow `greeting` but the string data is still owned by `greeting`
    println!("Greeting: {}", greeting); // We can still use `greeting`

    let greeting2 = String::from("Hello");
    print_greeting(&greeting2); // `print_greeting` takes a `&String` not an owned `String` so we borrow `greeting` with `&`
    print_greeting(&greeting2); // Since `greeting` didn't move into `print_greeting` we can use it again

    let mut greeting3 = String::from("hello");
    change(&mut greeting3);
    println!("{}", greeting3);

    let magic1 = String::from("abracadabra!");
    let magic2 = String::from("shazam!");

    let result = longest_word(&magic1, &magic2);
    println!("The longest magic word is {}", result);

    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);
    println!("{:?}", fox);
    println!("{:?}", dog);
}