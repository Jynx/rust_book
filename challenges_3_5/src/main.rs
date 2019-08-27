use std::io;

fn main() {
    let mut number_to_convert = String::new();

    loop {
        let mut temperature_unit = String::new();
        println!("Please enter the temperature unit: C or F");

        io::stdin()
            .read_line(&mut temperature_unit)
            .expect("failed to read input");

        let temperature_unit = temperature_unit.trim().to_uppercase();

        if (temperature_unit.len() > 2) {
            println!("you did not enter a valid temperature unit");
        } 

        let unit_is_celcius = temperature_unit.starts_with('C');
        let unit_is_farenheit = temperature_unit.starts_with('F');

        if !unit_is_celcius || !unit_is_farenheit {
            println!("you did not enter a valid temperature unit");
            continue;
        }
    }

    println!("Please enter the value to convert");

    io::stdin()
        .read_line(&mut number_to_convert)
        .expect("failed to read input");

    let number_to_convert: i32 = match number_to_convert.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            panic!("You did not enter a valid number");
        }
    };

    if (unit_is_celcius) {
        let calculated_value: f32 = number_to_convert * 1.8 + 32;
    }
}

// fn get_valid_user_input() -> i32 {}
