use std::io;

fn main() {
    let mut temp_unit = String::new();

    query_for_temp_unit(&mut temp_unit);
    let val_to_convert = query_user_for_val();
    let converted_val = convert_temp_val(temp_unit, val_to_convert).to_string();
    println!("converted Value is {}", converted_val);
}

fn query_for_temp_unit(unit_input_buf: &mut String) {
    loop {
        println!("Please enter the temperature unit: C or F");

        io::stdin()
            .read_line(unit_input_buf)
            .expect("failed to read input");

        let temp_unit = unit_input_buf.trim().to_uppercase();
        let is_valid_temp_unit = is_valid_temp_unit(&temp_unit);

        if !is_valid_temp_unit {
            println!("you did not enter a valid temperature unit");
            unit_input_buf.clear();
            continue;
        }
        unit_input_buf.clear();
        unit_input_buf.push_str(&temp_unit);
        break;
    }
}

fn query_user_for_val() -> i32 {
    let mut input_buf = String::new();
    loop {
        println!("Please enter the value to convert");

        io::stdin()
            .read_line(&mut input_buf)
            .expect("failed to read input");

        match input_buf.trim().parse() {
            Ok(num) => {
                return num;
            }
            Err(_) => {
                println!("You did not enter a valid number");
                input_buf.clear();
                continue;
            }
        };
    }
}

fn is_valid_temp_unit(user_input: &String) -> bool {
    return user_input.len() == 1 && user_input.starts_with('C') || user_input.starts_with('F');
}

fn convert_temp_val(temp_unit: String, val: i32) -> f32 {
    return if temp_unit.starts_with('F') {
        val as f32 * 1.8 + 32.0
    } else {
        val as f32 * 1.8 - 32.0
    };
}
