fn main() {
    // addition
    let sum = 5 + 10;
    println!("sum: {}", sum);
    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference: {}", difference);
    // multiplication
    let product = 4 * 30;
    println!("product: {}", product);
    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("quotient: {}", quotient);
    println!("truncated: {}", truncated);
    // remainder
    let remainder = 43 % 5;
    println!("remainder: {}", remainder);
}


// Usage of structs in Rust.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        email,
        username,
        sign_in_count: 1,
    }
}

// Creating instances from Other Instances with Struct Update Syntax
fn main() {
    // --snip--

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}
// Using struct update syntax, we can achieve the same effect with ess code. The
// syntax '..' specifies that remaining fields not explicitly set should have
// the same value as the fields in the given instance.
fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}