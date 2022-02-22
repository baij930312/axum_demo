use core::num;

fn main() {
    let mut v = vec![4,6,67,8,87,32,3,12213,1,];
    let l = v.len();
    // find_kth_largest(&mut v, 0, l - 1, 2);

    println!("{:?}", reverse_pairs(&mut v, 0, l - 1));
}

fn bubble(mut v: Vec<i32>) -> Vec<i32> {
    let len = v.len();
    for i in 0..len {
        for j in 0..i {
            if v[i] < v[j] {
                v.swap(j, i);
            }
        }
    }
    v
}

fn fast_sort(v: &mut Vec<i32>, left: usize, right: usize) {
    if left >= right {
        return;
    }
    let index = position(v, left, right);
    if index != 0 {
        fast_sort(v, left, index - 1);
    }
    fast_sort(v, index + 1, right);
}

fn position(v: &mut Vec<i32>, left: usize, right: usize) -> usize {
    let pslot = right;
    let mut i = left;
    for j in left..right {
        if v[j] < v[pslot] {
            v.swap(j, i);
            i += 1;
        }
    }
    v.swap(i, pslot);
    i
}

fn find_kth_largest(v: &mut Vec<i32>, left: usize, right: usize, k: usize) -> i32 {
    if left == right {
        return v[left];
    }
    let index = position_k(v, left, right, right);
    // println!("{}", index);
    if index == k {
        return v[index];
    } else if index > k {
        return find_kth_largest(v, left, index - 1, k);
    } else {
        return find_kth_largest(v, index + 1, right, k);
    };
}

fn position_k(v: &mut Vec<i32>, left: usize, right: usize, k: usize) -> usize {
    let p = v[k];
    let mut i = left;
    for j in left..right {
        if v[j] > p {
            v.swap(j, i);
            i += 1;
        }
    }
    v.swap(i, k);
    i
}

fn merge_sort(v: &mut Vec<i32>, left: usize, right: usize) {
    if left >= right {
        return;
    }
    let mid = left + (right - left) / 2;
    merge_sort(v, left, mid);
    merge_sort(v, mid + 1, right);
    merge(v, left, right, mid);
}

fn merge(v: &mut Vec<i32>, left: usize, right: usize, mid: usize) {
    let mut i = left;
    let mut j = mid + 1;
    let mut k = left;
    let mut nums = vec![];
    while k <= right {
        if i > mid {
            nums.push(v[j]);
            j += 1;
            k += 1;
        } else if j > right {
            nums.push(v[i]);
            i += 1;
            k += 1;
        } else if v[i] < v[j] {
            nums.push(v[i]);
            i += 1;
            k += 1;
        } else {
            nums.push(v[j]);
            j += 1;
            k += 1;
        }
    }
    for i in 0..=(right - left) {
        v[left + i] = nums[i];
    }
    println!("{:?}", v);
}

fn reverse_pairs(v: &mut Vec<i32>, left: usize, right: usize) -> usize {
    if left >= right {
        return 0;
    }
    let mid = left + (right - left) / 2;
    let mut count = reverse_pairs(v, left, mid) + reverse_pairs(v, mid + 1, right);
    let mut i = left;
    let mut j = mid + 1; 
    while i <= mid && j <= right {
        if v[i] as i64 > 2 * v[j] as i64 {
            count += mid - i + 1;
            j += 1;
        } else {
            i += 1;
        }
    }
    println!("{}", count);
    merge(v, left, right, mid);
    count
}
