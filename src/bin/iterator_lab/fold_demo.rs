fn main() {
    let v = vec![1u64, 2, 3, 4, 5, 6];
    let val = v.iter()
        .enumerate()
        // 每两个元素剔除一个
        // [1, 3, 5]
        .filter(|&(idx, _)| idx % 2 == 0)
        .map(|(_, val)| val)
        // 累加 1+3+5 = 9
        // .sum::<u64>();
        // 也可以使用fold()
        // 如果只是将迭代器中的元素简单累加就是直接使用sum()
        // 如果是要对每一步累加做一些额外操作比如.fold(0u64, |sum, acm| sum + (acm - 1) * 2);
        // 则只能用fold实现，无法使用sum实现
        .fold(0u64, |sum, acm| sum + (acm - 1) * 2);

    println!("{}", val); // 结果是12
}