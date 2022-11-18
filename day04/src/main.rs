
fn validate(password: &Vec<u32>) -> bool {

    let mut has_double = false;

    for i in 0..password.len() -1 {
        let cur = password[i];
        let nxt = password[i+1];

        if cur > nxt {
            return false;
        }

        if cur == nxt && !has_double {
            has_double = true
        }

    }

    true && has_double
}

/*
    the two adjacent matching digits are not part of a larger group of matching digits
*/
fn validate_strict(password: &Vec<u32>) -> bool {

    let mut dupe_ct = 0;
    let mut has_double = false;

    for i in 0..password.len() -1 {
        let cur = password[i];
        let nxt = password[i+1];

        if cur > nxt {
            return false;
        }

        if cur == nxt  {
           dupe_ct += 1;
        } else {
            if dupe_ct == 1 {
                has_double = true;
            }
            dupe_ct = 0;
        }
    }    

    true &&( has_double || !has_double && dupe_ct == 1)
}



fn add_1(v: &mut Vec<u32>) {
    let mut idx: usize = v.len() -1;
    let mut carry = true;

    loop {
        if !carry {
            break;
        }
        carry = false;

        if v[idx] == 9 {
            v[idx] = 0;           
            carry = true;
        } else {
            v[idx] += 1;
        }

        if idx == 0 {
            break;
        }

        idx -= 1;

    }

}

fn part1(range_start: &Vec<u32>, steps: u32) -> u32 {
    let mut good_passwords = 0;

    let mut cur = range_start.clone();

    let mut ctr = 0;
    for _ in 0..steps {
        if validate(&cur) {
            good_passwords +=1;
        }

        add_1(&mut cur);

        ctr +=1;

    }

    println!("Total steps taken : {}", ctr);
    println!("array: {:?}", cur);


    good_passwords
}

fn part2(range_start: &Vec<u32>, steps: u32) -> u32 {
    let mut good_passwords = 0;

    let mut cur = range_start.clone();

    let mut ctr = 0;
    for _ in 0..steps {
        if validate_strict(&cur) {
            good_passwords +=1;
        }

        add_1(&mut cur);

        ctr +=1;

    }

    println!("Total steps taken : {}", ctr);
    println!("array: {:?}", cur);


    good_passwords
}


fn main() {
    // puzzle : 136760-595730
    let steps = 595730-136760;

    let start = vec![1,3,6,7,6,0];

    let ans1 = part1(&start, steps);
    let ans2 = part2(&start, steps);

    println!("Part 1: {}", ans1);
    println!("Part 2: {}", ans2);
}



#[cfg(test)]
mod tests {
    use super::*; // need this to test private methods
    #[test]
    fn all_the_same() {
        assert_eq!(validate(&vec![1,1,1,1,1,1]), true);
    }

    #[test]
    fn increasing_with_double() {
        assert_eq!(validate(&vec![1,2,3,3,4,9]), true);
    }

    #[test]
    fn decreasing_value() {
        assert_eq!(validate(&vec![2,2,3,4,5,0]), false);
    }

    
    #[test]
    fn increasing_no_double() {
        assert_eq!(validate(&vec![1,2,3,4,5,6]), false);
    }

    #[test]
    fn sets_of_doubles() {
        assert_eq!(validate_strict(&vec![1,1,2,2,3,3]), true);
    }

    #[test]
    fn quad_with_double() {
        assert_eq!(validate_strict(&vec![1,1,1,1,4,4]), true);
    }

    #[test]
    fn triple_no_double() {
        assert_eq!(validate_strict(&vec![1,2,3,4,4,4]), false);
    }

    #[test]
    fn adding_1_no_carry() {
        let mut nums = vec![1,2,3,4,5,6];
        let expect = vec![1,2,3,4,5,7];

        add_1(&mut nums);
        assert_eq!(nums, expect);
    }

    #[test]
    fn adding_1_with_1_carry() {
        let mut nums = vec![1,2,3,4,5,9];
        let expect = vec![1,2,3,4,6,0];

        add_1(&mut nums);
        assert_eq!(nums, expect);
    }

    #[test]
    fn adding_1_with_3_carry() {
        let mut nums = vec![1,2,3,4,9,9];
        let expect = vec![1,2,3,5,0,0];

        add_1(&mut nums);
        assert_eq!(nums, expect);
    }


    #[test]
    fn adding_1_with_5_carry() {
        let mut nums = vec![4,9,9,9,9,9];
        let expect = vec![5,0,0,0,0,0];

        add_1(&mut nums);
        assert_eq!(nums, expect);
    }   

}