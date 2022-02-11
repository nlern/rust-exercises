use std::io;

fn main() {
    loop {
        println!("Rust exercises on loops");
        println!("1. Temperature converter from Celsius to Fahrenheit and vice versa");
        println!("2. Find n-th Fibonacci number");
        println!("3. Print \"Twelve days of Christmas\" song lyrics & exit");
        println!("0. Exit");

        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");
        match option.trim() {
            "0" => {
                println!("Bye!");
                break;
            }
            "1" => celsius_fahrenheit_converter(),
            "2" => nth_fibonacci_number(),
            "3" => {
                twelve_days_of_christman();
                break;
            },
            _ => {
                println!("Invalid option selected. Please try again.");
                continue;
            }
        }
    }
}

fn twelve_days_of_christman() {
    let sent_items = [
        "A patridge in a pear tree",
        "Two turtle doves",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    let day_in_words = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let title = "The Twelve Days of Christmas lyrics";
    
    // print the title
    let dashes = "=".repeat(title.len());
    println!();
    println!("{}", dashes);
    println!("{}", title);
    println!("{}", dashes);
    println!();

    // print the verses
    for (i, day_word) in day_in_words.iter().enumerate() {
        println!("[Verse {}]", i + 1);
        println!(
            "On the {} day of Christmas, my true love sent to me",
            day_word
        );

        if i > 0 {
            for i1 in (2 .. i + 1).rev() {
                println!("{}", sent_items[i1]);
            }

            println!("{}, and", sent_items[1]);
        }

        println!("{}", sent_items[0]);

        println!();
    }
}

fn nth_fibonacci_number() {
    let mut f: [u32; 2] = [1; 2];
    loop {
        println!("Find nth Fibonacci number");
        let mut n = String::new();
        println!("Enter n (1 <= n <= 47).");

        io::stdin().read_line(&mut n).expect("Failed to read line");
        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please try again.");
                continue;
            }
        };

        if n <= 0 {
            println!("Invalid input. Please try again.");
            continue;
        }

        if n <= 2 {
            println!("F({}) = 1", n);
            break;
        }

        let mut counter = n;
        while counter > 2 {
            counter -= 1;
            let next = f[0] + f[1];
            f[0] = f[1];
            f[1] = next;
        }

        println!("F({}) = {}", n, f[1]);
        break;
    }
}

fn celsius_fahrenheit_converter() {
    loop {
        let mut unit = String::new();
        println!("Enter input unit(C/c=Celsius, F/f=Fahrenheit).");
        io::stdin()
            .read_line(&mut unit)
            .expect("Failed to read line");
        let mut t_input = String::new();
        println!("Enter temperature to convert.");
        io::stdin()
            .read_line(&mut t_input)
            .expect("Failed to read line");

        let t_input: f64 = match t_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid temperature. Please try again");
                continue;
            }
        };
        let unit = unit.trim().to_lowercase();
        let (t_out, unit_out) = match unit.as_str() {
            "c" => {
                // convert to fahrenheit
                let t_out = t_input * (9.0 / 5.0) + 32.0;
                (t_out, "Fahrenheit")
            }
            "f" => {
                // convert to celsius
                let t_out = (t_input - 32.0) * (5.0 / 9.0);
                (t_out, "Celsius")
            }
            _ => {
                println!("Invalid temperature unit! Please try again.");
                continue;
            }
        };

        println!("Converted temperature in {}: {:.2}", unit_out, t_out);

        break;
    }
}
