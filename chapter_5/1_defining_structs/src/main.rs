// To define group of informations we can use structs.
struct User {
    active: bool,
    username: String,  // We must define the type as String instead of the &str slice type.
                       // To use &str it is necessary to apply a `lifetime`.
                       // More about this in chapter 10.
    email: String,
    sign_in_count: u64,
}

// This is a struct without named fields. It behaves just like any tuple, with
// the difference that it hast fixed type when declaring it with the type as the
// name of the struct:
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like structs are structs that don`t have any defined fields. Ex:
struct AlwaysEqual;  // This are usually usefull to define traits. More on
                     // traits on chapter 10.



fn main() {
    // To fill in the information for a struct just fill in the struct
    // properties: if the user type mutable we can change it values later on. It
    // is not possible to make only some fields as mutable. The entire structure
    // will be mutable.
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    show_user_schama(&user1); // pass the reference from user, so we can use it later.

    // Changing the name:
    user1.username = String::from("Jhon Carter");
    show_user_schama(&user1); // pass the reference from user, so we can use it later.

    let mut user2 = build_user(
        String::from("seconduser@example.com"),
        String::from("seconduser123"),
    );
    user2.username = String::from("Jhoana D'arc");
    show_user_schama(&user2);

    let mut user3 = build_user_with_shorthand(
        String::from("thirduser@example.com"),
        String::from("thirduser123"),
    );
    user3.username = String::from("Paola");
    show_user_schama(&user3);

    let user4 = build_other_fields_inplicitly(String::from("paolamail@email.com"), user3);
    show_user_schama(&user4);

    // Testing the strcut tuple Color and Point
    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);
    let point1 = Point(0, 2, 1);
    let point2 = Point(1, 2, 1);
    println!("Black: {}{}{}", black.0, black.1, black.2);
    // it is also possible to destruct the struct tuple fields like any other
    // tuple using the dot operator followed by the attributes index:
    println!("White: [{},{},{}]", white.0, white.1, white.2);
    println!("Point 1: [{}, {}, {}]", point1.0, point1.1, point1.2);
    println!("Point 2: [{}, {}, {}]", point2.0, point2.1, point2.2);

    let _subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user_with_shorthand(email: String, username: String) -> User {
    // If the parameters have the same name as the struct properties it is
    // possible to use the shorthand notation, which basically means we can pass
    // the parameter as the struct property.
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_other_fields_inplicitly(email: String, olduser: User) -> User {
    // when changing some few attributes it is possible to reuse the fields from
    // other struct.
    User {
        email,  // shorthand annotation to change the email
        ..olduser  // all remaining fields are the same.
                   // When copying the remaining attributes, the original
                   // variable makes a move, losing its reference for the String
                   // attribute username.
    }
}





fn show_user_schama(user: &User) {
    // It is possible to access the value from a struct by using the dot operator:
    println!("--------- User Information -------");
    println!("- Name: {}", user.username);
    println!("- email: {}", user.email);
    println!("- active: {}", user.active);
    println!("- sign in count: {}", user.sign_in_count);
    println!("----------------------------------");
}
