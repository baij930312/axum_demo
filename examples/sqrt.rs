fn main() {
    let num: f64 = 9.0;
    let mut low = 0.0;
    let mut high = num;
    let mut mid = low + (low + high) / 2.0;
    let precise = 1e-9;
    while low <= high {
        mid = (low + high) / 2.0;
        if (num - mid * mid) < precise && (num - mid * mid) > -precise {
            break;
        }
        if mid * mid > num {
            high = mid;
        } else {
            low = mid;
        }
    }
    print!("{}", mid);
}
