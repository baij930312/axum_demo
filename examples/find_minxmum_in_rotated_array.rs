fn main() {
    let v = vec![4, 5, 6, 7, 8, 0, 1, 2, 3];

    println!("{}", minxmum(&v));
}

fn minxmum(v: &Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = v.len() - 1;
    if v[right] > v[left] {
        //没有旋转点
        return v[0];
    }
    while left <= right {
        let mid = left + (right - left) / 2;
        if v[mid] > v[mid + 1] {
            return v[mid + 1];
        }
        if v[mid] < v[mid - 1] {
            return v[mid];
        }
        if v[mid] < v[0] {
            right = mid;
        }
        if v[mid] > v[0] {
            left = mid;
        }
    }
    -1
}
