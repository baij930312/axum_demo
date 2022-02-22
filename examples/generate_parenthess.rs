fn main() {
    let mut v = vec![];
    recursion(&mut v, 0, 0, 2, "".to_string());
    println!("{:?}", v);
}

fn recursion(v: &mut Vec<String>, left: i32, right: i32, n: i32, s: String) {
    if left == n && right == n {
        v.push(s.clone());
    }
    if left < n {
        recursion(v, left + 1, right, n, format!("{}{}", &s, "("));
    }
    if right < left {
        recursion(v, left, right + 1, n, format!("{}{}", &s, ")"));
    }
}
