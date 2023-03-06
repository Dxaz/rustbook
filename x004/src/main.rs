fn main() {
    let mut s: String = String::from("Hello");

    println!("This is me before the function: {s}");

    some_func(&mut s);
    println!("This is me after the function: {}", s);
}

fn some_func(param: &mut String) {
    param.push_str(", world");
    println!("this is me during the function: {param}");
}
