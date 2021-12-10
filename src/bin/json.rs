use serde::{Deserialize, Serialize};
use serde_json as sj;

#[derive(Debug, Deserialize, Serialize)]
struct User {
    name: String,
    age: i32,
}

fn main() {
    let json = r#"
    {"name" : "asdada" , "age" : 13}
    "#;

    let user: User = sj::from_str(json).unwrap();
    println!("{:?}", user);

    let u = User {
        name: "asdad".into(),
        age: 111,
    };

    let s = sj::to_string(&u).unwrap();
    println!("{}",s);
}
