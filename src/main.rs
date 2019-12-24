fn main() {
    let days = ["first","second","third","fourth","fifth","sixth","seventh","eighth","nineth","tenth","eleventh","twelfeth"];
    let cosas = ["And a partridge in a pear tree",
                    "Two turtle doves",
                    "Three French hens",
                    "Four calling birds",
                    "Five gold rings, badam-pam-pam",
                    "Six geese a laying",
                    "Seven swans a swimming",
                    "Eight maids a milking",
                    "Nine ladies dancing",
                    "Ten lords a leaping",
                    "Eleven pipers piping",
                    "12 drummers drumming"];

    for (i, item) in days.iter().enumerate() {
        println!("On the {} day of Christmas\nMy true love gave to me",item);

        let mut n = 0;
        while n <= i {
            if i == 0 {
                println!("{}","A partridge in a pear tree");
                n += 1;
                continue;
            }
            println!("{}",cosas[i-n]);
            n += 1;
        }
        println!("{}","");
    }
}
