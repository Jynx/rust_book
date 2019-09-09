use std::io;

fn main() {
    let mut number_to_convert = String::new();
    let mut temperature_unit: std::string::String = String::new();

    query_user_for_temperature_unit(&mut temperature_unit);
    println!("user value is {}", temperature_unit);

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

    // if (unit_is_celcius) {
    // let calculated_value: f32 = number_to_convert * 1.8 + 32;
    // }
}

fn query_user_for_temperature_unit(temperature_unit: &mut std::string::String) {
    loop {
        println!("Please enter the temperature unit: C or F");

        io::stdin()
            .read_line(temperature_unit)
            .expect("failed to read input");

        let temperature_unit_formatted = temperature_unit.trim().to_uppercase();
        let is_valid_unit = is_valid_temperature_unit(temperature_unit_formatted);

        if !is_valid_unit {
            println!("you did not enter a valid temperature unit");
            temperature_unit.clear();
            continue;
        }
        break;
    }
}

fn is_valid_temperature_unit(user_input: std::string::String) -> bool {
    if user_input.len() > 1 {
        return false;
    }
    return user_input.starts_with('C') || user_input.starts_with('F');
}
