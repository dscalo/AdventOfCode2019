fn process_intcode(codes: &Vec<i64>, value_at: usize) -> i64 {
    let mut optcodes = codes.clone();

    let mut pos: usize = 0;

    loop {
        match optcodes[pos] {
            1 => {
                let pos1 = optcodes[pos + 1] as usize;
                let pos2 = optcodes[pos + 2] as usize;
                let pos3 = optcodes[pos + 3] as usize;

                optcodes[pos3] = optcodes[pos1] + optcodes[pos2];
            }
            2 => {
                let pos1 = optcodes[pos + 1] as usize;
                let pos2 = optcodes[pos + 2] as usize;
                let pos3 = optcodes[pos + 3] as usize;

                optcodes[pos3] = optcodes[pos1] * optcodes[pos2];
            }
            99 => break,
            _ => panic! {"Bad optcode value"},
        }
        pos += 4;

        if pos >= codes.len() {
            panic!("Invalid incode size!")
        }
    }
    optcodes[value_at]
}

fn part1(optcodes: &Vec<i64>) -> i64 {
    let mut oc = optcodes.clone();
    oc[1] = 12;
    oc[2] = 2;

    process_intcode(&oc, 0)
}

fn part2(optcodes: &Vec<i64>) -> i64 {
    let mut oc = optcodes.clone();

    for i in 0..=99 {
        for j in 0..=99 {
            oc[1] = i;
            oc[2] = j;

            let val = process_intcode(&oc, 0);

            if val == 19690720 {
                return 100 * i + j;
            }
        }
    }
    0
}

fn main() {
    let puzzle = vec![
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 10, 1, 19, 1, 19, 9, 23, 1, 23, 13, 27,
        1, 10, 27, 31, 2, 31, 13, 35, 1, 10, 35, 39, 2, 9, 39, 43, 2, 43, 9, 47, 1, 6, 47, 51, 1,
        10, 51, 55, 2, 55, 13, 59, 1, 59, 10, 63, 2, 63, 13, 67, 2, 67, 9, 71, 1, 6, 71, 75, 2, 75,
        9, 79, 1, 79, 5, 83, 2, 83, 13, 87, 1, 9, 87, 91, 1, 13, 91, 95, 1, 2, 95, 99, 1, 99, 6, 0,
        99, 2, 14, 0, 0,
    ];

    let ans1 = part1(&puzzle);
    let ans2 = part2(&puzzle);

    println!("Part 1: {}", ans1);
    println!("Part 2: {}", ans2);
}
