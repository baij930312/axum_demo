fn main() {
    let v = vec![
        vec![1, 3],
        vec![5, 8],
        vec![7, 9],
        vec![2, 4],
        vec![8, 11],
        vec![12, 15],
    ];
    println!("{:?}", merge(v));
}

fn merge(mut v: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut merged = Vec::new();
    if v.is_empty() {
        return merged;
    }
    v.sort();
    for inteval in v {
        if merged.is_empty() {
            merged.push(inteval);
        } else {
            let size = merged.len();
            if let Some(last_interval) = merged.last() {
                let last = last_interval[1];
                if last < inteval[0] {
                    merged.push(inteval);
                } else {
                    merged[size - 1][1] = inteval[1];
                }
            }
        }
    }
    merged
}
