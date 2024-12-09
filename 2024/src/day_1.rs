use std::collections::HashMap;

use crate::prob;

prob!(d1p1("d1") => |input| -> u64 {
    let mut left: Vec<u64> = vec![];
    let mut right: Vec<u64> = vec![];

    for line in (0..input.len()).step_by(5 + 3 + 5 + 1) {
        left.push(input[line..line + 5].parse().unwrap());
        right.push(input[line + 5 + 3..line + 5 + 3 + 5].parse().unwrap());
    }

    left.sort();
    right.sort();

    left.iter().zip(right).map(|(l, r)| l.abs_diff(r)).sum::<u64>()
});

prob!(d1p2("d1") => |input| -> u64 {
    let mut left: Vec<u64> = vec![];
    let mut right: HashMap<u64, u64> = HashMap::new();

    for line in (0..input.len()).step_by(5 + 3 + 5 + 1) {
        left.push(input[line..line + 5].parse().unwrap());
        right.entry(input[line + 5 + 3..line + 5 + 3 + 5].parse().unwrap()).and_modify(|u| *u += 1).or_insert(1);
    }

    left.iter().map(|l| l * right.get(l).unwrap_or(&0)).sum::<u64>()
});
