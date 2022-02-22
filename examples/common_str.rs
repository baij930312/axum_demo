fn main() {
    let str1 = "a1g234456aa11";
    let str2 = "11a1234456aa34";
    println!("{}", common_str(str1, str2));
}

fn common_str<'a>(str1: &'a str, str2: &'a str) -> String {
    let mut chars1 = vec![];
    for c in str1.chars() {
        chars1.push(c);
    }
    let mut chars2 = vec![];
    for c in str2.chars() {
        chars2.push(c);
    }
    let len1 = str1.len();
    let len2 = str2.len();
    let mut ss: Vec<String> = vec![];
    for i in 0..len1 {
        let mut s = vec![];
        let mut x = i;
        let mut j = 0;
        while j < len2 {
            if x < len1 && chars1[x] == chars2[j] {
                s.push(chars1[x]);
                x += 1;
            } else {
                x = i;
                if chars1[x] == chars2[j] {
                    j -= 1;
                }
                if !s.is_empty() {
                    ss.push(s.iter().collect());
                    s.clear();
                }
            }
            j += 1;
        }
        if !s.is_empty() {
            ss.push(s.iter().collect());
            s.clear();
        }
    }
    println!("{:?}", ss);
    ss.iter()
        .reduce(|accum, item| {
            if accum.len() >= item.len() {
                accum
            } else {
                item
            }
        })
        .unwrap()
        .clone()
}
