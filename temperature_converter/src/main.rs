use std::io;

fn main() {
    loop {
        println!("Please select conversion type");
        println!("1. Fahrenheit to Celcius");
        println!("2. Celcius to Fahrenheit");

        let mut conversion_type = String::new();
        io::stdin()
            .read_line(&mut conversion_type)
            .expect("Failed to read line!");

        let conversion_type = match conversion_type.trim() {
            "1" => 1,
            "2" => 2,
            _ => {
                println!("Please input 1 or 2 !");
                continue;
            }
        };

        println!("Please input temperature");
        let mut temperature = String::new();
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: f64 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input valid temperature");
                continue;
            }
        };

        let converted_temperature = if conversion_type == 1 {
            (temperature - 32.0) / 1.8
        } else {
            temperature * 1.8 + 32.0
        };

        println!(
            "Converted temperature is {:.1} {}",
            converted_temperature,
            {
                if conversion_type == 1 {
                    "C"
                } else {
                    "F"
                }
            }
        );
    }
}
