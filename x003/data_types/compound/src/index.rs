use std::io;

fn main() {
    let _a = [1, 2, 3, 4, 5];
    let months: [&str; 12] = [
        "January",
        "Feburary",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let _b: [i8; 5] = [3; 5];

    let _first = a[0];
    let _second = a[1];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = months[index];

    println!("The value of the element at index {index} is: {element}");
}
