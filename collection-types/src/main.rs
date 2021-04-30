use std::collections::HashMap;

fn main() {
    // ARRAYS
    // a comma-separated list inside of brackets
    let weekdays = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
    
    // initialize an array of 512 elements where every element is a zero
    let byte_buffer = [0_u8; 512];

    println!("First day {}", weekdays[0]);
    println!("second element {}", byte_buffer[1]);

    // VECTORS
    let mut a_vec : Vec<char> = Vec::new();
    a_vec.push('a');
    a_vec.push('b');
    a_vec.push('c');
    println!("First vector {:?}", a_vec);

    let three_numbers = vec![1, 2, 3];
    println!("Initial vector: {:?}", three_numbers);  // prints "[1, 2, 3]"

    // the vec! macro also accepts the same syntax as the array constructor
    let ten_zeroes = vec![0; 10];
    println!("Ten zeroes: {:?}", ten_zeroes); // prints [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

    let mut v = Vec::new();  // creates an empty vector,
    v.push(5);               // pushes the number five into it...
    v.push(6);               // ... an then six, and so on
    v.push(7);
    v.push(8);
    println!("{:?}", v); // prints [5, 6, 7, 8]

    let mut new_v = vec![1, 2];
    new_v.pop();
    println!("Poping {:?}, {}", new_v, new_v[0]);

    //HASH
    let mut book_reviews: HashMap<String, String> = HashMap::new();

    // Review some books.
    book_reviews.insert(
        "Adventures of Huckleberry Finn".to_string(),
        "My favorite book.".to_string(),
    );
    book_reviews.insert(
        "Grimms' Fairy Tales".to_string(),
        "Masterpiece.".to_string(),
    );
    book_reviews.insert(
        "Pride and Prejudice".to_string(),
        "Very enjoyable.".to_string(),
    );
    book_reviews.insert(
        "The Adventures of Sherlock Holmes".to_string(),
        "Eye lyked it alot.".to_string(),
    );

    if !book_reviews.contains_key("Les Misérables") {
        println!("We've got {} reviews, but Les Misérables ain't one.",
        book_reviews.len());
    }

    // Searching for an existing key returns the value associated to it
    println!("Review for Jane: {}", book_reviews["Pride and Prejudice"]);

    let sherlock = "The Adventures of Sherlock Holmes";
    assert_eq!(book_reviews.contains_key(sherlock), true);
    book_reviews.remove(sherlock);
    assert_eq!(book_reviews.contains_key(sherlock), false);
}
