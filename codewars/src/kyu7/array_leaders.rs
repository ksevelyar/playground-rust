fn array_leaders(arr: &[i32]) -> Vec<i32> {
    arr.iter()
        .enumerate()
        .filter(|(ind, num)| is_leader(arr, *ind, **num))
        .map(|(_ind, num)| *num)
        .collect()
}

fn is_leader(arr: &[i32], ind: usize, num: i32) -> bool {
    num > arr[ind..].iter().sum::<i32>() - num
}
