static RED_MAX:u32 = 12;
static GREEN_MAX:u32 = 13;
static BLUE_MAX:u32 = 14;

fn is_cube_count_valid(cube_count: &u32, cube_color: &String) -> bool{
    match cube_color.as_str() {
        "red" => return cube_count <= &RED_MAX,
        "green" => return  cube_count <= &GREEN_MAX,
        "blue" => return cube_count <= &BLUE_MAX,
        _ => panic!("Invalid color!")
    }
}

fn is_grab_valid(grab: &String) -> bool{
    let mut cube_groups = grab.split(", ");
    return cube_groups.all(|group| {
        let mut group_info = group.trim().split(" ");
        let (cube_count , cube_color) = (group_info.next().unwrap().parse::<u32>().unwrap(), group_info.next().unwrap().to_string());
        return is_cube_count_valid(&cube_count, &cube_color);
    })

}

fn proccess_game(game: &String) -> u32{
    let mut game_split = game.split(":");
    let [game_label, rounds] = [game_split.next().unwrap(), game_split.next().unwrap()];
    let game_num = game_label.split(" ").skip(1).next().unwrap().parse::<u32>().unwrap();

    if rounds.split(";").any(|val| !is_grab_valid(&val.to_string())) {
        return 0;
    };
    return game_num;
}

pub fn get_result(file: &String) -> String{
    let sum = file.lines().fold(0, |acc, game| acc + proccess_game(&game.to_string()));
    return sum.to_string();
}