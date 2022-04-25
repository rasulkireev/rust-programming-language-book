use std::io;

fn main() {
    // Start by getting a temperature type:
    //  Farenheit (f) or Celcius (c)
    let starting_type = loop {
        println!("What are you converting from? use 'c' or 'f':");
        let mut starting_type = String::new();

        io::stdin()
            .read_line(&mut starting_type)
            .expect("Failed to read line");

        let starting_type = starting_type.trim().to_string();

        if starting_type == "f" {
            break starting_type;
        } else if starting_type == "c" {
            break starting_type;
        }
    };

    // Get the actual vlue to convert
    println!("Enter value to convert:");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let starting_value: u32 = input.trim().parse().expect("Wanted a number");

    if starting_type == "f" {
        let final_value: u32 = (starting_value - 32) * 5 / 9;
        let final_type: &str = "c";
        println!(
            "{}{} is {}{}.",
            starting_value, starting_type, final_value, final_type
        )
    } else if starting_type == "c" {
        let final_value: u32 = (starting_value * 5 / 9) + 32;
        let final_type: &str = "f";
        println!(
            "{}{} is {}{}.",
            starting_value, starting_type, final_value, final_type
        )
    }
}
