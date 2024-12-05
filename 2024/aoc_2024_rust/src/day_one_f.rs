use std::fs;
use crate::quick_sort;

pub fn day_one_f() {
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
    right.sort();
    let mut i: usize = 0;
    let mut sum: i64 = 0;
    loop {
        let li = left[i];
        let ri = right[i];
        let si = li - ri;
        println!("{sum} += {li} - {ri} = {si}");
        sum += si;
        i += 1;
        if i >= left.len() {
            break;
        }
        println!("{i}");
    }
    println!("{}", sum);
}
