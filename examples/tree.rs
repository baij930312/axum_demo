use std::{cell::RefCell, collections::VecDeque, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            right: None,
            left: None,
        }
    }
}
///
///         7
///      /    \
///   3           6
/// /   \       /   \
/// 2   1       5   4
///
fn main() {
    let mut n1 = TreeNode::new(1);
    let mut n2 = TreeNode::new(2);
    let mut n3 = TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(n2))),
        right: Some(Rc::new(RefCell::new(n1))),
    };
    let mut n9 = TreeNode::new(9);
    let mut n4 = TreeNode {
        val: 4,
        left: Some(Rc::new(RefCell::new(n9))),
        right: None,
    };
    let mut n5 = TreeNode::new(5);
    let mut n6 = TreeNode {
        val: 6,
        left: Some(Rc::new(RefCell::new(n5))),
        right: Some(Rc::new(RefCell::new(n4))),
    };
    let mut n7 = TreeNode {
        val: 7,
        left: Some(Rc::new(RefCell::new(n3))),
        right: Some(Rc::new(RefCell::new(n6))),
    };

    // let mut v = vec![];
    // dfs(Some(Rc::new(RefCell::new(n7.clone()))), &mut v);
    // for i in v.iter() {
    //     if let Some(node) = i {
    //         println!("{}", node.borrow().val);
    //     }
    // }
    // println!("{:?}", v);
    println!("{:?}", find_max_depth(Some(Rc::new(RefCell::new(n7)))));
}

fn preorder(root: Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
    match root {
        Some(node) => {
            v.push(node.borrow().val);
            preorder(node.borrow().left.clone(), v);
            preorder(node.borrow().right.clone(), v);
        }
        None => return,
    }
}

fn postorder(root: Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
    match root {
        Some(node) => {
            postorder(node.borrow().left.clone(), v);
            postorder(node.borrow().right.clone(), v);
            v.push(node.borrow().val);
        }
        None => return,
    }
}

fn inorder(root: Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<i32>) {
    match root {
        Some(node) => {
            inorder(node.borrow().left.clone(), v);
            v.push(node.borrow().val);
            inorder(node.borrow().right.clone(), v);
        }
        None => return,
    }
}

fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut v: Vec<i32> = vec![];
    let mut curr = root;
    let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];
    while curr.is_some() || !stack.is_empty() {
        while let Some(node) = curr {
            v.push(node.borrow().val);
            stack.push(node.clone());
            curr = node.borrow().left.clone();
        }
        curr = stack.pop();
        if let Some(node) = curr {
            curr = node.borrow().right.clone();
        }
    }
    v
}

fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut v: Vec<i32> = vec![];
    let mut curr = root;
    let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];
    while curr.is_some() || !stack.is_empty() {
        while let Some(node) = curr {
            stack.push(node.clone());
            curr = node.borrow().left.clone();
        }
        curr = stack.pop();
        if let Some(node) = curr {
            v.push(node.borrow().val);
            curr = node.borrow().right.clone();
        }
    }
    v
}

fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut v: Vec<i32> = vec![];
    let mut stack1: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
    let mut stack2: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
    stack1.push(root);
    while let Some(Some(node)) = stack1.pop() {
        if node.borrow().left.is_some() {
            stack1.push(node.borrow().left.clone());
        }
        if node.borrow().right.is_some() {
            stack1.push(node.borrow().right.clone());
        }
        stack2.push(Some(node));
    }
    while let Some(Some(node)) = stack2.pop() {
        v.push(node.borrow().val);
    }
    v
}

fn levelorder(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut queue = VecDeque::new();
    let mut v = vec![];
    queue.push_front(root);
    while let Some(Some(node)) = queue.pop_back() {
        if node.borrow().left.is_some() {
            queue.push_front(node.borrow().left.clone());
        }
        if node.borrow().right.is_some() {
            queue.push_front(node.borrow().right.clone());
        }
        v.push(node.borrow().val);
    }
    v
}

fn dfs(node: Option<Rc<RefCell<TreeNode>>>, v: &mut Vec<Option<Rc<RefCell<TreeNode>>>>) {
    if v.contains(&node) {
        return;
    }
    v.push(node.clone());
    let mut childs = vec![];
    if let Some(node) = node {
        if node.borrow().left.is_some() {
            childs.push(node.borrow().left.clone());
        }
        if node.borrow().right.is_some() {
            childs.push(node.borrow().right.clone());
        }
    }
    for node in childs {
        if v.contains(&node) {
            continue;
        }
        dfs(node, v);
    }
}

fn dfs_traversal(node: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
    let mut v: Vec<i32> = vec![];
    stack.push(node);
    while let Some(Some(node)) = stack.pop() {
        v.push(node.borrow().val);
        if node.borrow().right.is_some() {
            stack.push(node.borrow().right.clone());
        }
        if node.borrow().left.is_some() {
            stack.push(node.borrow().left.clone());
        }
    }
    v
}

fn find_max_depth(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match node {
        Some(node) => {
            let left = find_max_depth(node.borrow().left.clone());
            let right = find_max_depth(node.borrow().right.clone());
            1 + left.max(right)
        }
        None => 0,
    }
}

fn find_max_depth_bfs(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut queue = VecDeque::new();
    queue.push_front(node);
    let mut depth = 0;
    while !queue.is_empty() {
        let size = queue.len();
        println!("size: {}", size);
        depth += 1;
        for _ in 0..size {
            if let Some(Some(node)) = queue.pop_back() {
                if node.borrow().left.is_some() {
                    queue.push_front(node.borrow().left.clone());
                }
                if node.borrow().right.is_some() {
                    queue.push_front(node.borrow().right.clone());
                }
            }
        }
    }
    depth
}
