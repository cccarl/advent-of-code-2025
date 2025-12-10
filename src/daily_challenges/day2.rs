use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// https://adventofcode.com/2025/day/2
pub fn day2(mut input_reader: BufReader<File>) {
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

        buffer.clear();
    }
}
