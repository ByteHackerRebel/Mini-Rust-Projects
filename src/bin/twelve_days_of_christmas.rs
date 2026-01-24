fn main() {
    let gifts = [
        "A Partridge in a Pear Tree",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Golden Rings",
        "Six Geese a-Laying",
        "Seven Swans a-Swimming",
        "Eight Maids a-Milking",
        "Nine Ladies Dancing",
        "Ten Lords a-Leaping",
        "Eleven Pipers Piping",
        "Twelve Drummers Drumming",
    ];

    for outer in 1..12 {
        let suffixe = if outer == 1 {
            "st"
        } else if outer == 2 {
            "nd"
        } else {
            "th"
        };

        println!(
            "\nOn the {}{} day of Christmas, my true love sent to me",
            outer, suffixe
        );
        for inner in (0..outer).rev() {
            if outer > 1 && inner == 0 {
                println!("And, {}", gifts[inner]);
            } else {
                println!("{}", gifts[inner]);
            }
        }
    }
}
