use core::num;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    println!("Starting program.");
    // let mut reader_handle = take_stdin();
    question2();

    println!("Finished program.");
}

fn take_stdin() -> BufReader<File> {
    let mut filename = String::new();
    io::stdin().read_line(&mut filename).unwrap();

    let file: File = File::open(filename.trim()).unwrap();
    let reader = BufReader::new(file);

    reader
}

fn question1() -> io::Result<()> {
    println!("Enter the filename for question1's input:");
    let reader = take_stdin();

    let mut left_side_ints: Vec<i32> = Vec::new();
    let mut right_side_ints: Vec<i32> = Vec::new();

    //iterate over all the lines, split them, and convert each into an int
    //and add into its corresponding array

    for line in reader.lines() {
        println!("Gets in loop");
        let line_unwrapped = line.unwrap();
        let line_split: Vec<&str> = line_unwrapped.split(' ').collect();
        let left_number = line_split[0];
        let right_number = line_split[3];
        let left_number: i32 = left_number.parse().unwrap();
        let right_number: i32 = right_number.parse().unwrap();

        left_side_ints.push(left_number);
        right_side_ints.push(right_number);
    }

    let mut distances: Vec<i32> = Vec::new();
    left_side_ints.sort();
    right_side_ints.sort();

    //Lengths should be the same
    if left_side_ints.len() == right_side_ints.len() {
        let num_lines = left_side_ints.len();
        let mut index = 0;

        while index < num_lines {
            let distance = (left_side_ints[index] - right_side_ints[index]).abs();
            distances.push(distance);
            index += 1;
        }
    }

    let total_distance: i32 = distances.iter().sum();

    println!("Answer for question 1 is: {}", total_distance);

    Ok(())
}

fn question2() {
    let reader = take_stdin();

    let mut levels: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        //Convert to array of numbers
        let line_unwrapped = line.unwrap();
        let mut line_split: Vec<i32> = line_unwrapped
            .split(' ')
            .map(|num| num.parse::<i32>().unwrap())
            .collect();

        //Add an int at the end to act as a flag whether it is safe or unsafe
        line_split.push(-1);

        levels.push(line_split);

        println!("Line split is {:?}", levels.get(0));
    }

    //Now iterate over each one and check if it's always increasing

    for level in levels.as_mut_slice() {
        let mut index = 0;
        let mut count = 0;

        while index + 1 < level.len() - 1 {
            let previous_num = level.get(index).unwrap();
            let next_num = level.get(index + 1).unwrap();

            println!("Gets in while loop");

            let diff = (previous_num - next_num).abs();
            let is_within_limits = diff >= 1 && diff <= 3;
            let is_increasing = previous_num < next_num;
            if is_within_limits && is_increasing {
                count += 1;
            }
            index += 1;
        }

        if count == (level.len() - 2) {
            *level.last_mut().unwrap() = 1;
        } else {
            *level.last_mut().unwrap() = -1;
        }

        println!("Count is: {}", count);
        println!("Level is: {:?}", level);
    }

    //Now check if it's always decreasing

    for level in levels.as_mut_slice() {
        let mut index = 0;
        let mut count = 0;

        //Skip if we already know it's a valid increasing one from the previous scan
        if *level.last().unwrap() == 1 {
            continue;
        }

        while index + 1 < level.len() - 1 {
            let previous_num = level.get(index).unwrap();
            let next_num = level.get(index + 1).unwrap();

            println!("Gets in while loop");

            let diff = (previous_num - next_num).abs();
            let is_within_limits = diff >= 1 && diff <= 3;
            let is_decreasing = previous_num > next_num;
            if is_within_limits && is_decreasing {
                count += 1;
            }
            index += 1;
        }

        if count == (level.len() - 2) {
            *level.last_mut().unwrap() = 1;
        } else {
            *level.last_mut().unwrap() = -1;
        }

        println!("Count is: {}", count);
        println!("Level is: {:?}", level);
    }

    let mut num_safe_reports = 0;

    for level in levels {
        let is_safe_flag = *level.last().unwrap();
        if is_safe_flag == 1 {
            num_safe_reports += 1;
        }
    }
    println!("Number of safe reports is: {}", num_safe_reports);
    println!("Finished question 2");
}
