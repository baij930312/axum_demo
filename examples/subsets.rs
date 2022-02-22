fn main() {
    let mut v: Vec<i32> = vec![];
    let mut vs: Vec<Vec<i32>> = vec![];
    let nums = vec![1, 2, 3];
    backtrack(&nums, &mut vs, &mut v, 0);
    println!("{:?}", vs);
}

fn backtrack(nums: &Vec<i32>, vecs: &mut Vec<Vec<i32>>, vec: &mut Vec<i32>, start: usize) {
    vecs.push(vec.clone());
    for i in start..nums.len() {
        vec.push(nums[i]);
        backtrack(nums, vecs, vec, start + 1);
        vec.remove(vec.len() - 1);
    }
}
