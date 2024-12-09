use crate::prob;

prob!(d9p1("d9") => |input| -> u64 {
    let mut blocks = input.trim().bytes().enumerate().map(|(i, n)| {
        let count = (n - '0' as u8) as usize;
        vec![if i % 2 == 0 { i as i32 / 2 } else { -1 }; count]
    }).collect::<Vec<_>>();
    let files_rev = blocks.clone().into_iter().enumerate().filter(|(i, _)| i % 2 == 0).rev().collect::<Vec<_>>();

    let mut a = 1;
    let mut b = 0;

    // Iterate through the file blocks in reverse, setting the first blank position in all the blocks to be the id.
    // Once a file block is completed, merge it with the previous free space block. Repeat until only one free space block
    // exists at the end of the disk map.

    0
});
