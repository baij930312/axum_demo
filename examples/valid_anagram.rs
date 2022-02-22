use std::collections::HashMap;

fn main() {
    group_anagram();
    // let a = "asdasasdasd";
    // let b = "dasasdasasd";

    // let mut mapa = HashMap::new();
    // let mut mapb = HashMap::new();

    // for c in a.chars() {
    //     let count = mapa.entry(c).or_insert(0);
    //     *count += 1;
    // }

    // for c in b.chars() {
    //     let count = mapb.entry(c).or_insert(0);
    //     *count += 1;
    // }

    // if mapa.keys().len() != mapb.keys().len() {
    //     panic!("asd");
    // }

    // for c in mapa.keys() {
    //     if let Some(v) = mapb.get(c) {
    //         if mapa[c] != *v {
    //             panic!("error")
    //         }
    //     } else {
    //         panic!("error")
    //     }
    // }
    // println!("ok")
}

fn group_anagram() {
    let v = [
        "asdasda",
        "assdada",
        "ghrtdwwe122d",
        "ghrtd33wwed",
        "ghr221tdwwed",
    ];

    let mut map = HashMap::new();
    for str in v {
        let mut chars = vec![];
        for c in str.chars() {
            chars.push(c);
        }
        chars.sort();
        let key: String = chars.into_iter().collect();
        let vs = map.entry(key).or_insert(vec![]);
        vs.push(str);
    }
    println!("{:?}", map);
}
