use core::num;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::hash::Hash;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::num::NonZeroIsize;
use std::path::absolute;
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

    let mut reports: Vec<Vec<isize>> = Vec::new();

    for line in reader.lines() {
        //Convert to array of numbers
        let line_unwrapped: String = line.unwrap();
        let line_split: Vec<isize> = line_unwrapped
            .split(' ')
            .map(|num| num.parse::<isize>().unwrap())
            .collect();

        reports.push(line_split);
    }

    let num_safe_reports = scan_list(&mut reports);

    println!("Number of safe reports is: {}", num_safe_reports);
    println!("Finished question 2");
}

fn scan_list(reports: &mut Vec<Vec<isize>>) -> isize {
    let mut num_safe_reports = 0;

    for report in reports {
        num_safe_reports += scan_report(&report);
    }

    num_safe_reports
}

fn create_differences_list(report: &Vec<isize>) -> Vec<isize> {
    let mut differences = Vec::new();
    let report_length = report.len();

    let mut index = 0;

    while index < report_length - 1 {
        let diff = *report.get(index).unwrap() - *report.get(index + 1).unwrap();

        differences.push(diff);
        index += 1;
    }

    differences
}

fn scan_report(report: &Vec<isize>) -> isize {
    let report_length = report.len();
    let differences = create_differences_list(report);

    let increasing_or_decreasing = is_increasing_or_decreasing(&differences);
    let within_range = is_within_range(&differences);

    //If both of these are true, then the report is safe
    if within_range && increasing_or_decreasing {
        return 1;
    }

    let mut index = 0;

    //If it fails for either reason, let's just try it again by removing every level until
    //we run out of levels or find one that works. Note this is very brute force as we're
    //re-copying the original every iteration of the loop

    while index < report_length {
        let mut report_copy = report.clone();
        println!("Report_copy is {:?}", report_copy);
        report_copy.remove(index);
        println!("Report_copy after removing one is {:?}", report_copy);

        let new_differences = create_differences_list(&report_copy);
        println!("New differences is :{:?}", new_differences);
        if is_within_range(&new_differences) && is_increasing_or_decreasing(&new_differences) {
            return 1;
        }

        index += 1;
    }

    //If it gets here, then it means the report isn't safe so return 0
    return 0;
}

fn is_within_range(differences: &Vec<isize>) -> bool {
    if differences.len() == 0 {
        return false;
    } else if differences.len() == 1 {
        let abs_difference = differences.get(0).unwrap().abs();
        if abs_difference > 0 && abs_difference <= 3 {
            return true;
        }
        return false;
    }

    for difference in differences {
        let abs_difference = difference.abs();
        if !(abs_difference > 0 && abs_difference <= 3) {
            return false;
        }
    }
    return true;
}

fn is_increasing_or_decreasing(differences: &Vec<isize>) -> bool {
    let mut index = 0;
    let differences_length = differences.len();

    if differences_length == 0 {
        return false;
    } else if differences_length == 1 {
        return true;
    }

    while index < differences_length - 1 {
        let current_diff = *differences.get(index).unwrap();
        let next_diff = *differences.get(index + 1).unwrap();

        if current_diff.signum() != next_diff.signum() {
            return false;
        }
        index += 1;
    }

    println!("Differences are : {:?}", differences);
    return true;
}

#[cfg(test)]
mod tests {

    use crate::scan_report;

    #[test]
    fn scan_report_increasing() {
        let increasing_report = vec![1, 2, 3, 4, 5, 6];

        assert_eq!(1, scan_report(&increasing_report));
    }

    #[test]
    fn scan_report_one_bad_level_increasing() {
        let increasing_report = vec![1, 2, 3, 10, 6];

        assert_eq!(1, scan_report(&increasing_report,));
    }

    #[test]
    fn scan_report_decreasing() {
        let decreasing_report = vec![6, 5, 4, 3, 2, 1];

        assert_eq!(1, scan_report(&decreasing_report));
    }

    #[test]
    fn scan_report_increasing_and_decreasing() {
        let increasing_and_decreasing = vec![1, 2, 3, 4, 3, 2, 1];

        assert_eq!(0, scan_report(&increasing_and_decreasing));
    }

    #[test]
    fn scan_report_duplicate_increasing() {
        let report_with_two_bad_levels = vec![1, 2, 3, 7, 7, 4];

        assert_eq!(0, scan_report(&report_with_two_bad_levels))
    }

    #[test]
    fn scan_report_duplicate_decreasing() {
        let report_with_two_bad_levels = vec![9, 9, 9, 8, 7, 6, 5, 4, 3, 2, 1];

        assert_eq!(0, scan_report(&report_with_two_bad_levels));
    }

    #[test]
    fn scan_report_outside_range_increasing() {
        let report_with_outside_range = vec![1, 10, 20];

        assert_eq!(0, scan_report(&report_with_outside_range));
    }

    #[test]
    fn scan_report_outside_range_increasing_within_one_level() {
        let report_with_outside_range = vec![1, 10, 12];

        assert_eq!(1, scan_report(&report_with_outside_range));
    }
}
