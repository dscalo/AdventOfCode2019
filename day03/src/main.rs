extern crate file_reader;
use file_reader::read_file;

#[derive(Debug)]
struct mov {
    dir: char,
    numb: i32,
}

impl mov {
    fn new(dir: char, numb: i32) -> mov {
        mov { dir, numb }
    }
}

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(start: Point, end: Point) -> Line {
        Line { start, end }
    }
}

type Moves = Vec<Vec<mov>>;

fn parse_moves(s: &str, moves: &mut Moves) {
    let line = s.parse::<String>().unwrap();

    let line_moves = line
        .split(",")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut ms = Vec::new();

    for lm in line_moves {
        let chars = lm.chars().collect::<Vec<char>>();

        let numb = chars[1..]
            .iter()
            .collect::<String>()
            .parse::<i32>()
            .unwrap();

        ms.push(mov::new(chars[0], numb));
    }
    moves.push(ms);
}

fn create_lines(moves: &Vec<mov>) -> Vec<Line> {
    let mut start = Point::new(0, 0);
    let mut end = Point::new(0, 0);
    let mut lines = Vec::new();

    for m in moves {
        match m.dir {
            'U' => {
                end.x = start.x;
                end.y = start.y + m.numb;
            }
            'R' => {
                end.x = start.x + m.numb;
                end.y = start.y;
            }
            'D' => {
                end.x = start.x;
                end.y = start.y - m.numb;
            }
            'L' => {
                end.x = start.x - m.numb;
                end.y = start.y;
            }
            _ => panic!("Invalid move"),
        }

        let l = Line::new(start.clone(), end.clone());
        lines.push(l);
        start.x = end.x;
        start.y = end.y;
        end.x = 0;
        end.y = 0;
    }

    lines
}

fn get_distance(p1: &Point, p2: &Point) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn intersect(l1: &Line, l2: &Line) -> Option<Point> {
    let x1 = l1.start.x as f64;
    let x2 = l1.end.x as f64;
    let x3 = l2.start.x as f64;
    let x4 = l2.end.x as f64;
    let y1 = l1.start.y as f64;
    let y2 = l1.end.y as f64;
    let y3 = l2.start.y as f64;
    let y4 = l2.end.y as f64;

    let t_numerator = ((x1 - x3) * (y3 - y4)) - ((y1 - y3) * (x3 - x4));
    let t_denominator = ((x1 - x2) * (y3 - y4)) - ((y1 - y2) * (x3 - x4));

    if t_denominator == 0.0 {
        return None;
    }
    let t = t_numerator / t_denominator;

    let u_numerator = ((x1 - x3) * (y1 - y2)) - ((y1 - y3) * (x1 - x2));
    let u_denominator = ((x1 - x2) * (y3 - y4)) - ((y1 - y2) * (x3 - x4));

    if u_denominator == 0.0 {
        return None;
    }
    let u = u_numerator / u_denominator;

    if t > 0.0 && t < 1.0 && u > 0.0 && u < 1.0 {
        // the x,y where the 2 lines intersect
        let x = (x1 + t * (x2 - x1)) as i32;
        let y = (y1 + t * (y2 - y1)) as i32;
        return Some(Point::new(x, y));
    }

    None
}

fn part1(moves: &Moves) -> i32 {
    let lines1 = create_lines(&moves[0]);
    let lines2 = create_lines(&moves[1]);

    let origin = Point::new(0, 0);

    let mut distance = i32::MAX;

    for l1 in &lines1 {
        for l2 in &lines2 {
            if let Some(p) = intersect(&l1, &l2) {
                let d = get_distance(&p, &origin);

                if d < distance {
                    distance = d;
                }
            }
        }
    }
    distance
}

fn get_steps(lines: &Vec<Line>, index: usize) -> i32 {
    let mut steps = 0;

    for i in 0..index {
        steps +=
            (lines[i].start.x - lines[i].end.x).abs() + (lines[i].start.y - lines[i].end.y).abs();
    }

    steps
}

fn part2(moves: &Moves) -> i32 {
    let lines1 = create_lines(&moves[0]);
    let lines2 = create_lines(&moves[1]);

    let mut steps = i32::MAX;

    for i in 0..lines1.len() {
        for j in 0..lines2.len() {
            if let Some(p) = intersect(&lines1[i], &lines2[j]) {
                let line1_steps = get_steps(&lines1, i)
                    + (p.x - lines1[i].start.x).abs()
                    + (p.y - lines1[i].start.y).abs();
                let line2_steps = get_steps(&lines2, j)
                    + (p.x - lines2[j].start.x).abs()
                    + (p.y - lines2[j].start.y).abs();

                if line1_steps + line2_steps < steps {
                    steps = line1_steps + line2_steps;
                }
            }
        }
    }
    steps
}

fn main() {
    let mut moves = Vec::new();

    read_file("puzzle.txt", parse_moves, &mut moves);

    let ans1 = part1(&moves);
    let ans2 = part2(&moves);

    println!("Part 1 answer is: {}", ans1);
    println!("Part 2 answer is: {}", ans2);
}
