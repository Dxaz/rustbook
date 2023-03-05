// constant in rust
// THREE_HOURS_IN_SECONDS = 10800
const THREE_HOURS_IN_SECONDS: u16 = 60 * 60 * 3;

fn main() {
    // let x = 5; <-- Immutable by default

    // mutable variable
    let mut x: u8 = 5;
    println!("The value of x is: {x}");

    // adding a `: u8` after `x` would be considered type ascription becuase it is already assumed
    // to be u8 from above
    x = 6;
    println!("The value of x is: {x}");

    println!("The value of three hours in seconds is: {THREE_HOURS_IN_SECONDS}");
}
