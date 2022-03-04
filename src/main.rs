

fn main() {
    let v: Result<Vec<(usize, usize)>, SublistsPairsError> =
        sublists_pairs(vec![vec![1, 2], vec![3, 4, 5, 6]]).collect();
    println!("{:?}", v);
}
