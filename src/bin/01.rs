use regex::Regex;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {

    let mut total_calibration: Option<i32> = Some(0);
    let re = Regex::new(r"(\d)").unwrap();
    // let re2 = Regex::new(r"(\d(?!.+\d))").unwrap(); // negative lookahead are not allowed in Rust regex

    for line in input.lines() {
        let first_digit_match = re.find(line);
        let reversed_line: String = line.chars().rev().collect();
        let last_digit_match = re.find(reversed_line.as_str());

        if let (Some(first_digit), Some(last_digit)) = (first_digit_match, last_digit_match)  {
            let calibration_str = format!("{}{}", first_digit.as_str(), last_digit.as_str());

            match calibration_str.parse::<i32>() {
                Ok(calibration) => {
                    if let Some(total) = total_calibration.as_mut() {
                        *total += calibration;
                    }
                }
                Err(e) => {
                    println!("Failed to parse calibration in i32: {}", e);
                }
            }
        } else {
            println!("Failed to find first and last digit in line: {}", line);
        }
    }
    println!("Total calibration: {:?}", total_calibration);
    total_calibration
}

pub fn part_two(_input: &str) -> Option<i32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        let example_answer: Option<i32> = Some(142);
        assert_eq!(result, example_answer);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, None);
    }
}
