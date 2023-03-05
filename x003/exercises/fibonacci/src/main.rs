use std::io;

fn main() {
    loop {
        println!("Enter the a number to retrieve nth value in the fibbonaci sequence: (Enter \"quit\" to exit)");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Cannot read input.");

        if input.trim() == "quit" {
            println!("Quitting...");
            break;
        }

        let input = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };

        let mut vector: Vec<usize> = Vec::new();
        vector.push(0);
        vector.push(1);
        let vector: Vec<usize> = vector;
        //if input <= 93 {
        //    let vect = fibbonaci(input, vector);
        //    let input = input as usize;
        //    println!("{}", vect[input])
        //} else {
        //    println!("Value too high")
        //}
        let vect = fibbonaci(input, vector);
        println!("{}", vect)
    }
}

fn fibbonaci(input: usize, mut vector: Vec<usize>) -> usize {
    for x in 2..=input {
        let x_1 = vector[x - 1];
        let x_2 = vector[x - 2];
        if x_1.checked_add(x_2) != None {
            let fib = x_1 + x_2;
            vector.push(fib);
        } else {
            println!("Value is too high! Max is: {}", x - 1);
            vector.push(0);
            return vector[x];
        }
    }
    return vector[input];
}
