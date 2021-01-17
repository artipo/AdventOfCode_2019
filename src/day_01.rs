use std::fs;

pub fn solve(input: &str) -> String {
    let result_a = puzzle_a(&input);
    let result_b = puzzle_b(&input);

    let result = format!("Solutions day 01: {}\n\t\t  {}", result_a, result_b);
    result
}

fn puzzle_a(input: &str) -> String {
    let input_text = fs::read_to_string(input).unwrap();
    let mut total_fuel_needed = 0;

    for line in input_text.lines() {
        let mass = match line.parse::<f32>(){
            Err(why) => panic!("couldn't parse line {}: {}", line, why),
            Ok(mass) => mass
        };

        let module_fuel = ((mass / 3.0).floor() - 2.0) as i32;
        total_fuel_needed += module_fuel;
    }

    let result = format!("{}", total_fuel_needed);
    result
}

fn puzzle_b(input: &str) -> String {
    let input_text = fs::read_to_string(input).unwrap();
    let mut total_fuel_needed = 0;

    for line in input_text.lines() {

        let mut fuel = match line.parse::<i32>(){
            Err(why) => panic!("couldn't parse line {}: {}", line, why),
            Ok(mass) => mass
        };

        while fuel > 0
        {
            fuel = ((fuel as f32 / 3.0).floor() - 2.0) as i32;

            if fuel > 0 {
                total_fuel_needed += fuel;
            }
        }
    }
    
    let result = format!("{}", total_fuel_needed);
    result
}