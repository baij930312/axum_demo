fn main() {
    println!("{}", valid(10003));
}

fn valid(n: i32) -> bool {
    if n <= 1 {
        return true;
    }
    let mut left = 2;
    let mut right = n;

    while left < right {
        let mid = left + (right - left) / 2;
        let square = mid * mid;
        if square == n {
            return true;
        } else if square > n {
            right -= 1;
        } else {
            left += 1;
        }
    }

    false
}
