use std::io;

fn main() {
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
