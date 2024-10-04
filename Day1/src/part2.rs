use std::cmp;
use std::convert::TryFrom;

fn get_first_occurences_of_word(line: &String, num_strs: &Vec<String>) -> Vec<Option<usize>>{
    return num_strs.clone().to_vec().into_iter().map(|num_str| line.find(&num_str)).collect();
}

fn get_one_dir(line: &String, num_strs: &Vec<String>) -> u32 {
    let mut first_indexes = get_first_occurences_of_word(&line, num_strs);

    let first_digit_idx = line.find(char::is_numeric).unwrap();
    let first_digit = usize::try_from((line.as_bytes()[first_digit_idx] as char).to_digit(10).unwrap()).unwrap();
    
    match first_indexes[first_digit] {
        Some(val) => first_indexes[first_digit] = Some(cmp::min(val, first_digit_idx)),
        None => first_indexes[first_digit] = Some(first_digit_idx)
    };

    let min_index = first_indexes.clone().into_iter().fold(None, |cur_idx,cur_min| {
        match cur_idx {
            Some(num) => match cur_min {
                Some(min_num) => return Some(cmp::min(min_num, num)),
                None => return Some(num)
            },
            None => return cur_min
        };
    }).unwrap();

    let first_value = first_indexes.into_iter().position(|val| val == Some(min_index)).unwrap();

    return u32::try_from(first_value).unwrap();
}

fn get_line_num(line: &str, fwd_digit_strs: &Vec<String>, rev_digit_strs: &Vec<String>) -> u32 {
    let reversed_line = rev_word(line.to_string());
    let first_digit = get_one_dir(&line.to_string(), fwd_digit_strs);
    let last_digit = get_one_dir(&reversed_line, rev_digit_strs);
    println!("line: {line} | rev: {reversed_line} | first: {first_digit} | last: {last_digit}");
    return 10 * first_digit + last_digit;
    
}

fn rev_word(word: String) -> String {
    return word.chars().rev().collect::<String>();
}

pub fn get_result(file: &String) -> String {
    let fwd_digit_strs:Vec<String> = vec!["zero","one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].into_iter().map(|ind| ind.to_string()).collect();
    let rev_digit_strs:Vec<String> = fwd_digit_strs.clone().into_iter().map(|word| rev_word(word)).collect();
    let sum = file.lines().fold(0, |acc: u32, val: &str| acc + get_line_num(val, &fwd_digit_strs, &rev_digit_strs));
    return sum.to_string();
}