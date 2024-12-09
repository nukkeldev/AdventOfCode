use crate::prob;

prob!(d2p1("d2") => |input| -> u64 {
    input.lines().map(|l| {
        let levels = l.split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect::<Vec<_>>();
        let safe = (levels.is_sorted() || levels.iter().rev().is_sorted()) &&
            levels.iter().map_windows(|[&a, &b]| {let diff = a.abs_diff(b); diff >= 1 && diff <= 3}).fold(true, |v, b| v && b);
        safe as u64
}   ).sum()
});
