use std::io;

fn main() {
    let mut temperature_unit: std::string::String = String::new();

    query_user_for_temperature_unit(&mut temperature_unit);
    let value_to_convert: i32 = query_user_for_value_to_convert();
    let converted_value = convert_user_input(temperature_unit, value_to_convert).to_string();
    println!("converted Value is {}", converted_value);
}

fn query_user_for_temperature_unit(temp_unit_input_buf: &mut std::string::String) {
    loop {
        println!("Please enter the temperature unit: C or F");

        io::stdin()
            .read_line(temp_unit_input_buf)
            .expect("failed to read input");

        let temperature_unit = temp_unit_input_buf.trim().to_uppercase();
        let is_valid_temp_unit = is_valid_temperature_unit(&temperature_unit);

        if !is_valid_temp_unit {
            println!("you did not enter a valid temperature unit");
            temp_unit_input_buf.clear();
            continue;
        }
        temp_unit_input_buf.clear();
        temp_unit_input_buf.push_str(&temperature_unit);
        break;
    }
}

fn query_user_for_value_to_convert() -> i32 {
    let mut input_buffer: std::string::String = String::new();
    loop {
        println!("Please enter the value to convert");

        io::stdin()
            .read_line(&mut input_buffer)
            .expect("failed to read input");

        match input_buffer.trim().parse() {
            Ok(num) => {
                return num;
            }
            Err(_) => {
                println!("You did not enter a valid number");
                input_buffer.clear();
                continue;
            }
        };
    }
}

fn is_valid_temperature_unit(user_input: &std::string::String) -> bool {
    return user_input.len() == 1 && user_input.starts_with('C') || user_input.starts_with('F');
}

fn convert_user_input(temp_unit: std::string::String, val: i32) -> f32 {
    return if temp_unit.starts_with('F') {
        val as f32 * 1.8 + 32.0
    } else {
        val as f32 * 1.8 - 32.0
    };
}
