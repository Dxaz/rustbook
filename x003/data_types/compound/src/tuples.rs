fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, _z) = tup;
    let five_hundred: i32 = tup.0;
    let _six_point_four: f64 = tup.1;
    let _one: u8 = tup.2;

    println!("The value of y is: {y}");
    println!("The value of five_hundred is: {}", five_hundred);
}
