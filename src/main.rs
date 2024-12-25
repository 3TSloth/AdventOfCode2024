use core::num;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::hash::Hash;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::thread::current;

fn main() {
    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());

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

    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        //Convert to array of numbers
        let line_unwrapped = line.unwrap();
        let line_split: Vec<i32> = line_unwrapped
            .split(' ')
            .map(|num| num.parse::<i32>().unwrap())
            .collect();

        reports.push(line_split);
    }

    let num_safe_reports = scan_list(&mut reports);

    println!("Number of safe reports is: {}", num_safe_reports);
    println!("Finished question 2");
}

fn scan_list(reports: &mut Vec<Vec<i32>>) -> usize {
    //let hashset_list: Vec<HashSet<i32>> = Vec::new();
    let mut num_safe_reports = 0;

    for report in reports {
        scan_report(report, &mut num_safe_reports, false);
    }

    num_safe_reports
}

pub fn scan_report(report: &Vec<i32>, num_safe: &mut usize, is_recursing: bool) {
    println!("Report is: {:?}", report);
    let mut differences: Vec<i32> = Vec::new();
    let report_length = report.len();
    let mut indices_of_bad_levels = Vec::new();

    if report_length > 2 {
        let mut previous_num = report.get(0).unwrap();
        let mut next_num = report.get(1).unwrap();

        let mut index = 0;

        while index < report_length - 1 {
            let difference = previous_num - next_num;
            let abs_difference = difference.abs();
            let is_within_limits = abs_difference >= 1 && abs_difference <= 3;

            println!("The difference is: {}", difference);

            //If it's not within limits, add it to the list of levels to be removed
            if !is_within_limits && !is_recursing {
                indices_of_bad_levels.push(index);
            }

            differences.push(difference);
            index += 1;

            previous_num = report.get(index).unwrap();

            if index + 1 < report_length - 1 {
                next_num = report.get(index + 1).unwrap();
            }
        }

        let mut num_positive = 0;
        let mut num_negative = 0;

        for difference in differences {
            if difference >= 1 {
                num_positive += 1;
            } else {
                num_negative += 1;
            }
        }

        println!("Num positive is: {}", num_positive);
        println!("Num negative is : {}", num_negative);
        println!("Report length is: {}", report_length);

        let is_safe = num_positive == report_length - 1 || num_negative == report_length - 1;

        println!("Num safe is: {}", num_safe);
        if is_safe {
            *num_safe += 1;
        }

        if is_recursing {
            println!("Num_safe within recursing is: {}", num_safe);
            return;
        }

        println!("Gets here");

        println!("Indices of bad levels is {:?}", indices_of_bad_levels);
        //If it's not safe and there's 'bad levels' to remove (which should be at least one), attempt again
        if !is_safe && indices_of_bad_levels.len() >= 1 {
            //Remove each bad level from the original and see if it's safe
            for bad_level_index in indices_of_bad_levels {
                let mut report_copy = report.clone();
                report_copy.remove(bad_level_index);

                println!("New report to be recursed on: {:?}", report_copy);

                println!("Num safe before rcursing {}", num_safe);
                scan_report(&report_copy, num_safe, true);
                println!("Num safe after recursing {}", num_safe);
            }
        }
    } else if report_length == 2 {
        println!("Gets in report_length 2 if");
        let first_num = report.get(0).unwrap();
        let second_num = report.get(1).unwrap();
        let is_safe = first_num < second_num || first_num > second_num;

        println!("Within recursion, is safe is {}", is_safe);

        if is_safe {
            *num_safe += 1;
            println!("Gets inside is safe report length 2");
            println!("Num safe should now be: {}", num_safe);
        }
    } else if report_length == 1 {
        *num_safe += 1;
    }
}

#[cfg(test)]
mod tests {

    use crate::scan_report;

    #[test]
    fn scan_report_increasing() {
        let increasing_report = vec![1, 2, 3, 4, 5, 6];
        let num_safe_reports = 0;

        assert_eq!(
            1,
            scan_report(&increasing_report, &mut num_safe_reports, false)
        );
    }

    #[test]
    fn scan_report_one_bad_level_increasing() {
        let increasing_report = vec![1, 2, 3, 10, 6];
        let max_tolerance = 1;

        assert_eq!(true, scan_report(&increasing_report,));
    }

    #[test]
    fn scan_report_decreasing() {
        let decreasing_report = vec![6, 5, 4, 3, 2, 1];
        let max_tolerance = 0;

        assert_eq!(true, scan_report(&decreasing_report));
    }

    #[test]
    fn scan_report_increasing_and_decreasing() {
        let increasing_and_decreasing = vec![1, 2, 3, 4, 3, 2, 1];
        let max_tolerance = 0;

        assert_eq!(false, scan_report(&increasing_and_decreasing));
    }

    #[test]
    fn scan_report_duplicate_increasing() {
        let report_with_two_bad_levels = vec![1, 2, 3, 7, 7, 4];
        let max_tolerance = 1;
        assert_eq!(false, scan_report(&report_with_two_bad_levels))
    }

    #[test]
    fn scan_report_duplicate_decreasing() {
        let report_with_two_bad_levels = vec![9, 9, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let max_tolerance = 1;
        assert_eq!(false, scan_report(&report_with_two_bad_levels));
    }

    #[test]
    fn scan_report_outside_range_increasing() {
        let report_with_outside_range = vec![1, 10, 12];
        let max_tolerance = 0;

        assert_eq!(false, scan_report(&report_with_outside_range));
    }

    #[test]
    fn scan_report_outside_range_increasing_within_one_level() {
        let report_with_outside_range = vec![1, 10, 12];
        let max_tolerance = 1;

        assert_eq!(true, scan_report(&report_with_outside_range));
    }
}
