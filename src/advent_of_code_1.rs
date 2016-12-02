use std::env;
use std::collections::HashSet;

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

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
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

fn get_distance_to_easter_bunny_hq(puzzle_input: &String) -> (i32, i32) {
    let mut direction = Direction::NORTH;
    //Since we assume that we start at 0,0 we can easily count the distance later.
    let mut position = Point{x: 0, y: 0};
    let mut visited_positions = HashSet::new();
    let mut first_visited_node_distance = -9999;
    visited_positions.insert(position);
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
            let initialPosition = position;
            for x in 1..number_of_steps+1{
                //This will return the direction vector
                position.x = initialPosition.x + direction.value().x * x;
                position.y = initialPosition.y + direction.value().y * x;
                if visited_positions.contains(&position) && first_visited_node_distance == -9999{
                    first_visited_node_distance = position.x.abs() + position.y.abs();
                }
                visited_positions.insert(position);
            }
        },
        Err(e) => panic!("Unable to get number of moves. Abort! Kill it with fire!"),
    }
};

if first_visited_node_distance == -9999{
    println!("Warning! No node visited twice. Double check puzzle input. Visited positions: {:?}", visited_positions);
}

println!("Final destination: {:?}", position);
let distance_to_hQ = position.x.abs() + position.y.abs();
println!("Distance: {:?}", distance_to_hQ);
println!("Distance to first already visited node: {:?}", first_visited_node_distance);

(distance_to_hQ, first_visited_node_distance)
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
    assert_eq!(5, get_distance_to_easter_bunny_hq(&String::from("R2, L3")).0);
}
#[test]
fn example2() {
    assert_eq!(2, get_distance_to_easter_bunny_hq(&String::from("R2, R2, R2")).0);
}#[test]
fn example3() {
    assert_eq!(12, get_distance_to_easter_bunny_hq(&String::from("R5, L5, R5, R3")).0);
}
#[test]
fn example4() {
    assert_eq!(4, get_distance_to_easter_bunny_hq(&String::from("R8, R4, R4, R8")).1);
}
