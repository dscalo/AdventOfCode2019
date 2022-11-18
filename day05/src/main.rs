extern crate file_reader;
use file_reader::read_file;

extern crate intcode;
use intcode::Intcode;

fn parse_items(s: &str, items: &mut Vec<i64>) {
    let nums: Vec<i64> = s.split(",").map(|i| i.parse::<i64>().unwrap()).collect();

    for n in nums {
        items.push(n);
    }
    
}

fn part1(ins: &Vec<i64>) -> i64 {
    let mut intcode = Intcode::new(ins.clone());
    intcode.add_input(1);
    intcode.process();

    intcode.get_output().unwrap()
}

fn part2(ins: &Vec<i64>) -> i64 {
    let mut intcode = Intcode::new(ins.clone());
    intcode.add_input(5);
    intcode.process();

    intcode.get_output().unwrap()
}


fn main() {
    let mut numbs = Vec::new();
    read_file("puzzle.txt", parse_items, &mut numbs);

    let ans1 = part1(&numbs);
    let ans2 = part2(&numbs);

    println!("Part 1: {}", ans1);
    println!("Part 2: {}", ans2);

   
}
