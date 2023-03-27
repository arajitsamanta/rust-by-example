#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);
#[derive(Debug)]
struct AlwaysEqual;


pub fn structure_example() {
    println!("\n====== Structures ======");

    println!("******Simple structure");
    simple_struct();

    println!("******Mutable structure");
    mutable_struct();

    println!("******Structure creation with field init short hand");
    let user2 = struct_create_with_field_init_short_hand(
        "Keyser Soze".to_string(),
        "keyser.soze@noemail.com".to_string(),
    );
    print_user(&user2);

    println!("******Update structure");
    let user3 = User {
        email: String::from("updated-email@example.com"),
        ..user2
    };
    print_user(&user3);

    println!("******Tuple structure");
    tuple_struct();

    println!("******Unit like structure");
    unit_like_structs();
}

fn simple_struct() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    print_user(&user1)
}

fn mutable_struct() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    print_user(&user1);
}

fn struct_create_with_field_init_short_hand(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn print_user(user: &User) {
    println!(
        "User '{}' is active '{}' with email '{}' and sign in count '{}'",
        user.username, user.active, user.email, user.sign_in_count
    );
}

fn tuple_struct() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{:?} {:?}", black, origin);
}

fn unit_like_structs(){
    let subject = AlwaysEqual;
    println!("{:?}",subject)
}