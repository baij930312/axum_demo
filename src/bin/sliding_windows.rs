use std::collections::VecDeque;

fn main() {
    let res = maximum(vec![1, 2, 3, 65, 5674, 6754, 422, 45, 6], 3);
    println!("{:?}", res);
}

fn maximum(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut v = vec![];
    let mut dequeue: VecDeque<i32> = VecDeque::new();
    if nums.is_empty() || k == 1 {
        return nums;
    }
    for i in 0..nums.len() {
        push(&mut dequeue, nums[i]);
        if (i as i32) > k - 1 {
            pop(&mut dequeue, nums[i - k as usize]);
            v.push(max(&dequeue))
        } else if (i as i32) == k - 1 {
            v.push(max(&dequeue))
        }
    }
    v
}

fn push(dequeue: &mut VecDeque<i32>, n: i32) {
    while !dequeue.is_empty() && *dequeue.back().unwrap() < n {
        dequeue.pop_back();
    }
    dequeue.push_back(n)
}

fn pop(dequeue: &mut VecDeque<i32>, n: i32) {
    if !dequeue.is_empty() && *dequeue.front().unwrap() == n {
        dequeue.pop_front();
    }
}

fn max(dequeue: &VecDeque<i32>) -> i32 {
    *dequeue.front().unwrap()
}
