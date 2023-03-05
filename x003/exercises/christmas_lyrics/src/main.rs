use std::collections::HashMap;

fn main() {
    let things_my_true_love_gave = HashMap::from([
        ("0", "A partridge in a pear tree"),
        ("1", "Two turtle doves, and"),
        ("2", "Three french hens,"),
        ("3", "Four calling birds,"),
        ("4", "Five golden rings,"),
        ("5", "Six geese a-laying,"),
        ("6", "Seven swans a-swimming,"),
        ("7", "Eight maids a-milking,"),
        ("8", "Nine ladies dancing,"),
        ("9", "Ten lords a-leaping,"),
        ("10", "Eleven pipers piping,"),
        ("11", "Twelve drummers drumming,"),
    ]);

    let days: [&str; 12] = [
        "First", "Second", "Third", "Fourth", "Fifth", "Sixth", "Seventh", "Eighth", "Ninth",
        "Tenth", "Eleventh", "Twelveth",
    ];

    for x in 0..12 {
        println!(
            "On the {} day of Christmas\nMy true love gave to me",
            days[x]
        );

        for index in (0..=x).rev() {
            let key = index.to_string();
            let key: &str = &*key;

            println!("{}", things_my_true_love_gave[key]);
        }
        println!("\r\n")
    }
}
