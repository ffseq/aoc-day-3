use std::fs::File;
use std::io::{BufReader, prelude::*};


const INPUT_FILE: &str = "input.txt";



fn main() {
    let f = File::open(INPUT_FILE).unwrap();
    let f = BufReader::new(f);

    let mut trees = 0;
    let mut xpos = 0;
    for line in f.lines() {
        let linestr = line.unwrap();
        let repeat = linestr.len();
        match linestr.into_bytes()[xpos % repeat] as char {
            '.' => (),         // empty space, nothing
            '#' => trees += 1, // oof, hit a tree
            c => panic!("Ran into unexpected char! {}", c)
        }
        xpos += 3;
    }

    println!("Trees encountered: {}", trees);
}

