fn main() {
    println!("{}", pow(2.0, 32));
}

fn pow(x: f64, n: i32) -> f64 {
    let mut x = x;
    let mut n = n;
    if n < 0 {
        x = 1.0 / x;
        n = -n;
    }
    fast_pow(x, n)
}

fn fast_pow(x: f64, n: i32) -> f64 {
    if n == 0 {
        return 1.0;
    }
    let half = fast_pow(x, n / 2);
    if n % 2 == 0 {
        return half * half;
    } else {
        return x * half * half;
    }
}
