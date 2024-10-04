use std::cmp::max;

fn get_round_counts(round: &str) -> [u32; 3]{
    let mut round_count: [u32; 3] = [0, 0, 0];
    round.split(", ").for_each(|dice_info|{
        let mut dice_split = dice_info.split(" ");
        let (dice_num, dice_color) = (dice_split.next().unwrap().parse::<u32>().unwrap(), dice_split.next().unwrap());
        match dice_color {
            "red" => round_count[0] = max(round_count[0], dice_num),
            "green" => round_count[1] = max(round_count[1], dice_num),
            "blue" => round_count[2] = max(round_count[2], dice_num),
            _ => panic!("Invalid color!")
        }
    });

    return round_count;
}

fn calc_game_power(game: &String) -> u32{
    let mut out_arr:[u32; 3] = [0, 0, 0];
    let rounds = game.split(":").skip(1).next().unwrap();

    rounds.split(";").for_each(|round| {
        let requires = get_round_counts(round.trim());
        [out_arr[0], out_arr[1], out_arr[2]] = [max(out_arr[0], requires[0]), max(out_arr[1], requires[1]), max(out_arr[2], requires[2])];
    });
    return out_arr.iter().fold(1, |acc, cur_dice_req| acc * cur_dice_req);
}

pub fn get_result(file: &String) -> String{
    let sum = file.lines().fold(0, |acc, game| acc + calc_game_power(&game.to_string()));
    return sum.to_string();
}