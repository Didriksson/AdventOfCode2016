use std::env;


#[derive(Debug)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST
}
 #[derive(PartialEq)]
enum Rotation{
    RIGHT,
    LEFT
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Direction{
    fn value(&self) -> Point{
        match *self{
            Direction::NORTH => Point{x: 0, y: 1},
            Direction::EAST => Point{x: 1, y: 0},
            Direction::SOUTH => Point{x: 0, y: -1},
            Direction::WEST => Point{x: -1, y: 0},
        }
    }

    fn rotate(&self, rot: Rotation) -> Direction {
        match *self{
            Direction::NORTH => {
                if rot == Rotation::RIGHT {
                    Direction::EAST
                }
                else{
                    Direction::WEST
                }
            },
            Direction::EAST => {
                if rot == Rotation::RIGHT{
                    Direction::SOUTH
                }
                else{
                    Direction::NORTH
                }
            },
            Direction::SOUTH => {
                if rot == Rotation::RIGHT{
                    Direction::WEST
                }
                else{
                    Direction::EAST
                }
            },
            Direction::WEST => {
                if rot == Rotation::RIGHT{
                    Direction::NORTH
                }
                else{
                    Direction::SOUTH
                }
            },
        }
    }
}

fn get_distance_to_easter_bunny_hq(puzzle_input: &String) -> i32 {
    let mut direction = Direction::NORTH;
    //Since we assume that we start at 0,0 we can easily count the distance later.
    let mut position = Point{x: 0, y: 0};

    for command in puzzle_input.split(", ") {
        let rotation_direction = command.chars().next().unwrap();
        if rotation_direction == 'R'{
            direction = direction.rotate(Rotation::RIGHT);
        }
        else if rotation_direction == 'L'{
            direction = direction.rotate(Rotation::LEFT);
        }
    else { panic!("Expected R or L found {}", command)}

    match command[1..command.len()].parse::<i32>(){
        Ok(number_of_steps) => {
            //This will return the direction vector
            position.x += direction.value().x * number_of_steps;
            position.y += direction.value().y * number_of_steps;
        },
        Err(e) => panic!("Unable to get number of moves. Abort! Kill it with fire!"),
    }
};

println!("Final destination: {:?}", position);
let distance = position.x.abs() + position.y.abs();
println!("Distance: {:?}", distance);
distance
}

fn main() {
    println!("Advent of Code: 1");
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expects one argument: Puzze input");
    }
    get_distance_to_easter_bunny_hq(&args[1]);
}

#[test]
fn example1() {
    assert_eq!(5, get_distance_to_easter_bunny_hq(&String::from("R2, L3")));
}
#[test]
fn example2() {
    assert_eq!(2, get_distance_to_easter_bunny_hq(&String::from("R2, R2, R2")));
}#[test]
fn example3() {
    assert_eq!(12, get_distance_to_easter_bunny_hq(&String::from("R5, L5, R5, R3")));
}
