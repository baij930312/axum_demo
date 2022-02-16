#[derive(Debug, Clone)]
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Node { next: None, val }
    }
}

fn main() {
    let node1 = Node::new(4);
    let mut node2 = Node::new(3);
    let mut node3 = Node::new(2);
    let mut node4 = Node::new(1);

    let mut node5 = Node::new(7);
    let mut node6 = Node::new(5);
    let mut node7 = Node::new(1);
    node2.next = Some(Box::new(node1));
    node3.next = Some(Box::new(node2));
    node4.next = Some(Box::new(node3));
    node5.next = Some(Box::new(node4));
    node6.next = Some(Box::new(node5));
    node7.next = Some(Box::new(node6));
    println!("{:?}", remove_from_end_2(Some(Box::new(node7)), 2));
}

fn remove_from_end(head: Option<Box<Node>>, n: i32) -> Option<Box<Node>> {
    let mut len = 0;
    let mut fore = Some(Box::new(Node {
        val: -1,
        next: head,
    }));
    let mut curr = &mut fore;
    while curr.is_some() && curr.as_ref().unwrap().next.is_some() {
        len += 1;
        curr = &mut curr.as_mut().unwrap().next;
    }
    let index = len - n;
    curr = &mut fore;
    for _ in 0..index {
        curr = &mut curr.as_mut().unwrap().next;
    }
    let next = curr.as_mut().unwrap().next.as_mut().unwrap().next.take();
    curr.as_mut().unwrap().next = next;
    println!("{}", len);
    fore.unwrap().next
}

fn remove_from_end_2(head: Option<Box<Node>>, n: i32) -> Option<Box<Node>> {
    let mut fore = Some(Box::new(Node {
        val: -1,
        next: head,
    }));
    let mut slow_p = &mut fore;
    let mut fast_p = &mut slow_p.clone();
    for _ in 0..n {
        fast_p = &mut fast_p.as_mut().unwrap().next;
    }
  
    while fast_p.is_some() && fast_p.as_ref().unwrap().next.is_some()  {
        fast_p = &mut fast_p.as_mut().unwrap().next;
        slow_p = &mut slow_p.as_mut().unwrap().next;
    }
    // println!("{:?}", slow_p);
    let next = slow_p.as_mut().unwrap().next.as_mut().unwrap().next.take();
    slow_p.as_mut().unwrap().next = next;
  
    fore.unwrap().next
}
