use std::fs;
mod part1;

fn main(){
    let input_file = fs::read_to_string("../input.txt").unwrap();
    let first_result = part1::get_result(&input_file);
    println!("part 1: {first_result}");
}