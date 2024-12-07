use std::fs;

pub fn day_one_s() {
    let input = fs::read_to_string("./../input/dayOne.txt").expect("Couldn't find file");
    let mut left: Vec<i64> = Vec::new();
    let mut right: Vec<i64> = Vec::new();
    for line in input.split("\n") {
        if line.len() <= 0 {
            continue;
        }
        let mut i: i8 = 0;
        for num in line.split("   ") {
            if i == 0 {
                left.push(num.trim().parse().expect("Number"));
                i += 1;
            } else {
                right.push(num.trim().parse().expect("Number"));
            }
        }
    }
    left.sort();
    left.dedup();
    right.sort();
    let mut sum: i64 = 0;
    for un in left {
        let mut i: usize = 0;
        loop {
            let ri = right[i];
            i += 1;
            if ri == un {
                sum += un;
            }
            if i >= right.len() {
                break;
            }
        }
    }
    println!("{}", sum);
}
