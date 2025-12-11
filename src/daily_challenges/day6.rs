use std::{
    fs::File,
    io::{BufRead, BufReader, Seek},
};

#[derive(Debug)]
enum OpType {
    Add,
    Mul,
}

// https://adventofcode.com/2025/day/6
pub fn day6(input_reader: &mut BufReader<File>) {
    let mut operation_data: Vec<Vec<u64>> = vec![];
    let mut opetation_types: Vec<OpType> = vec![];
    for line_res in input_reader.lines() {
        let line = match line_res {
            Ok(str) => str,
            Err(e) => {
                eprintln!("Error when reading line: {}", e);
                continue;
            }
        };

        let mut current_line_values: Vec<u64> = vec![];
        let line_split = line.split(' ');
        for pot_number in line_split {
            if pot_number.is_empty() {
                continue;
            }

            let number_parsed = pot_number.parse::<u64>();
            match number_parsed {
                Ok(num) => {
                    current_line_values.push(num);
                }
                // probably a letter then
                Err(_) => {
                    if pot_number == "+" {
                        opetation_types.push(OpType::Add);
                    } else if pot_number == "*" {
                        opetation_types.push(OpType::Mul);
                    }
                }
            }
        }
        if !current_line_values.is_empty() {
            operation_data.push(current_line_values);
        }
    }

    let mut final_sum = 0;
    for i in 0..operation_data[0].len() {
        let mut curr_op_nums: Vec<u64> = vec![];
        let operation = &opetation_types[i];
        for j in 0..operation_data.len() {
            curr_op_nums.push(operation_data[j][i]);
        }
        let result = match operation {
            OpType::Add => curr_op_nums.iter().sum::<u64>(),
            OpType::Mul => curr_op_nums.iter().product::<u64>(),
        };

        final_sum += result;
    }

    println!("[Part 1] the final sum is: {}", final_sum);

    // part 2: actually parsing the numbers was completely different, oops!
    /* let rew_res = input_reader.rewind();
    if rew_res.is_err() {
        panic!("IDK BRO rewind just didn't work");
    }

    for line_res in input_reader.lines() {
        let line = match line_res {
            Ok(str) => str,
            Err(e) => {
                eprintln!("Error when reading line: {}", e);
                continue;
            }
        };
        
    } */

}
