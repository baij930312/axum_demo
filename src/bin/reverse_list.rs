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
    let node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let mut node3 = Node::new(3);
    let mut node4 = Node::new(4);
    let mut node5 = Node::new(5);
    let mut node6 = Node::new(6);
    let mut node7 = Node::new(7);
    node2.next = Some(Box::new(node1));
    node3.next = Some(Box::new(node2));
    node4.next = Some(Box::new(node3));
    node5.next = Some(Box::new(node4));
    node6.next = Some(Box::new(node5));
    node7.next = Some(Box::new(node6));
    println!("{:?}", middle(Some(Box::new(node7))));
}

fn middle(head: Option<Box<Node>>) -> Option<Box<Node>> {
    let mut p1 = &head;
    let mut p2 = &head;

    while p1.is_some() && p1.as_ref().unwrap().next.is_some() {
        p2 = &p2.as_ref().unwrap().next;
        p1 = &p1.as_ref().unwrap().next.as_ref().unwrap().next;
    }
    p2.clone()
    // while let Some(fast_node) = p2 {
    //     if let Some(n1) = &fast_node.next {
    //         p2 = &n1.next;
    //     } else {
    //         break;
    //     }
    //     if let Some(n) = p1 {
    //         p1 = &n.next;
    //     }
    // }
    // p1.clone()
}

fn reverse(head: Option<Box<Node>>) -> Option<Box<Node>> {
    let mut prev = None;
    let mut curr = head;
    while let Some(mut curr_node) = curr {
        let next_node = curr_node.next;
        curr_node.next = prev;
        prev = Some(curr_node);
        curr = next_node;
    }
    prev
}
