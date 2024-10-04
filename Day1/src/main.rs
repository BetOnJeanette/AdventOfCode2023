use std::fs;
mod part1;
mod part2;

fn main(){
    let input_file = fs::read_to_string("../input.txt").unwrap();
    let first_result = part1::get_result(&input_file);
    let second_result = part2::get_result(&input_file);
    println!("part 1: {first_result}");
    println!("part 2: {second_result}");
}