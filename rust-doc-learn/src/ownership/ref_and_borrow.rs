pub fn reference_and_borrows() {
    println!("\n====== Refernces and Borrowing ======");

    let s = String::from("Hello");

    println!("******Pass by reference");
    let len = pass_by_reference(&s);
    println!("The length of '{}' is {}", s, len);

    println!("******Immutable reference");
    immutable_reference(&s);

    println!("******Mutable reference");
    let mut s1 = String::from("A mutable string by reference");
    mutable_reference(&mut s1);
    println!("String after change {}", s1);

    println!("******Multiple mutable reference to same data - not alllowed");
    multiple_mut_ref_to_same_data();

    println!("******Multiple mutable reference to same data - allow wth scope");
    multiple_mut_ref_to_same_data_with_scope();

    println!("******Mutable reference while we have an immutable - not allowed");
    multable_and_immutable_at_the_same_time();

    println!("******Mutable reference while we have an immutable - allow with scope");
    multable_and_immutable_at_the_same_time_with_scope();

    println!("******Dangling reference");
    //let reference_to_nothing = dangling_reference();

    println!("******No dangling reference");
    no_dangle();
}

fn pass_by_reference(s: &String) -> usize {
    s.len()
}

fn immutable_reference(_some_string: &String) {
    //some_string.push_str(", world");
    println!("Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.")
}

fn mutable_reference(mutable_string: &mut String) {
    mutable_string.push_str(", changed");
}

fn multiple_mut_ref_to_same_data() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    //The restriction preventing multiple mutable references to the same data at
    //the same time allows for mutation but in a very controlled fashion.
    //let r2 = &mut s;

    println!("{}", r1);
}

fn multiple_mut_ref_to_same_data_with_scope() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    {
        println!("{}", r1);
    }
    //The restriction preventing multiple mutable references to the same data at
    //the same time allows for mutation but in a very controlled fashion.
    let r2 = &mut s;

    println!("{}", r2);
}

fn multable_and_immutable_at_the_same_time() {
    let mut _s = String::from("hello");

    let r1 = &_s; // no problem
    let r2 = &_s; // no problem
                  //We also cannot have a mutable reference while we have an immutable one to the same value
                  //let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}", r1, r2);
}

fn multable_and_immutable_at_the_same_time_with_scope() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

//fn dangling_reference() -> &String {
//    let s = String::from("hello");
//    &s
//}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
