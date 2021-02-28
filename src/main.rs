fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    let lyrics = ["A partridge in a pear tree", "Two turtle doves, and", "Three french hens", "Four calling birds", "Five golden rings", "Six geese a-laying", "Seven swans a-swimming", "Eight maids a-milking", "Nine ladies dancing", "Ten lords a-leaping", "Eleven pipers piping", "Twelve drummers drumming"];

    for n in 0..12 {
        println!("On the {} day of Christmas, my true love sent to me", days[n]);

        if n == 0 {
            println!("{}\n", lyrics[n]);
        } else {
            let x = n;
            let mut words = String::new();
            for index in (0..x + 1).rev() {
                words.push_str(lyrics[index]);
                words.push_str("\n");
            }
            println!("{}", words)
        };
    }
}
