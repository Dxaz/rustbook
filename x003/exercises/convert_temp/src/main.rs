use std::io;

fn main() {
    println!("Tempeture Converter");

    loop {
        println!("Enter a number to show the conversions to °F or °C: (Enter \"quit\" to Quit)");

        let mut num_to_convert = String::new();
        io::stdin()
            .read_line(&mut num_to_convert)
            .unwrap_or_default();

        if num_to_convert.trim() != "quit" {
            let num_to_convert: i32 = match num_to_convert.trim().parse() {
                Ok(num) => num,
                Err(_) => 0,
            };

            let celsius = f_to_c(num_to_convert);
            let fahrenheit = c_to_f(num_to_convert);

            println!("\r\n{num_to_convert}°C to fahrenheit equals {fahrenheit}°F");
            println!("{num_to_convert}°F to celsius equals {celsius}°C\r\n");
        } else {
            break;
        }
    }
}

fn f_to_c(fahren: i32) -> i32 {
    let cel: i32 = (fahren - 32) * 5 / 9;
    return cel;
}

fn c_to_f(cel: i32) -> i32 {
    let fahren: i32 = (cel * 9 / 5) + 32;
    return fahren;
}
