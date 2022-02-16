fn main() {
    let n = 4;
    let mut v = vec![0; n];
    println!("{}", recursion(n, &mut v));
}

fn recursion(n: usize, v: &mut Vec<u32>) -> u32 {
    if n <= 2 {
        return n as u32;
    }
    let n = n as usize;
    if v[n - 1] == 0 {
        v[n - 1] = recursion(n - 1, v);
    }
    if v[n - 2] == 0 {
        v[n - 2] = recursion(n - 2, v);
    }
    v[n - 1] + v[n - 2]
}
