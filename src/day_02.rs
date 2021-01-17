use std::fs;

pub fn solve(input: &str) -> String {
    let result_a = puzzle_a(&input);
    let result_b = puzzle_b(&input);

    let result = format!("Solutions day 02: {}\n\t\t  {}", result_a, result_b);
    result
}

fn puzzle_a(input: &str) -> String {
    let input_text = fs::read_to_string(input).unwrap();
    let str_vals = input_text.trim().split(',');
    let mut positions: Vec<i32> = vec![];

    for str_val in str_vals {
        let int_val = match str_val.parse::<i32>() {
            Err(why) => panic!("ERROR: couldn't parse value {}: {}", str_val, why),
            Ok(int_val) => int_val
        };

        positions.push(int_val);
    }

    // restore "1202 program alarm" state
    positions[1] = 12;
    positions[2] = 2;

    let mut counter: usize = 0;
    const NEXT_SUB_PROGRAM: usize = 4;

    loop {
        let opcode = positions[counter];
        
        match opcode {
            1 => {
                let index1 = positions[counter + 1] as usize;
                let val1 = positions[index1];

                let index2 = positions[counter + 2] as usize;
                let val2 = positions[index2];

                let index3 = positions[counter + 3] as usize;
                let val3 = val1 + val2;

                positions[index3] = val3;
            },
            2 => {
                let index1 = positions[counter + 1] as usize;
                let val1 = positions[index1];

                let index2 = positions[counter + 2] as usize;
                let val2 = positions[index2];

                let index3 = positions[counter + 3] as usize;
                let val3 = val1 * val2;

                positions[index3] = val3;
            },
            99 => break,
            _ => panic!("ERROR: opcode unknown {}", opcode)
        };

        counter += NEXT_SUB_PROGRAM;
    }

    let result = format!("{}", positions[0]);
    result
}

fn puzzle_b(input: &str) -> String {
    let input_text = fs::read_to_string(input).unwrap();

    let output_target = 19690720;
    let mut noun = 0;
    let mut verb = 0;
    let mut is_successful = false;

    loop {
        let str_vals = input_text.trim().split(',');
        let mut positions: Vec<i32> = vec![];

        // reset values
        for str_val in str_vals {
            let int_val = match str_val.parse::<i32>() {
                Err(why) => panic!("ERROR: couldn't parse value {}: {}", str_val, why),
                Ok(int_val) => int_val
            };

            positions.push(int_val);
        }

        // restore "1202 program alarm" state
        positions[1] = noun;
        positions[2] = verb;

        let mut counter: usize = 0;
        const NEXT_SUB_PROGRAM: usize = 4;

        loop {
            let opcode = positions[counter];

            match opcode {
                1 => {
                    let index1 = positions[counter + 1] as usize;
                    let val1 = positions[index1];

                    let index2 = positions[counter + 2] as usize;
                    let val2 = positions[index2];

                    let index3 = positions[counter + 3] as usize;
                    let val3 = val1 + val2;

                    positions[index3] = val3;
                },
                2 => {
                    let index1 = positions[counter + 1] as usize;
                    let val1 = positions[index1];

                    let index2 = positions[counter + 2] as usize;
                    let val2 = positions[index2];

                    let index3 = positions[counter + 3] as usize;
                    let val3 = val1 * val2;

                    positions[index3] = val3;
                },
                99 => break,
                _ => panic!("ERROR: opcode unknown {}", opcode)
            };

            counter += NEXT_SUB_PROGRAM;
        }

        if positions[0] == output_target {
            is_successful = true;
            break;
        }

        noun += 1;
        if noun > 99 {
            noun = 0;
            verb += 1;

            if verb > 99 {
                break;
            }
        }
    }

    if is_successful {
        let result = format!("{:02}{:02}", noun, verb);
        result
    } else {
        panic!("Solution was not found");
    }
}