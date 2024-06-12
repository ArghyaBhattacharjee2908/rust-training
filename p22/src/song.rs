pub fn twelvedaysofchristmas() {
    let line11 = "On the ";
    let line12 = " day of Christmas,";
    let line2 = "my true love gave to me";
    let line_last = "partridge in a pear tree";
    let p1 = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let p2 = [
        "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Eleven", "Twelve",
    ];
    let p3 = [
        "turtle doves",
        "French hens",
        "calling birds",
        "golden rings",
        "geese a-laying",
        "swans a-swimming",
        "maids a-milking",
        "ladies dancing",
        "lords a-leaping",
        "pipers piping",
        "drummers drumming",
    ];

    let mut x: String;
    let mut y = String::new();

    for i in 0..12 {
        if i == 0 {
            x = line11.to_owned() + p1[i] + line12 + "\n" + line2 + "\n" + "A " + line_last;
        } else {
            y.clear();
            for j in 0..i {
                y = p2[j].to_owned() + " " + p3[j] + ",\n" + &y;
            }
            x = line11.to_owned()
                + p1[i]
                + line12
                + "\n"
                + line2
                + "\n"
                + &y
                + "And a "
                + line_last;
        }
        if i == 11 {
            x += "!";
        } else {
            x += ".\n";
        }
        println!("{}", x);
    }
}
