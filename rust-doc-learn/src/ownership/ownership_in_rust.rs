pub fn ownerships() {
    println!("\n====== Ownership in Rust ======");
    let mut s = String::from("String");

    s.push_str(" append!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    {
        let _s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no  longer valid

    let s1 = String::from("hello");
    let s2 = s1; //Shallow cope of s1
    let s3 = s2.clone();

    if s2 == s3 {
        println!("s2 == s3")
    } else {
        println!("s2 != s3")
    }

    // s comes into scope
    let s = String::from("hello");

    // s's value moves into the function and so is no longer valid here
    takes_ownership(s);

    //s is inaccessible now
    //println!("s {s}");

    // x comes into scope
    let x = 5;

    // x would move into the function,
    // but i32 is Copy, so it's okay to still
    // use x afterward
    makes_copy(x);

    // Here, x goes out of scope, then s. But because s's value was moved, nothing
    // special happens.

    // gives_ownership moves its return value into s1
    let s1 = gives_ownership();
    println!("Give ownership: {}", s1);

    // s2 comes into scope
    let s2 = String::from("hello");
    // s2 is moved into takes_and_gives_back, which also moves its return value into s3
    let s3 = takes_and_gives_back(s2);
    println!("Takes and gives back: {}", s3);
    // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
    // happens. s1 goes out of scope and is dropped.

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("String owenership: {}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("Integer ownership: {}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

// gives_ownership will move its return value into the function that calls it
fn gives_ownership() -> String {
    // some_string comes into scope
    let some_string = String::from("yours");

    // some_string is returned and
    // moves out to the calling
    // function
    some_string
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope

    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}