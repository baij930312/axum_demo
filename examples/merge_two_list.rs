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

    let node5 = Node::new(7);
    let mut node6 = Node::new(5);
    let mut node7 = Node::new(1);
    node2.next = Some(Box::new(node1));
    node3.next = Some(Box::new(node2));
    node4.next = Some(Box::new(node3));

    node6.next = Some(Box::new(node5));
    node7.next = Some(Box::new(node6));
    println!(
        "{:?}",
        merge(Some(Box::new(node4)), Some(Box::new(node7)))
    );
}

fn merge(node1: Option<Box<Node>>, node2: Option<Box<Node>>) -> Option<Box<Node>> {
    match (node1, node2) {
        (None, None) => None,
        (None, Some(n2)) => Some(n2),
        (Some(n1), None) => Some(n1),
        (Some(mut n1), Some(mut n2)) => {
            if n1.val < n2.val {
                n1.next = merge(n1.next, Some(n2)); 
                Some(n1)
            } else {
                n2.next = merge(Some(n1), n2.next);
                Some(n2)
            }
        }
    }
    // let mut p_1 = &node1;
    // let mut p_2 = &node2;
    // let mut head = None;
    // let mut last = None;
    // while p_1.as_ref().is_some() || p_2.as_ref().is_some() {
    //     let mut curr = None;
    //     if p_1.as_ref().is_some() && p_2.as_ref().is_some() {
    //         let node1 = p_1.as_ref().unwrap();
    //         let node2 = p_2.as_ref().unwrap();
    //         if node1.val < node2.val {
    //             curr = Some(node2.clone());
    //             p_2 = &node2.next;
    //         } else {
    //             curr = Some(node1.clone());
    //             p_1 = &node1.next;
    //         }
    //     } else {
    //         if p_1.as_ref().is_some() {
    //             let node1 = p_1.as_ref().unwrap();
    //             curr = Some(node1.clone());
    //             p_1 = &node1.next;
    //         }
    //         if p_2.as_ref().is_some() {
    //             let node2 = p_2.as_ref().unwrap();
    //             curr = Some(node2.clone());
    //             p_2 = &node2.next;
    //         }
    //     }
    //     if head.as_ref().is_none() {
    //         head = curr.clone();
    //         last = curr;
    //     } else {
    //         // let mut a = last.unwrap();
    //         // last.next = Some(curr.unwrap().clone());
    //     }
    // }
    // Some(head.unwrap().clone())
}

fn merge_k_lists(lists: Vec<Option<Box<Node>>>) -> Option<Box<Node>> {
    // let mut val = 9999;
    let len: usize = lists.len();
    let mut ans: Option<Box<Node>> = None;
    let mut ans_p = &mut ans;
    let mut ps: Vec<&Option<Box<Node>>> = vec![];
    for list_node in lists.iter() {
        ps.push(&list_node);
    }
    let mut count = 0;
    let mut min_val = 9999;
    let mut min_index = 0;
    loop {
        for i in 0..len {
            if let Some(node) = ps[i] {
                // println!("node_val: {}, min_val: {}", node.val, min_val);
                if min_val > node.val {
                    min_val = node.val;
                    min_index = i;
                }
                count += 1;
            }
            // println!("{}", count);
        }
        if count == 0 {
            break;
        } else {
            // println!("{}", min_val);
            *ans_p = Some(Box::new(Node::new(min_val)));
            if let Some(node) = ans_p {
                ans_p = &mut node.next;
            }
            if let Some(node) = ps[min_index] {
                ps[min_index] = &node.next;
            }
            count = 0;
            min_val = 9999;
        }
    }
    ans
}
