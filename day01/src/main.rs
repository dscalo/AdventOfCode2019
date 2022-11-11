extern crate file_reader;
use file_reader::read_file;

fn parse_items(s: &str, items: &mut Vec<i64>) {
    let num = s.parse::<i64>().unwrap();
    items.push(num);
}

fn calculate_fuel(mass: i64) -> i64 {
    let d = mass as f64 / 3.0;
    let f = d.floor() as i64 - 2;
    if f < 0 {
        return 0;
    }
    f
}

fn part1(numbs: &Vec<i64>) -> i64 {
    let mut tot = 0;
    for numb in numbs {
        tot += calculate_fuel(*numb);
    }
    tot
}

fn part2(numbs: &Vec<i64>) -> i64 {
    let mut tot = 0;

    for numb in numbs {
        let mut f = calculate_fuel(*numb);

        tot += f;
        while f > 0 {
            f = calculate_fuel(f);
            tot += f
        }
    }

    tot
}

fn main() {
    println!("mass 12 fuel: {}", calculate_fuel(12));
    println!("mass 100756, fuel: {}", calculate_fuel(100756));

    let mut numbs = Vec::new();
    read_file("puzzle.txt", parse_items, &mut numbs);

    let p1 = part1(&numbs);
    let p2 = part2(&numbs);

    println!("Answer for Part 1 is {}", p1);
    println!("Answer for Part 2 is {}", p2);
}
