use std::collections::HashMap;

fn main() {
    let nums = vec![4, 4, 45, 456, 77, 645];
    let target = 8;
    let mut map = HashMap::new();
    let mut res = vec![];
    for (i, v) in nums.iter().enumerate() {
        map.insert(*v, i);
    }

    for (i, v) in nums.iter().enumerate() {
        if let Some(index) = map.get(&(target - *v)) {
            if *index == i {
                continue;
            }
            res.push(i);
            res.push(*index);
        }
    }
    println!("{:?}", res)
}
