use std::env;
use std::collections::HashSet;
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}


fn get_keycode(puzzle_input: &String) -> Vec<i32> {
    let keypad = [[1, 2,  3],
                 [4, 5, 6 ],
                 [7, 8, 9]];

    let mut keycode = Vec::new();
    let mut position = Point{x: 1, y: 1};

    for line in puzzle_input.split("\n") {
        for command in line.chars(){
            match command {
                'L' => {
                    if position.x > 0 {
                        position.x -= 1;
                    }
                },
                'R' => {
                    if position.x < 2 {
                        position.x += 1;
                    }
                },
                'U' => {
                    if position.y > 0 {
                        position.y -= 1;
                    }
                },
                'D' => {
                    if position.y < 2 {
                        position.y += 1;
                    }
                },
                _ => panic!("Found unexpected input {}", command),
            }
        }
        keycode.push(keypad[position.y as usize][position.x as usize]);
    }
    keycode
}

fn get_keycode_two_stars(puzzle_input: &String) -> Vec<String> {
     let keypad_two_stars = [[" ", " ", "1"," ", " "],
                             [" ", "2", "3","4"," "],
                             ["5", "6", "7","8","9"],
                             [" ", "A", "B","C"," "],
                             [" ", " ", "D"," "," "]];
    let mut keycode_two_stars = Vec::new();

    let mut position = Point{x: 0, y: 2};

    for line in puzzle_input.split("\n") {
        for command in line.chars(){
            match command {
                'L' => {
                    if position.x > 0 && keypad_two_stars[position.y as usize][(position.x -1) as usize] != " "{
                        position.x -= 1;
                    }
                },
                'R' => {
                    if position.x < 4 && keypad_two_stars[position.y as usize][(position.x + 1) as usize] != " "{
                        position.x += 1;
                    }
                },
                'U' => {
                    if position.y > 0 && keypad_two_stars[(position.y -1) as usize][position.x as usize] != " "{
                        position.y -= 1;
                    }
                },
                'D' => {
                    if position.y < 4 && keypad_two_stars[(position.y + 1) as usize][position.x as usize] != " "{
                        position.y += 1;
                    }
                },
                _ => panic!("Found unexpected input {}", command),
            }
        }
        keycode_two_stars.push(String::from(keypad_two_stars[position.y as usize][position.x as usize]));
    }
    keycode_two_stars
}

fn main() {
    println!("Advent of Code: 1");
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expects one argument: Puzze input");
    }
    println!("Keycode: {:?}",get_keycode(&args[1]));
    println!("Keycode: {:?}",get_keycode_two_stars(&args[1]));
    }

    #[test]
    fn example1() {
        let mut expected = Vec::new();
        expected.push(1);
        expected.push(9);
        expected.push(8);
        expected.push(5);
        assert_eq!(expected, get_keycode(&String::from("ULL\nRRDDD\nLURDL\nUUUUD")));
    }

    #[test]
    fn example2() {
        let mut expected = Vec::new();
        expected.push("5");
        expected.push("D");
        expected.push("B");
        expected.push("3");
        assert_eq!(expected, get_keycode_two_stars(&String::from("ULL\nRRDDD\nLURDL\nUUUUD")));
    }
