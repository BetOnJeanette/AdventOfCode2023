fn get_line_num(line: &str) -> u32 {
    let first_digit = (line.as_bytes()[line.find(char::is_numeric).unwrap()] as char).to_digit(10).unwrap();
    let last_digit = (line.as_bytes()[line.rfind(char::is_numeric).unwrap()] as char).to_digit(10).unwrap();
    return 10 * first_digit + last_digit;
}

pub fn get_result(file: &String) -> String {
    let sum = file.lines().fold(0, |acc: u32, val: &str| acc + get_line_num(val));
    return sum.to_string();
}
