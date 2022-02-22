fn main() {
    let str = "[ ( ) [] {} ]";
    let mut stack: Vec<char> = Vec::with_capacity(10);
    for c in str.chars() {
        match c {
            '[' => stack.push(c),
            ' ' => continue,
            '{' => stack.push(c),
            '(' => stack.push(c),
            ')' => {
                if stack.pop().unwrap() != '(' {
                    panic!("error1");
                }
            }
            ']' => {
                if stack.pop().unwrap() != '[' {
                    panic!("error1");
                }
            }
            '}' => {
                if stack.pop().unwrap() != '{' {
                    panic!("error1");
                }
            }
            _ => panic!("error"),
        }
    }
    if stack.is_empty() {
        println!("{}", "is valid");
    } else {
        println!("{}", "is not valid");
    }
}
