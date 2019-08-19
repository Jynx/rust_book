use std::io;

fn main() {
    let mut temperature_unit = String::new();
    let mut number_to_convert = String::new();

    loop {
        println!("Please enter the temperature unit: C or F");

        io::stdin()
            .read_line(&mut temperature_unit)
            .expect("failed to read input");
        
        let temperature_unit = temperature_unit.to_uppercase();

        //compare inptu strings
        match temperature_unit.cmp(&)

        if (temperature_unit != 'C' || temperature_unit != 'F') {
            printf!("You did not enter a valid temperature Unit ");
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
}

fn get_valid_user_input() -> i32 {}
