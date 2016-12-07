extern crate crypto;
use std::env;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;



fn get_secret_message(puzzle_input: &String) -> (String, String) {
    let mut cols: Vec<HashMap<char, u32>> = Vec::new();
    let lines: Vec<&str> = puzzle_input.split("\n").collect();
    for n in 0..lines[0].len(){
        let mut col: HashMap <char, u32> = HashMap::new();
        cols.push(col);
    }

    for line in lines{
        let chars_in_message: Vec<char> = line.chars().collect();
        for n in 0..line.len(){
            *cols[n].entry(chars_in_message[n]).or_insert(0) += 1;
        }
    }
    let mut message_one_star: Vec<char> = Vec::new();
    let mut message_two_star: Vec<char> = Vec::new();

    for c in cols{
        let mut count_vec: Vec<(&char, &u32)> = c.iter().collect();
            count_vec.sort_by(|a, b| {
                if b.1.cmp(a.1) == Ordering::Equal {
                    b.0.cmp(a.0).reverse()
                }
                else{
                    b.1.cmp(a.1)
                }
            });
            if count_vec.len() > 0{
                message_one_star.push(*count_vec[0].0);
                message_two_star.push(*count_vec[count_vec.len()-1].0);
            }
    }
    (message_one_star.into_iter().collect(), message_two_star.into_iter().collect())
}


fn main() {
    println!("Advent of Code: 6");
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        println!("{:?}", get_secret_message(&args[1]));
    }
    else{
        let mut f = File::open("puzzleinput.txt").unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        println!("{:?}", get_secret_message(&s));
    }
}

#[test]
fn get_secret_message_test(){
     assert_eq!("easter", get_secret_message(&String::from("eedadn\ndrvtee\neandsr\nraavrd\natevrs\ntsrnev\nsdttsa\nrasrtv\nnssdts\nntnada\nsvetve\ntesnvt\nvntsnd\nvrdear\ndvrsen\nenarar")).0);
     assert_eq!("advent", get_secret_message(&String::from("eedadn\ndrvtee\neandsr\nraavrd\natevrs\ntsrnev\nsdttsa\nrasrtv\nnssdts\nntnada\nsvetve\ntesnvt\nvntsnd\nvrdear\ndvrsen\nenarar")).1);

}
