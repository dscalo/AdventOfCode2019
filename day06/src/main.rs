extern crate file_reader;
use file_reader::read_file;

use std::collections::HashMap;

type AList = HashMap<String, Vec<String>>;

fn parse_items(s: &str, a_list: &mut AList) {
    let ss: Vec<String> = s.split(")").map(|i| i.to_string()).collect();

    match a_list.get_mut(&ss[0]) {
        Some(v) => v.push(ss[1].clone()),
        None => {
            let v = vec![ss[1].clone()];
            a_list.insert(ss[0].clone(), v);
        }
    }
}

fn parse_items_part2(s: &str, a_list: &mut AList) {
    let ss: Vec<String> = s.split(")").map(|i| i.to_string()).collect();

    match a_list.get_mut(&ss[0]) {
        Some(v) => v.push(ss[1].clone()),
        None => {
            let v = vec![ss[1].clone()];
            a_list.insert(ss[0].clone(), v);
        }
    }

    match a_list.get_mut(&ss[1]) {
        Some(v) => v.push(ss[0].clone()),
        None => {
            let v = vec![ss[0].clone()];
            a_list.insert(ss[1].clone(), v);
        }
    }
}

fn find_orbits(a_list: &AList, index: &String) -> i64 {
    let mut ct = -1;

    let mut to_visit: Vec<String> = Vec::new();

    to_visit.push(index.clone());

    while to_visit.len() > 0 {
        let idx = to_visit.pop().unwrap();

        ct += 1;

        if let Some(list) = a_list.get(&idx) {
            for l in list {
                to_visit.push(l.clone());
            }
        }
    }

    ct
}

fn part1(a_list: &AList) -> i64 {
    let mut tot = 0;
    for key in a_list.keys() {
        let t = find_orbits(&a_list, &key);
        //println!("Key: {}, orbits: {}", key, t);
        tot += t;
    }

    tot
}

struct Node(String, i64);

fn part2(a_list: &AList) -> i64 {
    let dest = String::from("SAN");
    let mut dist = i64::MAX;

    let mut to_visit = vec![Node(String::from("YOU"), 0)];
    let mut visited: Vec<String> = Vec::new();

    while to_visit.len() > 0 {
        let node = to_visit.pop().unwrap();
        visited.push(node.0.clone());

        if let Some(list) = a_list.get(&node.0) {
            for l in list {
                if *l == dest {
                    if node.1 < dist {
                        dist = node.1 - 1;
                    }
                }
                if !visited.contains(&l) {
                    to_visit.push(Node(l.clone(), node.1 + 1));
                }
            }
        }
    }

    dist
}

fn main() {
    let mut a_list = AList::new();
    let mut a_list_p2 = AList::new();
    read_file("puzzle.txt", parse_items, &mut a_list);
    read_file("puzzle.txt", parse_items_part2, &mut a_list_p2);

    //println!("{:?}", a_list);

    let ans1 = part1(&a_list);
    let ans2 = part2(&a_list_p2);

    println!("Part 1: {}", ans1);
    println!("Part 2: {}", ans2);
}
