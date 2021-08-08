use std::fs::File;
use std::io::{BufReader, prelude::*};


const INPUT_FILE: &str = "input.txt";

fn main() {
    let mut v = Vec::new();
    v.push(find_trees(1, 1));
    v.push(find_trees(3, 1));
    v.push(find_trees(5, 1));
    v.push(find_trees(7, 1));
    v.push(find_trees(1, 2));

    let finalnum = v.into_iter().reduce(|acc, c| { acc * c}).unwrap();
    println!("All multiplied: {}", finalnum);
}

fn find_trees(x_increment: usize, y_increment: usize) -> usize {
    let f = File::open(INPUT_FILE).unwrap();
    let f = BufReader::new(f);

    let mut trees = 0;
    let mut xpos = 0;
    let mut ypos = 0;
    for line in f.lines() {
        let check_line = ypos % y_increment == 0;
        if check_line {
            let linestr = line.unwrap();
            let repeat = linestr.len();
            match linestr.into_bytes()[xpos % repeat] as char {
                '.' => (),         // empty space, nothing
                '#' => trees += 1, // oof, hit a tree
                c => panic!("Ran into unexpected char! {}", c)
            }
            xpos += x_increment;
        }
        ypos += 1;
    }

    println!("Trees encountered: {}", trees);
    
    trees
}

