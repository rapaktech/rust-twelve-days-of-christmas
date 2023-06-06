fn main() {
    let things = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let positions = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let mut index = 0;

    loop {
        let mut counter = 0;

        println!(
            "On the {} day of Christmas my true love sent to me",
            positions[index]
        );

        if index == 0 {
            println!("{}.", things[counter]);
            println!("\n");
            index += 1;
            continue;
        }

        while counter < index {
            println!("{},", things[index - counter]);
            counter += 1;
        }

        if counter == index {
            let (a, b) = things[index - counter].split_at(1);

            let c = a.to_ascii_lowercase();

            println!("And {}{}.", c, b);
            println!("\n");
        }

        if index == things.len() - 1 {
            break;
        }

        index += 1;
    }
}
