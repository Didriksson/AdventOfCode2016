extern crate crypto;
use std::env;
use crypto::md5::Md5;
use crypto::digest::Digest;
use std::collections::BTreeMap;



fn get_door_code_one_star(puzzle_input: &String) -> String {
    let mut hasher = Md5::new();
    let mut index = 0;
    let mut door_code = String::new();
    println!("Starting 1337-Haxxor decoding for door code with puzzle input ONE STAR {:?}. This may take awhile. Exit anytime by pressing ctrl + c", puzzle_input);
    while door_code.len() != 8{
        hasher.input_str(puzzle_input);
        hasher.input_str(&index.to_string());
        let result = hasher.result_str();
        if &result[..5] == "00000"{
             door_code.push_str(&result[5..6]);
             println!("Made progress with the code at index: {:?} Code so far: {:?}", index, door_code);
        }
        index = index  + 1;
        hasher.reset();
    }
    door_code
}

fn get_door_code_second_star(puzzle_input: &String) -> String {
    let mut hasher = Md5::new();
    let mut index: i64 = 0;
    let mut door_code: BTreeMap<i32, String> = BTreeMap::new();
    println!("Starting 1337-Haxxor decoding for door code with puzzle input SECOND STAR {:?}. This may take awhile. Exit anytime by pressing ctrl + c", puzzle_input);
    while door_code.len() != 8{
        hasher.input_str(puzzle_input);
        hasher.input_str(&index.to_string());
        let result = hasher.result_str();
        if &result[..5] == "00000"{
            let position = match result[5..6].parse::<i32>() {
                Err(e) => {-1},
                Ok(f) => f,
            };

            if position >= 0 && position < 8 && !door_code.contains_key(&position){
                 door_code.insert(position, String::from(&result[6..7]));
                 println!("Made progress! Code so far: {:?}", door_code);
            }
        }
        index = index  + 1;
        hasher.reset();
    }
    println!("Finnished! Door code: {:?}", door_code);
    let stringbuilde: Vec<String> = door_code.values().cloned().collect();
    stringbuilde.join("")
}

fn main() {
    println!("Advent of Code: 5");
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        println!("{:?}", get_door_code_second_star(&args[1]));
        println!("{:?}", get_door_code_one_star(&args[1]));

    }
}

#[test]
fn door_code_one_star_test(){
     assert_eq!("18f47a30", get_door_code_one_star(&String::from("abc")));
}
#[test]
fn door_code_second_star_test(){
     assert_eq!("05ace8e3", get_door_code_second_star(&String::from("abc")));
}
