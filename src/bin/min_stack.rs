use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::Arc;


fn move_0_to_end(s: &[i32]) {
    let mut v = vec![0i32; s.len()];
    let mut index = 0;
    for i in s {
        if *i != 0 {
            v[index] = *i;
            index += 1;
        }
    }
    println!("{:?}", v);
}


struct MinStack {
    datas: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            datas: vec![],
            min_stack: vec![],
        }
    }

    fn push(&mut self, data: i32) {
        self.datas.push(data);
        if self.min_stack.is_empty() || data <= *self.min_stack.last().unwrap() {
            self.min_stack.push(data);
        }
    }

    fn pop(&mut self) -> Option<i32> {
        if let Some(data) = self.datas.pop() {
            if data == *self.min_stack.last().unwrap(){
                self.min_stack.pop();
            }
        }
        None
    }

    fn top(&self) -> Option<i32> {
        if let Some(data) = self.datas.last() {
            return Some(*data);
        }
        None
    }

    fn get_min(&self) -> Option<i32> {
        if let Some(data) = self.min_stack.last() {
            return Some(*data);
        }
        None
    }
}

// #[tokio::main]
// async fn main()->io::Result<()>{
//     let listener = TcpListener::bind("127.0.0.1::6142").await.unwrap();

//     let (  socket,_) = listener.accept().await.unwrap();
// io::split 内部使用了arc和mutex使得读写handle可以跨线程
// socket.split 只是返回两个内部的引用，由于引用已经被返回使用所有权转移所以两个handle只能在同一线程 这种方式是0开销的
//     let (mut rt,mut wt) = io::split(socket);
//        tokio::spawn(async move{
//             wt.write_all(b"hello").await?;
//             wt.write_all(b"world").await?;
//             Ok::<_, io::Error>(())

//        });
//        let mut buf = vec![0,128];

//     loop  {
//     let n = rt.read(&mut buf).await?;
//     if n == 0 {
//         break;
//     }

//     println!("{:?}",&buf[..n] );
//     }
//     Ok(())
// }

// struct ListNode{
//     val : i32,
//     node : option<Box<ListNode>>
// }

struct Student {
    name: &'static str,
    score: u32,
}

struct Color(i32, i32, i32);

#[derive(Debug)]
enum ColorNoParam {
    Red(String),
    Blue(String),
    Yellow(String),
}

fn distinct(mut sorted_arr: Vec<i32>) -> i32 {
    let len = sorted_arr.len();
    if len == 0 {
        return 0;
    }
    let mut j = 0;
    for i in 1..len {
        if sorted_arr[i] != sorted_arr[j] {
            if i - j > 1 {
                sorted_arr[j + 1] = sorted_arr[i];
            }
            j += 1;
        }
    }
    println!("{:?}", sorted_arr);
    (j + 1) as i32
}

fn plus_one(mut num_vec: Vec<i32>) {
    let mut up = false;
    let len = num_vec.len();
    for (i, v) in num_vec.iter_mut().rev().enumerate() {
        println!("{:?}", i);
        if i == 0 {
            *v += 1;
            if *v >= 10 {
                *v = 0;
                up = true;
            }
        } else if i != len - 1 {
            if up {
                *v += 1;
                if *v >= 10 {
                    *v = 0;
                    up = true;
                }
            } else {
                break;
            };
        }
    }
    if up {
        let v = num_vec.first().unwrap();
        let v = v + 1;
        if v >= 10 {
            num_vec[0] = 0;
            num_vec.insert(0, 1);
        }
    }
    println!("{:?}", num_vec);
}

fn main() {
    let mut stack = MinStack::new();
    stack.push(-2);
    stack.push(0);
    stack.push(-3);
    println!("{:?}", stack.get_min());
    stack.pop();
    println!("{:?}", stack.top());
    println!("{:?}", stack.get_min());



    let v = [1, 34, 543, 6577, 0, 23, 0, 8];
    move_0_to_end(&v);
    let v = RefCell::new(vec![1, 2, 3]);
    let mut bv = v.borrow_mut();
    println!("{}", bv.remove(1));

    let aaa = Arc::new(5);
    let bbb = aaa.clone();

    println!("{}", *aaa);
    println!("{}", Arc::strong_count(&aaa));

    let a = Box::new(3);
    println!("{}", a);
    println!("{}", *a);
    let mut s = String::from("asdasdad你好ad");
    let mut s2 = String::from("676567");

    for i in s.bytes() {
        println!("{}", i);
    }

    for i in s.chars() {
        println!("{}", i);
    }

    s2.truncate(3);
    s = s + &s2;
    s.push_str("123131");
    s.insert_str(2, "5555");
    println!("{}", s);
    let key = "asdsadsad";
    let mut map = HashMap::new();
    map.insert(key, 12);
    map.insert("bbbbb", 12);
    map.insert("ccccc", 12);

    println!("{}", key);
    for (i, v) in map.iter_mut() {
        println!("{}", &i);
        *v += 2;
    }

    println!("{:?}", map);

    let mut v: VecDeque<u32> = VecDeque::new();

    v.push_back(3);
    v.push_back(4);
    v.push_front(1);
    v.push_front(2);
    v[1] = 2;
    println!("{:?}", v);
}
