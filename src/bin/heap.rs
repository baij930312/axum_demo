
fn main() {
    let mut v = vec![5, 5, 23, 55, 77, 8, 90];
    build_heap(&mut v);
    insert(&mut v, 100);
    println!("{:?}", v);
}

// 左子节点 2x+1  右子节点 2x+2  父节点 x-1/2
fn build_heap(nums: &mut Vec<i32>) {
    for i in (0..nums.len() / 2).rev() {
        heapify_up_to_down(nums, i);
    }
}

fn heapify_up_to_down(nums: &mut Vec<i32>, i: usize) {
    let mut index = i;
    let mut max_pos = index;
    loop {
        if 2 * index + 1 < nums.len() && nums[2 * index + 1] > nums[max_pos] {
            max_pos = 2 * index + 1;
        }
        if 2 * index + 2 < nums.len() && nums[2 * index + 2] > nums[max_pos] {
            max_pos = 2 * index + 2;
        }
        if max_pos == index {
            break;
        }
        nums.swap(index, max_pos);
        index = max_pos;
    }
}

fn heapify_down_to_up(nums: &mut Vec<i32>, i: usize) {
    let mut index = i;
    let mut parent_index = (index - 1) / 2;
    while nums[index] > nums[parent_index] {
        nums.swap(index, parent_index);
        index = parent_index;
        if index == 0 {
            break;
        }
        parent_index = (index - 1) / 2;
    }
}

fn insert(nums: &mut Vec<i32>, n: i32) -> bool {
    nums.push(n);
    heapify_down_to_up(nums, nums.len() - 1);
    true
}

fn remove(nums: &mut Vec<i32>) -> Option<i32> {
    if nums.is_empty() {
        return None;
    }
    let len = nums.len();
    nums.swap(0, len - 1);
    let res = nums.pop();
    heapify_up_to_down(nums, 0);
    res
}
