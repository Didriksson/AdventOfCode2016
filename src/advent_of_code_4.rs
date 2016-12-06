use std::env;
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use std::collections::HashMap;
use std::cmp::Ordering;

extern crate regex;


#[derive(Debug, PartialEq, Eq, Hash)]
struct Room {
    name: String,
    id: i32,
    checksum: String,
}

fn get_sum_of_sector_id(puzzle_input: &String) -> i32 {
    let mut rooms = Vec::new();

    let re = Regex::new(r"(\D+)(\d+)\[([a-z]+)").unwrap();
    let mut number_of_rooms = 0;
    //Get all rooms
    for line in puzzle_input.split("\n"){
        let caps = re.captures(line).unwrap();
        let room = Room{name: String::from(caps.at(1).unwrap()), id: caps.at(2).unwrap().parse::<i32>().unwrap(), checksum: String::from(caps.at(3).unwrap())};
        number_of_rooms += 1;
        //Count number of occurances of a char in name and check with the checksum.
        let mut count: HashMap<char, u32> = HashMap::new();
        for c in room.name.chars() {
            if c == '-' { continue; }
            *count.entry(c).or_insert(0) += 1;
        }
        let mut count_vec: Vec<(&char, &u32)> = count.iter().collect();
        count_vec.sort_by(|a, b| {
            if b.1.cmp(a.1) == Ordering::Equal {
                b.0.cmp(a.0).reverse()
            }
            else{
                b.1.cmp(a.1)
            }
        });

        let checksum_vec: Vec<char> = room.checksum.chars().collect();
        let mut checksum_ok = true;
        for n in 0..checksum_vec.len(){
            if count_vec[n].0 != &checksum_vec[n]{
                checksum_ok = false;
            }
        }
        if checksum_ok{
            rooms.push(room);
        }
    }

    for r in rooms.iter(){
        let re = Regex::new(r"north").unwrap();
        if re.is_match(&change_room_name(r.name.clone(), r.id)){
            println!("Found secret stash: {:?}", r);
        }
    }

    println!("Number of rooms: {:?}", number_of_rooms);
    println!("Number of correct rooms: {:?}", rooms.len());
    let sum = rooms.iter().map(|r| r.id).fold(0,|sum, id| sum + id);
    sum
}

fn shift_char(c : char, number_of_steps : i32) -> char {
    if !c.is_alphabetic() { return c;}
    let char_as_u8 = c as u8;
    let number_of_steps_moduled = (number_of_steps % 26) as u8;
    let shifted_char = char_as_u8 + number_of_steps_moduled;
    if shifted_char > 122 {
        ((shifted_char - 122) + 96) as char
    }
    else{
        shifted_char as char
    }
}

fn change_room_name(name: String, id: i32) -> String{
    let mut stringBuild: Vec<char> = Vec::new();
    for c in name.chars(){
        stringBuild.push(shift_char(c, id));
    }
    stringBuild.into_iter().collect()
}

// For some reason the input data was to long for Powershell.
// Opted for a simple file if no argument is given.
fn main() {
    println!("Advent of Code: 4");
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        println!("{:?}", get_sum_of_sector_id(&args[1]));
    }
    else{
        let mut f = File::open("puzzleinput.txt").unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        println!("{:?}", get_sum_of_sector_id(&s));
    }
}

#[test]
fn example1() {
    assert_eq!(1514, get_sum_of_sector_id(&String::from("aaaaa-bbb-z-y-x-123[abxyz]\na-b-c-d-e-f-g-h-987[abcde]\nnot-a-real-room-404[oarel]\ntotally-real-room-200[decoy]")));
}
#[test]
fn shift_char_test() {
    assert_eq!('a', (97 as char));
    assert_eq!('z', (122 as char));
    assert_eq!('b', shift_char('a', 1));
    assert_eq!('c', shift_char('a', 2));
    assert_eq!('a', shift_char('z', 1));
    assert_eq!('b', shift_char('z', 2));
    assert_eq!('-', shift_char('-', 2));
}

#[test]
fn change_room_test(){
     assert_eq!("very-encrypted-name-", change_room_name(String::from("qzmt-zixmtkozy-ivhz-"), 343));
}
