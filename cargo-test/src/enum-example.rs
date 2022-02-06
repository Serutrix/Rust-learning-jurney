enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn main() {
    //player direction is an type of Direction and the value is the Up variant
    let player_direction:Direction = Direction::Up;

    match player_direction {
        Direction::Up => println!("we are up"),
        Direction::Down => println!("we are Down  "),
        Direction::Left => println!("we are Left "),
        Direction::Right => println!("we are Right "),
    }

}