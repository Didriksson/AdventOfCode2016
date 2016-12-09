extern crate regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;



fn is_valid_abba(line: String) -> bool{
    let slice: Vec<_> = line.chars().collect();
    let mut is_within_bracket = false;
    let mut is_valid_abba = false;
    for window in slice.windows(4){
        if window[0] == '[' || window[0] == ']' {
            is_within_bracket = !is_within_bracket;
            continue;
        }

        if window[0] != window[1] && window[0] == window[3] && window[1] == window[2]{
            if is_within_bracket{ return false; }
            else { is_valid_abba = true; }
        }
    }
    is_valid_abba
}

fn is_valid_bab(line: String) -> bool {
    let slice: Vec<_> = line.chars().collect();
    let mut is_within_bracket = false;
    let mut aba_sequences: Vec<String> = Vec::new();
    let mut bab_sequences: Vec<String> = Vec::new();
    for window in slice.windows(3){
        if window[0] == '[' || window[0] == ']' {
            is_within_bracket = !is_within_bracket;
            continue;
        }

        if window[0] != window[1] && window[0] == window[2]{
            if is_within_bracket{
                let char_array = [window[1], window[0], window[1]];
                bab_sequences.push(char_array.into_iter().cloned().collect());
            }
            else {
                aba_sequences.push(window.into_iter().cloned().collect());
            }
        }
    }
    aba_sequences.into_iter().any(|x| bab_sequences.contains(&x))
}

fn get_number_of_aba_compatible_networks(puzzle_input: &String) -> i32{
    let count_valid = puzzle_input.lines().filter(|l| is_valid_bab(l.to_string())).count();
    count_valid as i32
}

fn get_number_of_abba_compatible_networks(puzzle_input: &String) -> i32 {
    let count_valid = puzzle_input.lines().filter(|l| is_valid_abba(l.to_string())).count();
    count_valid as i32
}


fn main() {
    println!("Advent of Code: 7");
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        println!("{:?}", get_number_of_abba_compatible_networks(&args[1]));
        println!("{:?}", get_number_of_aba_compatible_networks(&args[1]));
    }
    else{
        let mut f = File::open("puzzleinput.txt").unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        println!("Abba: {:?}", get_number_of_abba_compatible_networks(&s));
        println!("Aba: {:?}", get_number_of_aba_compatible_networks(&s));
    }
}
#[test]
fn get_number_of_aba_compatible_networks_test(){
    assert_eq!(1, get_number_of_aba_compatible_networks(&String::from("aba[bab]xyz")));
    assert_eq!(0, get_number_of_aba_compatible_networks(&String::from("xyx[xyx]xyx")));
    assert_eq!(1, get_number_of_aba_compatible_networks(&String::from("aaa[kek]eke")));
    assert_eq!(1, get_number_of_aba_compatible_networks(&String::from("zazbz[bzb]cdb")));
}

fn get_number_of_abba_compatible_networks_test(){
     assert_eq!(1, get_number_of_abba_compatible_networks(&String::from("abba[mnop]qrst[olsa]")));
     assert_eq!(0, get_number_of_abba_compatible_networks(&String::from("abcd[bddb]xyyx")));
     assert_eq!(2, get_number_of_abba_compatible_networks(&String::from("abcd[bddb]xyyx\nabba[mnop]qrst\nioxxoj[asdfgh]zxcvbn")));
}

