use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// https://adventofcode.com/2025/day/2
pub fn day2(mut input_reader: BufReader<File>) {
    let mut final_sum: u64 = 0;
    let mut final_sum_pt2: u64 = 0;
    let mut buffer = Vec::<u8>::new();
    loop {
        let bytes_read = input_reader
            .read_until(0x2C, &mut buffer)
            .expect("Error when reading bytes from file");
        if bytes_read == 0 {
            break;
        }

        let range_str = if let Ok(segment) = String::from_utf8(buffer.clone()) {
            let stripped_line = segment.strip_suffix([',', '\n']);
            match stripped_line {
                Some(str) => str.to_string(),
                None => segment,
            }
        } else {
            eprintln!("Invalid UTF-8 sequence found");
            break;
        };

        let mut range_split = range_str.split("-");
        let range_start: u64 = range_split
            .next()
            .expect(&format!("Could not get range start in line: {}", range_str))
            .to_string()
            .parse()
            .expect(&format!(
                "Could not parse to int range start in line: {}",
                range_str
            ));
        let range_end: u64 = range_split
            .next()
            .expect(&format!("Could not get range end in line: {}", range_str))
            .to_string()
            .parse()
            .expect(&format!(
                "Could not parse to int range end in line: {}",
                range_str
            ));

        println!("{}-{}", range_start, range_end);

        for i in range_start..=range_end {
            let num_str = i.to_string();
            let num_len = num_str.len();

            // part 1 silly code detection
            if num_len % 2 == 0 {
                let first_half = num_str
                    .get(0..num_len / 2)
                    .expect("Could not slice first half of string");
                let sec_half = num_str
                    .get(num_len / 2..num_len)
                    .expect("Could not slice second half of string");
                if first_half == sec_half {
                    //println!("Detected repeated halfs! {} {}", first_half, sec_half);
                    final_sum += i;
                }
            }

            // part 2 silly code detection
            let mut line_made_of_repeats = false;
            for slice_size in 1..num_len {
                let current_slice = num_str
                    .get(0..slice_size)
                    .expect("Could not get slice for repeat check");

                let mut this_slice_repeats = true;
                // does not perfectly divide the entire digit
                if num_len % slice_size != 0 {
                    continue;
                }

                for str_index_to_check in 1..num_len {
                    let index_start = slice_size * str_index_to_check;

                    let secondary_slice = num_str.get(index_start..(index_start + slice_size));
                    if let Some(string_to_compare) = secondary_slice {
                        /* println!(
                            "[{}] Comparing: {} {}",
                            num_str, current_slice, string_to_compare
                        ); */
                        if string_to_compare != current_slice {
                            this_slice_repeats = false;
                            break;
                        }
                    }
                }
                if this_slice_repeats {
                    line_made_of_repeats = true;
                    break;
                }
            }

            if line_made_of_repeats {
                final_sum_pt2 += i;
                println!("This line is made only of repeats: {}", i);
            }
        }

        buffer.clear();
    }

    println!("[Part 1] Final sum of invalid IDs: {}", final_sum);
    println!("[Part 2] Final sum of invalid IDs: {}", final_sum_pt2);
}
