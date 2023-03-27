pub fn slices_example() {
    println!("\n====== Slices ======");

    let mut s = String::from("hello world");
    let word = first_word_without_slice(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
    println!("Length of word {}", word);

    //Slice example
    println!("******Slice string");
    string_slices();

    //if we have an immutable reference to something, we cannot
    //also take a mutable reference. Because clear needs to
    //truncate the String, it needs to get a mutable reference.
    //The println! after the call to clear uses the reference in
    //word, so the immutable reference must still be active at
    // that point. Rust disallows the mutable reference in clear
    //and the immutable reference in word from existing at the same time
    let mut s1 = String::from("hello world");
    let _word1 = first_word(&s1);
    s1.clear();
    //println!("the first word is: {}", word1);

    println!("******String slices as parameters");
    string_slices_as_parameter();

    println!("******Array slice");
    array_slice();
}

fn first_word_without_slice(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn string_slices() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("Slicing '{}' to {} - {}", s, hello, world);

    //With Rust’s .. range syntax, if you want to start at index 0, you can drop the
    //value before the two periods. In other words, these are equal:
    let _slice = &s[0..2];
    let slice = &s[..2];
    println!("Slice with omiting start index {} ", slice);

    //By the same token, if your slice includes the last byte of the String,
    //you can drop the trailing number. That means these are equal:
    let len = s.len();
    let _slice = &s[3..len];
    let slice = &s[3..];
    println!("Slice with omiting end index {} ", slice);

    //You can also drop both values to take a slice of the entire string.
    //So these are equal:
    let _slice = &s[0..len];
    let slice = &s[..];
    println!("Slice with omit of start end index {} ", slice);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn string_slices_as_parameter() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    println!("First word: {}", word);
    let word = first_word(&my_string[..]);
    println!("First word: {}", word);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);
    println!("First word: {}", word);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    println!("First word: {}", word);
    let word = first_word(&my_string_literal[..]);
    println!("First word: {}", word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("First word: {}", word);
}

fn array_slice() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3], "Should be equal");
    println!("Array slice {:?}", slice);
}
