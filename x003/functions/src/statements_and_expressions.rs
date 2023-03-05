fn main() {
    let x = (let y = 6);  // let y = 6 does is a statement in itself so it will not return a value
                          // to bind to x
    
    // function calls, calling macros, new scope blocks and math operations are all expressions
    // becuase they return something.
    
    let = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
    
    {
        let x = 3;
        x + 1
    }; // this block is an expressions that returns 4
// expressions don't end with a semicolon
}
