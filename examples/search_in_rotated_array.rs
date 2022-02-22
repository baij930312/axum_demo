fn main() {
    let v = vec![4, 5, 6, 7, 8, 0, 1, 2, 3];

    println!("{}", search(&v, 3));
}

fn search(v: &Vec<i32>, t: i32) -> i32 {
    let mut left = 0;
    let mut right = v.len() - 1;
    while left <= right {
        let mid = left + (right - left) / 2;
        if v[mid] == t {
            return mid as i32;
        } else if v[left] <= v[mid] {
            //前半部分有序
            if v[left] <= t && t < v[mid] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        } else {
            //后半部分有序
            if v[mid] < t && t <= v[right] {
                left = mid - 1;
            } else {
                right = mid + 1;
            }
        }
    }
    -1
}
